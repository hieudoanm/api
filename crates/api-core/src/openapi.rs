use serde_json::{Map, Value};

use crate::{
    ApiAuth, ApiBody, ApiCollection, ApiFolder, ApiParameter, ApiRequest, BodyKind, ClientFormat,
    ConvertOptions, ParameterLocation, Result,
};

const HTTP_METHODS: [&str; 8] = [
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];

pub fn parse_openapi(source: &str) -> Result<ApiCollection> {
    let trimmed = source.trim_start();
    let value: Value = if trimmed.starts_with('{') || trimmed.starts_with('[') {
        serde_json::from_str(source).map_err(|e| crate::ApiConvertError::Parse(e.to_string()))?
    } else {
        serde_yml::from_str(source).map_err(|e| crate::ApiConvertError::Parse(e.to_string()))?
    };

    let openapi = value
        .get("openapi")
        .and_then(Value::as_str)
        .ok_or_else(|| crate::ApiConvertError::Invalid("missing openapi version".into()))?;
    if !openapi.starts_with("3.") {
        return Err(crate::ApiConvertError::UnsupportedVersion(
            openapi.to_string(),
        ));
    }

    normalize(&value)
}

pub fn convert_openapi(
    source: &str,
    format: ClientFormat,
    options: &ConvertOptions,
) -> Result<Vec<crate::ConversionFile>> {
    let collection = parse_openapi(source)?;
    crate::convert_collection(&collection, format, options)
}

pub fn convert_openapi_all(
    source: &str,
    options: &ConvertOptions,
) -> Result<Vec<crate::ConversionFile>> {
    let collection = parse_openapi(source)?;
    let mut files = Vec::new();
    for format in ClientFormat::ALL {
        files.extend(crate::convert_collection(&collection, format, options)?);
    }
    Ok(files)
}

