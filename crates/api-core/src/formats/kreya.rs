use serde_json::{json, Value};

use crate::formats::util::{
    auth_for_request, base_url, collection_name, pseudo_uuid, slugify, unique_slug,
};
use crate::{
    ApiAuth, ApiCollection, BodyKind, ClientFormat, ConversionFile, ConvertOptions,
    ParameterLocation, Result,
};

pub fn convert(
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    let name = collection_name(collection, options);
    let slug = slugify(&name);
    let base = effective_base(collection, options);
    let project_id = pseudo_uuid(&format!("kreya:{name}"));

    let mut files = vec![
        ConversionFile {
            format: ClientFormat::Kreya,
            path: format!("{slug}/{slug}.krproj"),
            content: format!(
                "{}\n",
                serde_json::to_string_pretty(&json!({
                    "id": project_id,
                    "importStreams": [],
                    "authConfigs": [],
                    "certificates": [],
                    "proxies": []
                }))
                .map_err(|e| crate::ApiConvertError::Conversion(e.to_string()))?
            ),
        },
        ConversionFile {
            format: ClientFormat::Kreya,
            path: format!("{slug}/directory.krpref"),
            content: format!(
                "{}\n",
                serde_json::to_string_pretty(&json!({
                    "settings": [{
                        "options": {
                            "rest": {
                                "endpoint": base,
                                "headers": []
                            }
                        }
                    }]
                }))
                .map_err(|e| crate::ApiConvertError::Conversion(e.to_string()))?
            ),
        },
        ConversionFile {
            format: ClientFormat::Kreya,
            path: format!("{slug}/development.krenv"),
            content: format!(
                "{}\n",
                serde_json::to_string_pretty(&json!({
                    "id": pseudo_uuid(&format!("kreya:{name}:env")),
                    "color": "#2563eb",
                    "globalContent": {
                        "rest_endpoint": base
                    }
                }))
                .map_err(|e| crate::ApiConvertError::Conversion(e.to_string()))?
            ),
        },
    ];

    let mut used_folders = Vec::new();
    for folder in &collection.folders {
        let folder_slug = unique_slug(&folder.name, &mut used_folders);
        for request in &folder.requests {
            files.extend(operation_files(request, &slug, &folder_slug));
        }
    }
    Ok(files)
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

fn operation_files(
    request: &crate::ApiRequest,
    project_slug: &str,
    folder_slug: &str,
) -> Vec<ConversionFile> {
    let op_name = file_safe(&request.name);
    let mut files = vec![ConversionFile {
        format: ClientFormat::Kreya,
        path: format!("{project_slug}/{folder_slug}/{op_name}.krop"),
        content: format!(
            "{}\n",
            serde_json::to_string_pretty(&operation_json(request)).unwrap_or_default()
        ),
    }];

    if let Some((file_name, content)) = request_body_file(request, &op_name) {
        files.push(ConversionFile {
            format: ClientFormat::Kreya,
            path: format!("{project_slug}/{folder_slug}/{file_name}"),
            content,
        });
    }
    files
}

fn request_body_file(request: &crate::ApiRequest, op_name: &str) -> Option<(String, String)> {
    let body = request.body.as_ref()?;
    let json_body = |value: Value| {
        (
            format!("{op_name}-request.json"),
            format!(
                "{}\n",
                serde_json::to_string_pretty(&value).unwrap_or_default()
            ),
        )
    };
    match body.kind {
        BodyKind::Json => Some((
            format!("{op_name}-request.json"),
            format!("{}\n", body.raw.clone().unwrap_or_default()),
        )),
        BodyKind::Text | BodyKind::Other => Some((
            format!("{op_name}-request.txt"),
            format!("{}\n", body.raw.clone().unwrap_or_default()),
        )),
        BodyKind::UrlEncoded => Some(json_body(json!({
            "fields": body.fields.iter().map(|f| json!({
                "key": f.name,
                "value": f.value
            })).collect::<Vec<_>>()
        }))),
        BodyKind::FormData => Some(json_body(json!({
            "fields": body.fields.iter().map(|f| json!({
                "key": f.name,
                "value": f.value,
                "type": if f.value == "<file>" { "file" } else { "text" }
            })).collect::<Vec<_>>()
        }))),
    }
}

fn operation_json(request: &crate::ApiRequest) -> Value {
    let mut headers = request
        .headers
        .iter()
        .map(|param| json!({"key": param.name, "value": param.value}))
        .collect::<Vec<_>>();
    if let Some(body) = request.body.as_ref() {
        if !headers.iter().any(|h| {
            h.get("key")
                .and_then(Value::as_str)
                .map(|k| k.eq_ignore_ascii_case("content-type"))
                .unwrap_or(false)
        }) {
            headers.push(json!({"key": "Content-Type", "value": body.content_type}));
        }
    }
    match auth_for_request(request) {
        Some(ApiAuth::Bearer { token }) | Some(ApiAuth::OAuth2 { token }) => {
            headers.push(json!({"key": "Authorization", "value": format!("Bearer {token}")}));
        }
        Some(ApiAuth::Basic { username, password }) => {
            headers.push(json!({
                "key": "Authorization",
                "value": format!("Basic {username}:{password}")
            }));
        }
        Some(ApiAuth::ApiKey {
            name,
            location: ParameterLocation::Header,
            value,
        }) => {
            headers.push(json!({"key": name, "value": value}));
        }
        Some(ApiAuth::ApiKey {
            name,
            location: ParameterLocation::Cookie,
            value,
        }) => {
            headers.push(json!({"key": "Cookie", "value": format!("{name}={value}")}));
        }
        _ => {}
    }

    let mut query_params = request
        .query_params
        .iter()
        .map(|param| json!({"key": param.name, "value": param.value}))
        .collect::<Vec<_>>();
    if let Some(ApiAuth::ApiKey {
        name,
        location: ParameterLocation::Query,
        value,
    }) = auth_for_request(request)
    {
        query_params.push(json!({"key": name, "value": value}));
    }

    let requests = match request_body_spec(request) {
        Some((location, content_type)) => {
            vec![json!({"location": location, "contentType": content_type})]
        }
        None => vec![json!({"contentType": "none"})],
    };

    json!({
        "details": {
            "path": request.path,
            "method": request.method,
            "headers": headers,
            "pathParams": request.path_params.iter().map(|p| json!({
                "key": p.name,
                "value": p.value
            })).collect::<Vec<_>>(),
            "queryParams": query_params,
        },
        "requests": requests,
        "operationType": "unary",
        "invokerName": "rest",
        "typeHint": request.method,
    })
}

fn request_body_spec(request: &crate::ApiRequest) -> Option<(String, String)> {
    let body = request.body.as_ref()?;
    let op_name = file_safe(&request.name);
    match body.kind {
        BodyKind::Json => Some((format!("{op_name}-request.json"), "json".to_string())),
        BodyKind::Text | BodyKind::Other => {
            Some((format!("{op_name}-request.txt"), "text".to_string()))
        }
        BodyKind::UrlEncoded => Some((
            format!("{op_name}-request.json"),
            "formUrlEncoded".to_string(),
        )),
        BodyKind::FormData => Some((
            format!("{op_name}-request.json"),
            "multipartFormData".to_string(),
        )),
    }
}

fn file_safe(name: &str) -> String {
    let safe = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.') {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();
    let safe = safe.trim().to_string();
    if safe.is_empty() {
        "request".to_string()
    } else {
        safe
    }
}
