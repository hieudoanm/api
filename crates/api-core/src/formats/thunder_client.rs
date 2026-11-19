use serde_json::{json, Map, Value};

use crate::formats::util::{
    auth_for_request, base_url, client_path, collection_name, pseudo_uuid, slugify,
};
use crate::{
    ApiAuth, ApiBody, ApiCollection, BodyKind, ClientFormat, ConversionFile, ConvertOptions,
    ParameterLocation, Result,
};

const TIMESTAMP: &str = "2024-01-01T00:00:00.000Z";

pub fn convert(
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    let name = collection_name(collection, options);
    let base = if let Some(url) = options
        .base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        url.to_string()
    } else {
        base_url(collection, options)
    };
    let base = base.trim_end_matches('/').to_string();
    let collection_id = pseudo_uuid(&format!("thunder:{name}"));

    let mut folders = Vec::new();
    let mut requests = Vec::new();
    let mut sort_num = 10000usize;
    for folder in &collection.folders {
        let folder_id = pseudo_uuid(&format!("thunder:{name}:folder:{}", folder.name));
        folders.push(json!({
            "_id": folder_id,
            "name": folder.name,
            "created": TIMESTAMP,
            "modified": TIMESTAMP,
        }));
        for request in &folder.requests {
            requests.push(thunder_request(
                request,
                &base,
                &collection_id,
                &folder_id,
                sort_num,
            ));
            sort_num += 10000;
        }
    }

    let document = json!({
        "client": "Thunder Client",
        "collectionName": name,
        "collectionId": collection_id,
        "dateExported": TIMESTAMP,
        "version": "1.1",
        "folders": folders,
        "requests": requests,
        "settings": {
            "headers": [],
            "tests": []
        }
    });

    Ok(vec![ConversionFile {
        format: ClientFormat::ThunderClient,
        path: format!("thunder-collection_{}.json", slugify(&name)),
        content: serde_json::to_string_pretty(&document)
            .map_err(|e| crate::ApiConvertError::Conversion(e.to_string()))?,
    }])
}

fn thunder_request(
    request: &crate::ApiRequest,
    base: &str,
    collection_id: &str,
    folder_id: &str,
    sort_num: usize,
) -> Value {
    let mut obj = Map::new();
    obj.insert(
        "_id".to_string(),
        Value::String(pseudo_uuid(&format!(
            "thunder:{collection_id}:{}",
            request.name
        ))),
    );
    obj.insert(
        "colId".to_string(),
        Value::String(collection_id.to_string()),
    );
    obj.insert(
        "containerId".to_string(),
        Value::String(folder_id.to_string()),
    );
    obj.insert("name".to_string(), Value::String(request.name.clone()));
    obj.insert("url".to_string(), Value::String(request_url(base, request)));
    obj.insert("method".to_string(), Value::String(request.method.clone()));
    obj.insert("sortNum".to_string(), json!(sort_num));
    obj.insert("created".to_string(), Value::String(TIMESTAMP.to_string()));
    obj.insert("modified".to_string(), Value::String(TIMESTAMP.to_string()));
    obj.insert(
        "headers".to_string(),
        Value::Array(thunder_headers(request)),
    );
    obj.insert("params".to_string(), Value::Array(thunder_params(request)));
    if let Some(body) = request.body.as_ref() {
        obj.insert("body".to_string(), thunder_body(body));
    }
    obj.insert("tests".to_string(), Value::Array(vec![]));
    Value::Object(obj)
}

fn request_url(base: &str, request: &crate::ApiRequest) -> String {
    let mut path = client_path(&request.path);
    for param in &request.path_params {
        path = path.replace(
            &format!(":{}", param.name),
            &format!("{{{{{}}}}}", param.name),
        );
    }
    if !path.starts_with('/') {
        path.insert(0, '/');
    }
    let mut url = format!("{base}{path}");
    let query = request
        .query_params
        .iter()
        .map(|param| format!("{}={}", param.name, param.value))
        .collect::<Vec<_>>();
    if !query.is_empty() {
        url.push('?');
        url.push_str(&query.join("&"));
    }
    url
}

fn thunder_headers(request: &crate::ApiRequest) -> Vec<Value> {
    let mut headers = request
        .headers
        .iter()
        .map(|param| json!({"name": param.name, "value": param.value}))
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
            }));
        }
    }
    match auth_for_request(request) {
        Some(ApiAuth::Bearer { token }) | Some(ApiAuth::OAuth2 { token }) => {
            headers.push(json!({"name": "Authorization", "value": format!("Bearer {token}")}));
        }
        Some(ApiAuth::Basic { username, password }) => {
            headers.push(json!({
                "name": "Authorization",
                "value": format!("Basic {username}:{password}"),
            }));
        }
        Some(ApiAuth::ApiKey {
            name,
            location: ParameterLocation::Header,
            value,
        }) => {
            headers.push(json!({"name": name, "value": value}));
        }
        Some(ApiAuth::ApiKey {
            name,
            location: ParameterLocation::Cookie,
            value,
        }) => {
            headers.push(json!({"name": "Cookie", "value": format!("{name}={value}")}));
        }
        _ => {}
    }
    headers
}

fn thunder_params(request: &crate::ApiRequest) -> Vec<Value> {
    let mut params = request
        .path_params
        .iter()
        .map(|param| json!({"name": param.name, "value": param.value, "isPath": true}))
        .collect::<Vec<_>>();
    params.extend(
        request
            .query_params
            .iter()
            .map(|param| json!({"name": param.name, "value": param.value, "isPath": false})),
    );
    if let Some(ApiAuth::ApiKey {
        name,
        location: ParameterLocation::Query,
        value,
    }) = auth_for_request(request)
    {
        params.push(json!({"name": name, "value": value, "isPath": false}));
    }
    params
}

fn thunder_body(body: &ApiBody) -> Value {
    match body.kind {
        BodyKind::Json => json!({
            "type": "json",
            "raw": body.raw.clone().unwrap_or_default(),
            "form": []
        }),
        BodyKind::UrlEncoded => json!({
            "type": "formUrlEncoded",
            "raw": "",
            "form": body.fields.iter().map(|field| json!({
                "key": field.name,
                "value": field.value
            })).collect::<Vec<_>>()
        }),
        BodyKind::FormData => json!({
            "type": "formData",
            "raw": "",
            "form": body.fields.iter().map(|field| {
                let is_file = field.value == "<file>";
                json!({
                    "key": field.name,
                    "value": if is_file { field.name.clone() } else { field.value.clone() },
                    "type": if is_file { "file" } else { "text" }
                })
            }).collect::<Vec<_>>()
        }),
        BodyKind::Text | BodyKind::Other => json!({
            "type": "text",
            "raw": body.raw.clone().unwrap_or_default(),
            "form": []
        }),
    }
}
