use serde_json::{json, Map, Value};

use crate::formats::util::{
    auth_for_request, base_url, client_path, collection_name, is_json_content_type, slugify,
    unique_slug,
};
use crate::{
    ApiAuth, ApiBody, ApiCollection, ApiParameter, BodyKind, ClientFormat, ConversionFile,
    ConvertOptions, ParameterLocation, Result,
};

pub fn convert(
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    let name = collection_name(collection, options);
    let base = request_server_base(collection, options);
    let mut items = Vec::new();
    for folder in &collection.folders {
        let mut used = Vec::new();
        let requests = folder
            .requests
            .iter()
            .map(|request| postman_item(request, &base, &mut used))
            .collect::<Vec<_>>();
        items.push(json!({
            "name": folder.name,
            "item": requests,
        }));
    }

    let document = json!({
        "info": {
            "name": name,
            "description": collection.description,
            "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json",
            "version": collection.version,
        },
        "item": items,
        "variable": [{
            "key": "baseUrl",
            "value": base,
            "type": "string"
        }]
    });

    Ok(vec![ConversionFile {
        format: ClientFormat::Postman,
        path: format!("{}.postman_collection.json", slugify(&name)),
        content: serde_json::to_string_pretty(&document)
            .map_err(|e| crate::ApiConvertError::Conversion(e.to_string()))?,
    }])
}

fn request_server_base(collection: &ApiCollection, options: &ConvertOptions) -> String {
    if let Some(url) = options
        .base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return url.trim_end_matches('/').to_string();
    }
    base_url(collection, options)
}

fn postman_item(request: &crate::ApiRequest, base: &str, used: &mut Vec<String>) -> Value {
    let _ = base;
    let mut request_obj = Map::new();
    request_obj.insert("method".to_string(), Value::String(request.method.clone()));
    request_obj.insert("header".to_string(), Value::Array(postman_headers(request)));
    request_obj.insert("url".to_string(), postman_url(request));
    if let Some(body) = request.body.as_ref().and_then(postman_body) {
        request_obj.insert("body".to_string(), body);
    }
    if let Some(auth) = auth_for_request(request).and_then(postman_auth) {
        request_obj.insert("auth".to_string(), auth);
    }
    if !request.description.is_empty() {
        request_obj.insert(
            "description".to_string(),
            Value::String(request.description.clone()),
        );
    }

    json!({
        "name": unique_request_name(request, used),
        "request": Value::Object(request_obj),
        "response": []
    })
}

fn unique_request_name(request: &crate::ApiRequest, used: &mut Vec<String>) -> String {
    unique_slug(&format!("{}-{}", request.method, request.name), used)
}

fn postman_url(request: &crate::ApiRequest) -> Value {
    let path = client_path(&request.path);
    let path_segments = path
        .split('/')
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let query = request
        .query_params
        .iter()
        .map(|param| {
            json!({
                "key": param.name,
                "value": param.value,
                "description": param.description,
                "type": "text",
                "disabled": !param.required
            })
        })
        .collect::<Vec<_>>();
    let variable = request
        .path_params
        .iter()
        .map(|param| {
            json!({
                "key": param.name,
                "value": param.value,
                "description": param.description
            })
        })
        .collect::<Vec<_>>();
    let raw = raw_url(&path, &request.query_params);
    json!({
        "raw": raw,
        "host": ["{{baseUrl}}"],
        "path": path_segments,
        "query": query,
        "variable": variable
    })
}

fn raw_url(path: &str, query_params: &[ApiParameter]) -> String {
    let mut raw = format!("{{{{baseUrl}}}}{path}");
    if !query_params.is_empty() {
        let query = query_params
            .iter()
            .map(|param| format!("{}={}", param.name, param.value))
            .collect::<Vec<_>>()
            .join("&");
        raw.push('?');
        raw.push_str(&query);
    }
    raw
}

fn postman_headers(request: &crate::ApiRequest) -> Vec<Value> {
    let mut headers = Vec::new();
    for param in &request.headers {
        headers.push(json!({
            "key": param.name,
            "value": param.value,
            "description": param.description,
            "type": "text"
        }));
    }
    if let Some(body) = request.body.as_ref() {
        if !headers.iter().any(|h| {
            h.get("key")
                .and_then(Value::as_str)
                .map(|k| k.eq_ignore_ascii_case("content-type"))
                .unwrap_or(false)
        }) {
            headers.push(json!({
                "key": "Content-Type",
                "value": body.content_type,
                "type": "text"
            }));
        }
    }
    if let Some(ApiAuth::ApiKey {
        name,
        location: ParameterLocation::Header,
        ..
    }) = auth_for_request(request)
    {
        if !headers.iter().any(|h| {
            h.get("key")
                .and_then(Value::as_str)
                .map(|k| k.eq_ignore_ascii_case(name))
                .unwrap_or(false)
        }) {
            headers.push(json!({
                "key": name,
                "value": "{{apiKey}}",
                "type": "text"
            }));
        }
    }
    headers
}

fn postman_body(body: &ApiBody) -> Option<Value> {
    match body.kind {
        BodyKind::Json => body.raw.as_ref().map(|raw| {
            json!({
                "mode": "raw",
                "raw": raw,
                "options": { "raw": { "language": "json" } }
            })
        }),
        BodyKind::UrlEncoded => Some(json!({
            "mode": "urlencoded",
            "urlencoded": body.fields.iter().map(|field| json!({
                "key": field.name,
                "value": field.value,
                "description": field.description,
                "type": "text",
                "disabled": !field.required
            })).collect::<Vec<_>>()
        })),
        BodyKind::FormData => Some(json!({
            "mode": "formdata",
            "formdata": body.fields.iter().map(|field| json!({
                "key": field.name,
                "value": field.value,
                "description": field.description,
                "type": if field.value == "<file>" { "file" } else { "text" }
            })).collect::<Vec<_>>()
        })),
        BodyKind::Text | BodyKind::Other => body.raw.as_ref().map(|raw| {
            let language = if is_json_content_type(&body.content_type) {
                "json"
            } else {
                "text"
            };
            json!({
                "mode": "raw",
                "raw": raw,
                "options": { "raw": { "language": language } }
            })
        }),
    }
}

fn postman_auth(auth: &ApiAuth) -> Option<Value> {
    match auth {
        ApiAuth::Bearer { token } | ApiAuth::OAuth2 { token } => Some(json!({
            "type": "bearer",
            "bearer": [{ "key": "token", "value": token, "type": "string" }]
        })),
        ApiAuth::Basic { username, password } => Some(json!({
            "type": "basic",
            "basic": [
                { "key": "username", "value": username, "type": "string" },
                { "key": "password", "value": password, "type": "string" }
            ]
        })),
        ApiAuth::ApiKey {
            name,
            location,
            value,
        } => {
            let location = match location {
                ParameterLocation::Query => "query",
                _ => "header",
            };
            Some(json!({
                "type": "apikey",
                "apikey": [
                    { "key": "key", "value": name, "type": "string" },
                    { "key": "value", "value": value, "type": "string" },
                    { "key": "in", "value": location, "type": "string" }
                ]
            }))
        }
        ApiAuth::Other { .. } => None,
    }
}
