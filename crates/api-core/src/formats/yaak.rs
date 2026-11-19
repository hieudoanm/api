use serde_json::{json, Value};

use crate::formats::util::{
    auth_for_request, base_url, client_path, collection_name, slugify, stable_id,
};
use crate::{
    ApiAuth, ApiCollection, BodyKind, ClientFormat, ConversionFile, ConvertOptions,
    ParameterLocation, Result,
};

const TIMESTAMP: &str = "2024-01-01T00:00:00";

pub fn convert(
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    let name = collection_name(collection, options);
    let base = effective_base(collection, options);
    let workspace_id = stable_id("wk", &format!("yaak:{name}"));

    let mut files = vec![
        yaml_file(
            ClientFormat::Yaak,
            "workspace.yaml",
            &json!({
                "model": "workspace",
                "id": workspace_id,
                "createdAt": TIMESTAMP,
                "updatedAt": TIMESTAMP,
                "name": name,
                "description": collection.description,
                "authentication": {},
                "authenticationType": Value::Null,
                "headers": [],
            }),
        ),
        yaml_file(
            ClientFormat::Yaak,
            "environment-development.yaml",
            &json!({
                "model": "environment",
                "id": stable_id("ev", &format!("yaak:{name}:env")),
                "workspaceId": workspace_id,
                "createdAt": TIMESTAMP,
                "updatedAt": TIMESTAMP,
                "name": "development",
                "parentModel": "workspace",
                "parentId": Value::Null,
                "public": true,
                "sortPriority": 0,
                "variables": [
                    {"name": "base_url", "value": base, "enabled": true, "id": Value::Null},
                    {"name": "accessToken", "value": "{{accessToken}}", "enabled": true, "id": Value::Null},
                    {"name": "apiKey", "value": "{{apiKey}}", "enabled": true, "id": Value::Null},
                    {"name": "username", "value": "{{username}}", "enabled": true, "id": Value::Null},
                    {"name": "password", "value": "{{password}}", "enabled": true, "id": Value::Null},
                ],
            }),
        ),
    ];

    let mut sort_priority = 1.0f64;
    for folder in &collection.folders {
        let folder_id = stable_id("fl", &format!("yaak:{name}:{}", folder.name));
        files.push(yaml_file(
            ClientFormat::Yaak,
            &format!("folder-{}.yaml", slugify(&folder.name)),
            &json!({
                "model": "folder",
                "id": folder_id,
                "workspaceId": workspace_id,
                "folderId": Value::Null,
                "createdAt": TIMESTAMP,
                "updatedAt": TIMESTAMP,
                "name": folder.name,
                "description": "",
                "sortPriority": sort_priority,
            }),
        ));
        sort_priority += 1.0;
        for request in &folder.requests {
            files.push(yaml_file(
                ClientFormat::Yaak,
                &format!(
                    "request-{}-{}.yaml",
                    slugify(&folder.name),
                    slugify(&format!("{}-{}", request.method, request.name))
                ),
                &request_yaml(request, &workspace_id, &folder_id, sort_priority),
            ));
            sort_priority += 1.0;
        }
    }
    Ok(files)
}

fn yaml_file(format: ClientFormat, path: &str, value: &Value) -> ConversionFile {
    ConversionFile {
        format,
        path: path.to_string(),
        content: serde_yml::to_string(value)
            .unwrap_or_else(|_| serde_json::to_string_pretty(value).unwrap_or_default()),
    }
}

fn effective_base(collection: &ApiCollection, options: &ConvertOptions) -> String {
    if let Some(url) = options
        .base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return url.trim_end_matches('/').to_string();
    }
    base_url(collection, options)
        .trim_end_matches('/')
        .to_string()
}

