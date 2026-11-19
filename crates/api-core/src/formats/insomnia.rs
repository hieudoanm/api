use serde_json::{json, Value};

use crate::formats::util::{
    auth_for_request, client_path, collection_name, request_server, slugify,
};
use crate::{
    ApiAuth, ApiBody, ApiCollection, BodyKind, ClientFormat, ConversionFile, ConvertOptions,
    ParameterLocation, Result,
};

pub fn convert(
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    let name = collection_name(collection, options);
    let base = effective_base(collection, options);
    let workspace_id = "wrk_api000000000000000000000001";
    let mut resources = vec![
        json!({
            "_id": workspace_id,
            "_type": "workspace",
            "name": name,
            "description": collection.description,
            "scope": "collection"
        }),
        json!({
            "_id": "env_base0000000000000000000001",
            "_type": "environment",
            "name": "Base Environment",
            "parentId": workspace_id,
            "data": { "base_url": base }
        }),
    ];

    for (folder_index, folder) in collection.folders.iter().enumerate() {
        let folder_id = format!("fld_{folder_index:032}");
        resources.push(json!({
            "_id": folder_id,
            "_type": "request_group",
            "name": folder.name,
            "parentId": workspace_id,
            "description": "",
            "environment": {},
            "environmentPropertyOrder": null,
            "metaSortKey": folder_index as i64
        }));
        for (request_index, request) in folder.requests.iter().enumerate() {
            resources.push(insomnia_request(
                request,
                &folder_id,
                folder_index,
                request_index,
            ));
        }
    }

    let document = json!({
        "_type": "export",
        "__export_format": 4,
        "__export_date": "2026-01-01T00:00:00.000Z",
        "__export_source": "api",
        "resources": resources
    });

    Ok(vec![ConversionFile {
        format: ClientFormat::Insomnia,
        path: format!("{}.insomnia.json", slugify(&name)),
        content: serde_json::to_string_pretty(&document)
            .map_err(|e| crate::ApiConvertError::Conversion(e.to_string()))?,
    }])
}

fn effective_base(collection: &ApiCollection, options: &ConvertOptions) -> String {
    let raw = if let Some(url) = options
        .base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        url.to_string()
    } else {
        request_server(
            &crate::ApiRequest {
                id: String::new(),
                name: String::new(),
                method: String::new(),
                path: String::new(),
                server: None,
                description: String::new(),
                tags: Vec::new(),
                path_params: Vec::new(),
                query_params: Vec::new(),
                headers: Vec::new(),
                body: None,
                auth: Vec::new(),
            },
            collection,
            options,
        )
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

fn insomnia_request(
    request: &crate::ApiRequest,
    parent_id: &str,
    folder_index: usize,
    request_index: usize,
) -> Value {
    let path = client_path(&request.path);
    let url = format!("{{{{ _.base_url }}}}{path}");
    let id = format!("req_{folder_index:04}{request_index:04}");
    let mut parameters = request
        .query_params
        .iter()
        .map(|param| {
            json!({
                "id": format!("pair_{}", slugify(&format!("{}-{}", param.name, param.value))),
                "name": param.name,
                "value": param.value,
                "description": param.description,
                "disabled": !param.required
            })
        })
        .collect::<Vec<_>>();
    if let Some(ApiAuth::ApiKey {
        name,
        location: ParameterLocation::Query,
        value,
    }) = auth_for_request(request)
    {
        parameters.push(json!({
            "id": format!("pair_auth_{id}"),
            "name": name,
            "value": value,
            "description": "",
            "disabled": false
        }));
    }

    let mut headers = request
        .headers
        .iter()
        .map(|param| {
            json!({
                "id": format!("pair_header_{}", slugify(&param.name)),
                "name": param.name,
                "value": param.value,
                "description": param.description,
                "disabled": false
            })
        })
        .collect::<Vec<_>>();
    if let Some(body) = request.body.as_ref() {
        if !headers.iter().any(|h| {
            h.get("name")
                .and_then(Value::as_str)
                .map(|n| n.eq_ignore_ascii_case("content-type"))
                .unwrap_or(false)
        }) {
            headers.push(json!({
                "id": "pair_header_content_type",
                "name": "Content-Type",
                "value": body.content_type,
                "description": "",
                "disabled": false
            }));
        }
    }

    json!({
        "_id": id,
        "parentId": parent_id,
        "created": 1767225600000i64,
        "modified": 1767225600000i64,
        "_type": "request",
        "url": url,
        "name": request.name,
        "description": request.description,
        "method": request.method,
        "body": insomnia_body(request.body.as_ref()),
        "parameters": parameters,
        "headers": headers,
        "pathParameters": request.path_params.iter().map(|param| json!({
            "name": param.name,
            "value": param.value
        })).collect::<Vec<_>>(),
        "authentication": insomnia_auth(auth_for_request(request)),
        "isPrivate": false,
        "settingStoreCookies": true,
        "settingSendCookies": true,
        "settingDisableRenderRequestBody": false,
        "settingRebuildPath": true,
        "settingFollowRedirects": "global",
        "metaSortKey": request_index as i64
    })
}

fn insomnia_body(body: Option<&ApiBody>) -> Value {
    let Some(body) = body else {
        return json!({});
    };
    match body.kind {
        BodyKind::Json | BodyKind::Text | BodyKind::Other => json!({
            "mimeType": body.content_type,
            "text": body.raw.clone().unwrap_or_default()
        }),
        BodyKind::UrlEncoded | BodyKind::FormData => json!({
            "mimeType": body.content_type,
            "params": body.fields.iter().map(|field| {
                let mut item = json!({
                    "id": format!("pair_body_{}", slugify(&field.name)),
                    "name": field.name,
                    "value": field.value,
                    "description": field.description,
                    "disabled": !field.required
                });
                if body.kind == BodyKind::FormData && field.value == "<file>" {
                    item["type"] = json!("file");
                }
                item
            }).collect::<Vec<_>>()
        }),
    }
}

fn insomnia_auth(auth: Option<&ApiAuth>) -> Value {
    match auth {
        Some(ApiAuth::Bearer { token }) | Some(ApiAuth::OAuth2 { token }) => json!({
            "type": "bearer",
            "token": token,
            "prefix": "Bearer",
            "disabled": false
        }),
        Some(ApiAuth::Basic { username, password }) => json!({
            "type": "basic",
            "username": username,
            "password": password,
            "useISO88591": false,
            "disabled": false
        }),
        Some(ApiAuth::ApiKey {
            name,
            location,
            value,
        }) => {
            let add_to = match location {
                ParameterLocation::Query => "queryParams",
                ParameterLocation::Cookie => "cookie",
                _ => "header",
            };
            json!({
                "type": "apikey",
                "key": name,
                "value": value,
                "addTo": add_to,
                "disabled": false
            })
        }
        _ => json!({}),
    }
}
