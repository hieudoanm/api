use serde_json::{json, Value};

use crate::formats::util::{auth_for_request, collection_name, request_server, slugify};
use crate::{
    ApiAuth, ApiBody, ApiCollection, BodyKind, ClientFormat, ConversionFile, ConvertOptions,
    ParameterLocation, Result,
};

pub fn convert(
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    let name = collection_name(collection, options);
    let mut folders = Vec::new();
    for folder in &collection.folders {
        folders.push(json!({
            "v": 1,
            "name": folder.name,
            "folders": [],
            "requests": folder.requests.iter().map(|request| hopp_request(request, collection, options)).collect::<Vec<_>>()
        }));
    }
    let document = json!({
        "v": 1,
        "name": name,
        "folders": folders,
        "requests": []
    });
    Ok(vec![ConversionFile {
        format: ClientFormat::Hoppscotch,
        path: format!("{}.hoppscotch.json", slugify(&name)),
        content: serde_json::to_string_pretty(&document)
            .map_err(|e| crate::ApiConvertError::Conversion(e.to_string()))?,
    }])
}

fn hopp_request(
    request: &crate::ApiRequest,
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Value {
    let server = effective_server(request, collection, options);
    let endpoint = endpoint_url(&server, request);
    json!({
        "v": "1",
        "name": request.name,
        "endpoint": endpoint,
        "method": request.method,
        "params": request.query_params.iter().map(|param| json!({
            "key": param.name,
            "value": param.value,
            "active": param.required
        })).collect::<Vec<_>>(),
        "headers": hopp_headers(request),
        "auth": hopp_auth(auth_for_request(request)),
        "body": hopp_body(request.body.as_ref()),
        "preRequestScript": "",
        "testScript": ""
    })
}

fn effective_server(
    request: &crate::ApiRequest,
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> String {
    let raw = if let Some(url) = options
        .base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        url.to_string()
    } else {
        request_server(request, collection, options)
    };
    let raw = raw.trim_end_matches('/');
    if raw.starts_with("http://") || raw.starts_with("https://") || raw.is_empty() {
        raw.to_string()
    } else {
        format!(
            "https://api.example.com{}",
            if raw.starts_with('/') {
                raw.to_string()
            } else {
                format!("/{raw}")
            }
        )
    }
}

fn endpoint_url(server: &str, request: &crate::ApiRequest) -> String {
    let mut path = request.path.clone();
    for param in &request.path_params {
        path = path.replace(&format!("{{{}}}", param.name), &param.value);
    }
    let path = if path.starts_with('/') {
        path
    } else {
        format!("/{path}")
    };
    format!("{}{}", server.trim_end_matches('/'), path)
}

fn hopp_headers(request: &crate::ApiRequest) -> Vec<Value> {
    let mut headers = request
        .headers
        .iter()
        .map(|param| {
            json!({
                "key": param.name,
                "value": param.value,
                "active": true,
                "description": param.description
            })
        })
        .collect::<Vec<_>>();
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
                "active": true,
                "description": ""
            }));
        }
    }
    headers
}

fn hopp_body(body: Option<&ApiBody>) -> Value {
    let Some(body) = body else {
        return json!({
            "contentType": null,
            "body": null
        });
    };
    match body.kind {
        BodyKind::FormData => json!({
            "contentType": "multipart/form-data",
            "body": body.fields.iter().map(|field| json!({
                "key": field.name,
                "value": field.value,
                "active": field.required,
                "isFile": field.value == "<file>"
            })).collect::<Vec<_>>()
        }),
        BodyKind::UrlEncoded => json!({
            "contentType": "application/x-www-form-urlencoded",
            "body": body.fields.iter().map(|field| format!("{}={}", field.name, field.value)).collect::<Vec<_>>().join("&")
        }),
        _ => {
            let content_type = match body.content_type.as_str() {
                "application/json"
                | "application/ld+json"
                | "application/hal+json"
                | "application/vnd.api+json"
                | "application/xml"
                | "application/x-www-form-urlencoded"
                | "text/html"
                | "text/plain" => body.content_type.as_str(),
                _ if crate::formats::util::is_json_content_type(&body.content_type) => {
                    "application/json"
                }
                _ => "text/plain",
            };
            json!({
                "contentType": content_type,
                "body": body.raw.clone().unwrap_or_default()
            })
        }
    }
}

fn hopp_auth(auth: Option<&ApiAuth>) -> Value {
    match auth {
        Some(ApiAuth::Bearer { token }) | Some(ApiAuth::OAuth2 { token }) => json!({
            "authType": "bearer",
            "token": token,
            "authActive": true
        }),
        Some(ApiAuth::Basic { username, password }) => json!({
            "authType": "basic",
            "username": username,
            "password": password,
            "authActive": true
        }),
        Some(ApiAuth::ApiKey {
            name,
            value,
            location,
        }) => {
            let add_to = match location {
                ParameterLocation::Query => "QueryParams",
                ParameterLocation::Cookie => "Cookie",
                _ => "Headers",
            };
            json!({
                "authType": "api-key",
                "key": name,
                "value": value,
                "addTo": add_to,
                "authActive": true
            })
        }
        _ => json!({
            "authType": "none",
            "authActive": true
        }),
    }
}