fn normalize(root: &Value) -> Result<ApiCollection> {
    let info = root.get("info").cloned().unwrap_or(Value::Null);
    let name = info
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or("OpenAPI Collection")
        .to_string();
    let description = info
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let version = info
        .get("version")
        .and_then(Value::as_str)
        .unwrap_or("1.0.0")
        .to_string();
    let servers = root
        .get("servers")
        .and_then(Value::as_array)
        .map(|servers| {
            servers
                .iter()
                .filter_map(|server| server.get("url").and_then(Value::as_str))
                .map(|url| resolve_server_variables(url, server_variables(root)))
                .map(|url| url.trim_end_matches('/').to_string())
                .filter(|url| !url.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let mut folders: Vec<ApiFolder> = Vec::new();
    let paths = root
        .get("paths")
        .and_then(Value::as_object)
        .ok_or_else(|| crate::ApiConvertError::Invalid("missing paths object".into()))?;

    for (path, path_item_value) in paths {
        let path_item = resolve(path_item_value, root, 0);
        if path_item.get("$ref").is_some() {
            continue;
        }
        let path_parameters = path_item
            .get("parameters")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        for method in HTTP_METHODS {
            let Some(operation_value) = path_item.get(method) else {
                continue;
            };
            let operation = resolve(operation_value, root, 0);
            let server = first_server_url(&operation, root)
                .or_else(|| first_server_url(&path_item, root))
                .or_else(|| servers.first().cloned());
            let request =
                normalize_operation(root, path, method, &operation, &path_parameters, server);
            let folder_name = request
                .tags
                .first()
                .cloned()
                .unwrap_or_else(|| "Default".to_string());
            if let Some(folder) = folders.iter_mut().find(|f| f.name == folder_name) {
                folder.requests.push(request);
            } else {
                folders.push(ApiFolder {
                    name: folder_name,
                    requests: vec![request],
                });
            }
        }
    }

    Ok(ApiCollection {
        name,
        description,
        version,
        servers,
        folders,
    })
}

fn normalize_operation(
    root: &Value,
    path: &str,
    method: &str,
    operation: &Value,
    path_parameters: &[Value],
    server: Option<String>,
) -> ApiRequest {
    let mut parameters = Vec::new();
    for value in path_parameters.iter().chain(
        operation
            .get("parameters")
            .and_then(Value::as_array)
            .into_iter()
            .flatten(),
    ) {
        let resolved = resolve(value, root, 0);
        if let Some(parameter) = parameter_from_value(&resolved, root) {
            if parameters.iter().any(|p: &ApiParameter| {
                p.name == parameter.name && p.location == parameter.location
            }) {
                continue;
            }
            parameters.push(parameter);
        }
    }

    let mut path_params = Vec::new();
    let mut query_params = Vec::new();
    let mut headers = Vec::new();
    for parameter in parameters {
        match parameter.location {
            ParameterLocation::Path => path_params.push(parameter),
            ParameterLocation::Query => query_params.push(parameter),
            ParameterLocation::Header => headers.push(parameter),
            ParameterLocation::Cookie => {}
        }
    }

    let body = operation
        .get("requestBody")
        .map(|value| resolve(value, root, 0))
        .and_then(|value| body_from_value(&value, root));

    let tags = operation
        .get("tags")
        .and_then(Value::as_array)
        .map(|tags| {
            tags.iter()
                .filter_map(Value::as_str)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let method_upper = method.to_ascii_uppercase();
    let name = operation
        .get("summary")
        .or_else(|| operation.get("operationId"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    let name = if name.is_empty() {
        format!("{method_upper} {path}")
    } else {
        name
    };

    ApiRequest {
        id: format!("{}-{}", method, path.trim_start_matches('/')),
        name,
        method: method_upper,
        path: path.to_string(),
        server,
        description: operation
            .get("description")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        tags,
        path_params,
        query_params,
        headers,
        body,
        auth: auth_from_operation(root, operation),
    }
}

fn parameter_from_value(value: &Value, root: &Value) -> Option<ApiParameter> {
    let name = value.get("name").and_then(Value::as_str)?.to_string();
    let location = match value.get("in").and_then(Value::as_str)? {
        "query" => ParameterLocation::Query,
        "path" => ParameterLocation::Path,
        "header" => ParameterLocation::Header,
        "cookie" => ParameterLocation::Cookie,
        _ => return None,
    };
    let required = value
        .get("required")
        .and_then(Value::as_bool)
        .unwrap_or(location == ParameterLocation::Path);
    let description = value
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let example = parameter_example(value, root);
    let value = match example {
        Some(value) => value_to_string(&value),
        None if location == ParameterLocation::Path => name.clone(),
        None => String::new(),
    };
    Some(ApiParameter {
        name,
        location,
        required,
        description,
        value,
    })
}

fn parameter_example(parameter: &Value, root: &Value) -> Option<Value> {
    if let Some(example) = parameter.get("example") {
        return Some(example.clone());
    }
    if let Some(examples) = parameter.get("examples").and_then(Value::as_object) {
        for example in examples.values() {
            let resolved = resolve(example, root, 0);
            if let Some(value) = resolved.get("value") {
                return Some(value.clone());
            }
        }
    }
    parameter
        .get("schema")
        .map(|schema| schema_example(schema, root, 0))
        .filter(|value| !value.is_null())
}

fn body_from_value(request_body: &Value, root: &Value) -> Option<ApiBody> {
    let content = request_body.get("content").and_then(Value::as_object)?;
    if content.is_empty() {
        return None;
    }
    let (content_type, media) = choose_media_type(content);
    let kind = body_kind(content_type);
    let (raw, fields) = match kind {
        BodyKind::Json => (Some(json_body_raw(media, root)), Vec::new()),
        BodyKind::UrlEncoded | BodyKind::FormData => (None, form_fields(media, root)),
        BodyKind::Text | BodyKind::Other => (Some(text_body_raw(media, root)), Vec::new()),
    };
    Some(ApiBody {
        content_type: content_type.to_string(),
        kind,
        raw,
        fields,
    })
}

fn choose_media_type(content: &Map<String, Value>) -> (&str, &Value) {
    let preferred = [
        "application/json",
        "application/problem+json",
        "application/x-www-form-urlencoded",
        "multipart/form-data",
        "text/plain",
    ];
    for key in preferred {
        if let Some(value) = content.get(key) {
            return (key, value);
        }
    }
    for (key, value) in content {
        if key.contains("json") || key.ends_with("+json") {
            return (key, value);
        }
    }
    content.iter().next().map(|(k, v)| (k.as_str(), v)).unwrap()
}

fn body_kind(content_type: &str) -> BodyKind {
    let lower = content_type.to_ascii_lowercase();
    if lower.contains("json") {
        BodyKind::Json
    } else if lower == "application/x-www-form-urlencoded" {
        BodyKind::UrlEncoded
    } else if lower == "multipart/form-data" {
        BodyKind::FormData
    } else if lower.starts_with("text/") {
        BodyKind::Text
    } else {
        BodyKind::Other
    }
}

fn media_example(media: &Value, root: &Value) -> Option<Value> {
    if let Some(example) = media.get("example") {
        return Some(example.clone());
    }
    if let Some(examples) = media.get("examples").and_then(Value::as_object) {
        for example in examples.values() {
            let resolved = resolve(example, root, 0);
            if let Some(value) = resolved.get("value") {
                return Some(value.clone());
            }
        }
    }
    media
        .get("schema")
        .map(|schema| schema_example(schema, root, 0))
        .filter(|value| !value.is_null())
}

fn json_body_raw(media: &Value, root: &Value) -> String {
    match media_example(media, root) {
        Some(Value::String(text)) => {
            if serde_json::from_str::<Value>(&text).is_ok() {
                serde_json::to_string_pretty(&Value::String(text.clone())).unwrap_or(text)
            } else {
                text
            }
        }
        Some(value) if value.is_null() => "{}".to_string(),
        Some(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
        None => "{}".to_string(),
    }
}

fn text_body_raw(media: &Value, root: &Value) -> String {
    match media_example(media, root) {
        Some(Value::String(text)) => text,
        Some(value) => serde_json::to_string_pretty(&value).unwrap_or_default(),
        None => String::new(),
    }
}

fn form_fields(media: &Value, root: &Value) -> Vec<ApiParameter> {
    let Some(schema) = media.get("schema") else {
        return Vec::new();
    };
    let schema = resolve(schema, root, 0);
    let required = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut fields = Vec::new();
    for (name, property_value) in properties {
        let property = resolve(property_value, root, 0);
        let value = property
            .get("example")
            .or_else(|| property.get("default"))
            .or_else(|| {
                property
                    .get("enum")
                    .and_then(Value::as_array)
                    .and_then(|v| v.first())
            })
            .cloned()
            .map(|v| value_to_string(&v))
            .unwrap_or_else(|| {
                if property.get("format").and_then(Value::as_str) == Some("binary") {
                    "<file>".to_string()
                } else {
                    String::new()
                }
            });
        fields.push(ApiParameter {
            name: name.clone(),
            location: ParameterLocation::Query,
            required: required.iter().any(|item| item == name),
            description: property
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            value,
        });
    }
    fields
}

fn auth_from_operation(root: &Value, operation: &Value) -> Vec<ApiAuth> {
    let security = if operation.get("security").is_some() {
        operation.get("security")
    } else {
        root.get("security")
    };
    let Some(requirements) = security.and_then(Value::as_array) else {
        return Vec::new();
    };
    let Some(requirement) = requirements.iter().find_map(Value::as_object) else {
        return Vec::new();
    };
    let schemes = root
        .get("components")
        .and_then(|c| c.get("securitySchemes"))
        .and_then(Value::as_object);
    let mut auth = Vec::new();
    for (name, scopes) in requirement {
        let Some(scheme) = schemes.and_then(|s| s.get(name)) else {
            continue;
        };
        let scheme = resolve(scheme, root, 0);
        let _ = scopes;
        if let Some(item) = auth_from_scheme(name, &scheme) {
            auth.push(item);
        }
    }
    auth
}

fn auth_from_scheme(name: &str, scheme: &Value) -> Option<ApiAuth> {
    let kind = scheme.get("type").and_then(Value::as_str).unwrap_or("");
    match kind {
        "http" => {
            let http_scheme = scheme
                .get("scheme")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_ascii_lowercase();
            match http_scheme.as_str() {
                "bearer" => Some(ApiAuth::Bearer {
                    token: "{{accessToken}}".to_string(),
                }),
                "basic" => Some(ApiAuth::Basic {
                    username: "{{username}}".to_string(),
                    password: "{{password}}".to_string(),
                }),
                _ => Some(ApiAuth::Other {
                    kind: "http".to_string(),
                    name: http_scheme,
                }),
            }
        }
        "apiKey" => {
            let key_name = scheme
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("apiKey")
                .to_string();
            let location = match scheme.get("in").and_then(Value::as_str) {
                Some("query") => ParameterLocation::Query,
                Some("cookie") => ParameterLocation::Cookie,
                _ => ParameterLocation::Header,
            };
            Some(ApiAuth::ApiKey {
                name: key_name,
                location,
                value: "{{apiKey}}".to_string(),
            })
        }
        "oauth2" | "openIdConnect" => Some(ApiAuth::OAuth2 {
            token: "{{accessToken}}".to_string(),
        }),
        _ => Some(ApiAuth::Other {
            kind: kind.to_string(),
            name: name.to_string(),
        }),
    }
}

fn first_server_url(value: &Value, root: &Value) -> Option<String> {
    value
        .get("servers")
        .and_then(Value::as_array)
        .and_then(|servers| servers.first())
        .and_then(|server| server.get("url").and_then(Value::as_str))
        .map(|url| resolve_server_variables(url, server_variables(root)))
        .map(|url| url.trim_end_matches('/').to_string())
        .filter(|url| !url.is_empty())
}

fn server_variables(root: &Value) -> Map<String, Value> {
    let mut variables = Map::new();
    if let Some(servers) = root.get("servers").and_then(Value::as_array) {
        for server in servers {
            if let Some(server_vars) = server.get("variables").and_then(Value::as_object) {
                for (name, variable) in server_vars {
                    if let Some(default) = variable.get("default").and_then(Value::as_str) {
                        variables.insert(name.clone(), Value::String(default.to_string()));
                    }
                }
            }
        }
    }
    variables
}

fn resolve_server_variables(url: &str, variables: Map<String, Value>) -> String {
    let mut result = url.to_string();
    for (name, value) in variables {
        if let Some(default) = value.as_str() {
            result = result.replace(&format!("{{{name}}}"), default);
        }
    }
    result
}

fn resolve(value: &Value, root: &Value, depth: usize) -> Value {
    if depth > 16 {
        return value.clone();
    }
    let Some(reference) = value.get("$ref").and_then(Value::as_str) else {
        return value.clone();
    };
    let Some(target) = resolve_pointer(root, reference) else {
        return value.clone();
    };
    let mut resolved = resolve(target, root, depth + 1);
    if let (Some(source), Some(target)) = (value.as_object(), resolved.as_object_mut()) {
        for (key, sibling) in source {
            if key != "$ref" && !target.contains_key(key) {
                target.insert(key.clone(), sibling.clone());
            }
        }
    }
    resolved
}

fn resolve_pointer<'a>(root: &'a Value, reference: &str) -> Option<&'a Value> {
    if reference == "#" {
        return Some(root);
    }
    let pointer = reference.strip_prefix("#/")?;
    let mut current = root;
    for part in pointer.split('/') {
        let key = part.replace("~1", "/").replace("~0", "~");
        current = current.get(key.as_str())?;
    }
    Some(current)
}

fn schema_example(schema: &Value, root: &Value, depth: usize) -> Value {
    if depth > 8 {
        return Value::Null;
    }
    let schema = resolve(schema, root, depth);
    if let Some(example) = schema.get("example") {
        return example.clone();
    }
    if let Some(examples) = schema.get("examples").and_then(Value::as_array) {
        if let Some(first) = examples.first() {
            return first.clone();
        }
    }
    if let Some(default) = schema.get("default") {
        return default.clone();
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        if let Some(first) = values.first() {
            return first.clone();
        }
    }
    for key in ["allOf", "oneOf", "anyOf"] {
        if let Some(variants) = schema.get(key).and_then(Value::as_array) {
            if key == "allOf" {
                let mut merged = Map::new();
                for variant in variants {
                    let example = schema_example(variant, root, depth + 1);
                    if let Value::Object(object) = example {
                        for (name, value) in object {
                            merged.entry(name).or_insert(value);
                        }
                    }
                }
                if !merged.is_empty() {
                    return Value::Object(merged);
                }
            } else if let Some(first) = variants.first() {
                return schema_example(first, root, depth + 1);
            }
        }
    }

    let schema_type = schema
        .get("type")
        .and_then(|value| {
            value.as_str().map(ToString::to_string).or_else(|| {
                value
                    .as_array()?
                    .iter()
                    .filter_map(Value::as_str)
                    .find(|kind| *kind != "null")
                    .map(ToString::to_string)
            })
        })
        .unwrap_or_default();

    match schema_type.as_str() {
        "string" => Value::String(match schema.get("format").and_then(Value::as_str) {
            Some("date") => "2026-01-01".to_string(),
            Some("date-time") => "2026-01-01T00:00:00Z".to_string(),
            Some("uuid") => "00000000-0000-0000-0000-000000000000".to_string(),
            Some("binary") | Some("byte") => "<file>".to_string(),
            _ => "string".to_string(),
        }),
        "integer" | "number" => Value::from(0),
        "boolean" => Value::Bool(true),
        "array" => {
            let item = schema
                .get("items")
                .map(|items| schema_example(items, root, depth + 1))
                .unwrap_or(Value::Null);
            if item.is_null() {
                Value::Array(Vec::new())
            } else {
                Value::Array(vec![item])
            }
        }
        "object" | "" => {
            let mut object = Map::new();
            if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
                for (name, property) in properties {
                    object.insert(name.clone(), schema_example(property, root, depth + 1));
                }
            }
            Value::Object(object)
        }
        "null" => Value::Null,
        _ => Value::Null,
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Null => String::new(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::Array(_) | Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}