fn request_yaml(
    request: &crate::ApiRequest,
    workspace_id: &str,
    folder_id: &str,
    sort_priority: f64,
) -> Value {
    let mut url_parameters = Vec::new();
    for param in &request.path_params {
        url_parameters.push(json!({
            "enabled": true,
            "name": format!(":{}", param.name),
            "value": param.value,
            "id": Value::Null,
        }));
    }
    for param in &request.query_params {
        url_parameters.push(json!({
            "enabled": true,
            "name": param.name,
            "value": param.value,
            "id": Value::Null,
        }));
    }
    if let Some(ApiAuth::ApiKey {
        name,
        location: ParameterLocation::Query,
        value,
    }) = auth_for_request(request)
    {
        url_parameters.push(json!({
            "enabled": true,
            "name": name,
            "value": value,
            "id": Value::Null,
        }));
    }

    let mut body = json!({});
    let mut body_type = Value::Null;
    if let Some(api_body) = request.body.as_ref() {
        body_type = Value::String(api_body.content_type.clone());
        body = match api_body.kind {
            BodyKind::Json | BodyKind::Text | BodyKind::Other => {
                json!({"text": api_body.raw.clone().unwrap_or_default()})
            }
            BodyKind::UrlEncoded => json!({
                "text": api_body.fields.iter()
                    .map(|f| format!("{}={}", f.name, f.value))
                    .collect::<Vec<_>>()
                    .join("&")
            }),
            BodyKind::FormData => json!({
                "text": api_body.fields.iter()
                    .map(|f| format!("{}={}", f.name, f.value))
                    .collect::<Vec<_>>()
                    .join("\n")
            }),
        };
    }

    json!({
        "model": "http_request",
        "id": stable_id("rq", &format!("yaak:{}:{}", request.method, request.name)),
        "workspaceId": workspace_id,
        "folderId": folder_id,
        "createdAt": TIMESTAMP,
        "updatedAt": TIMESTAMP,
        "name": request.name,
        "method": request.method,
        "url": yaak_url(request),
        "description": request.description,
        "sortPriority": sort_priority,
        "headers": yaak_headers(request),
        "urlParameters": url_parameters,
        "authentication": {},
        "authenticationType": "none",
        "body": body,
        "bodyType": body_type,
    })
}

fn yaak_url(request: &crate::ApiRequest) -> String {
    let path = client_path(&request.path);
    let path = if path.starts_with('/') {
        path
    } else {
        format!("/{path}")
    };
    format!("${{[ base_url ]}}{path}")
}

fn yaak_headers(request: &crate::ApiRequest) -> Vec<Value> {
    let mut headers = request
        .headers
        .iter()
        .map(|param| json!({"name": param.name, "value": param.value, "enabled": true, "id": Value::Null}))
        .collect::<Vec<_>>();
    if let Some(body) = request.body.as_ref() {
        if !headers.iter().any(|h| {
            h.get("name")
                .and_then(Value::as_str)
                .map(|n| n.eq_ignore_ascii_case("content-type"))
                .unwrap_or(false)
        }) {
            headers.push(json!({
                "name": "Content-Type",
                "value": body.content_type,
                "enabled": true,
                "id": Value::Null,
            }));
        }
    }
    match auth_for_request(request) {
        Some(ApiAuth::Bearer { token }) | Some(ApiAuth::OAuth2 { token }) => {
            headers.push(json!({
                "name": "Authorization",
                "value": format!("Bearer {token}"),
                "enabled": true,
                "id": Value::Null,
            }));
        }
        Some(ApiAuth::Basic { username, password }) => {
            headers.push(json!({
                "name": "Authorization",
                "value": format!("Basic {username}:{password}"),
                "enabled": true,
                "id": Value::Null,
            }));
        }
        Some(ApiAuth::ApiKey {
            name,
            location: ParameterLocation::Header,
            value,
        }) => {
            headers.push(json!({
                "name": name,
                "value": value,
                "enabled": true,
                "id": Value::Null,
            }));
        }
        Some(ApiAuth::ApiKey {
            name,
            location: ParameterLocation::Cookie,
            value,
        }) => {
            headers.push(json!({
                "name": "Cookie",
                "value": format!("{name}={value}"),
                "enabled": true,
                "id": Value::Null,
            }));
        }
        _ => {}
    }
    headers
}
