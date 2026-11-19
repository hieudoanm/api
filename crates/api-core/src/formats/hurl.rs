use crate::formats::util::{auth_for_request, base_url, collection_name, slugify, unique_slug};
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
    let mut files = Vec::new();
    let mut used_folders = Vec::new();

    for folder in &collection.folders {
        let folder_slug = unique_slug(&folder.name, &mut used_folders);
        let mut used = Vec::new();
        for request in &folder.requests {
            let filename = format!(
                "{}.hurl",
                unique_slug(&format!("{}-{}", request.method, request.name), &mut used)
            );
            files.push(ConversionFile {
                format: ClientFormat::Hurl,
                path: format!("{folder_slug}/{filename}"),
                content: request_file(request, &base),
            });
        }
    }
    let _ = name;
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

fn request_file(request: &crate::ApiRequest, base: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n", request.name));
    if !request.description.is_empty() {
        for line in request.description.lines() {
            out.push_str(&format!("# {}\n", line));
        }
    }
    out.push_str(&format!(
        "{} {}\n",
        request.method,
        request_url(base, request)
    ));
    for (name, value) in headers(request) {
        out.push_str(&format!("{name}: {value}\n"));
    }
    write_query_params(&mut out, request);
    if let Some(body) = request.body.as_ref() {
        out.push('\n');
        write_body(&mut out, body);
    }
    out
}

fn request_url(base: &str, request: &crate::ApiRequest) -> String {
    let mut path = request.path.clone();
    for param in &request.path_params {
        path = path.replace(&format!("{{{}}}", param.name), param.value.as_str());
    }
    let path = if path.starts_with('/') {
        path
    } else {
        format!("/{path}")
    };
    format!("{base}{path}")
}

fn write_query_params(out: &mut String, request: &crate::ApiRequest) {
    let mut params = request.query_params.clone();
    if let Some(ApiAuth::ApiKey {
        name,
        location: ParameterLocation::Query,
        value,
    }) = auth_for_request(request)
    {
        params.push(crate::ApiParameter {
            name: name.clone(),
            location: ParameterLocation::Query,
            required: true,
            description: String::new(),
            value: value.clone(),
        });
    }
    if params.is_empty() {
        return;
    }
    out.push_str("[QueryStringParams]\n");
    for param in &params {
        out.push_str(&format!("{}: {}\n", param.name, param.value));
    }
}

fn headers(request: &crate::ApiRequest) -> Vec<(String, String)> {
    let mut headers = request
        .headers
        .iter()
        .map(|param| (param.name.clone(), param.value.clone()))
        .collect::<Vec<_>>();
    if let Some(body) = request.body.as_ref() {
        if !headers
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("content-type"))
        {
            headers.push(("Content-Type".to_string(), body.content_type.clone()));
        }
    }
    match auth_for_request(request) {
        Some(ApiAuth::Bearer { token }) | Some(ApiAuth::OAuth2 { token }) => {
            headers.push(("Authorization".to_string(), format!("Bearer {token}")));
        }
        Some(ApiAuth::Basic { username, password }) => {
            headers.push((
                "Authorization".to_string(),
                format!("Basic {username}:{password}"),
            ));
        }
        Some(ApiAuth::ApiKey {
            name,
            location: ParameterLocation::Header,
            value,
        }) => {
            headers.push((name.clone(), value.clone()));
        }
        Some(ApiAuth::ApiKey {
            name,
            location: ParameterLocation::Cookie,
            value,
        }) => {
            headers.push(("Cookie".to_string(), format!("{name}={value}")));
        }
        _ => {}
    }
    headers
}

fn write_body(out: &mut String, body: &ApiBody) {
    match body.kind {
        BodyKind::Json | BodyKind::Text | BodyKind::Other => {
            out.push_str(&body.raw.clone().unwrap_or_default());
            out.push('\n');
        }
        BodyKind::UrlEncoded => {
            out.push_str("[FormParams]\n");
            for field in &body.fields {
                out.push_str(&format!("{}: {}\n", field.name, field.value));
            }
        }
        BodyKind::FormData => {
            out.push_str("[MultipartFormData]\n");
            for field in &body.fields {
                if field.value == "<file>" {
                    out.push_str(&format!(
                        "{}: file,path=path/to/{}\n",
                        field.name,
                        slugify(&field.name)
                    ));
                } else {
                    out.push_str(&format!("{}: {}\n", field.name, field.value));
                }
            }
        }
    }
}
