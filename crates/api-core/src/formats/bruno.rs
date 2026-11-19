use crate::formats::util::{
    auth_for_request, base_url, client_path, collection_name, slugify, unique_slug,
};
use crate::{
    ApiAuth, ApiBody, ApiCollection, ApiParameter, ApiRequest, BodyKind, ClientFormat,
    ConversionFile, ConvertOptions, ParameterLocation, Result,
};

pub fn convert(
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    let name = collection_name(collection, options);
    let base = effective_base(collection, options);
    let mut files = vec![
        ConversionFile {
            format: ClientFormat::Bruno,
            path: "bruno.json".to_string(),
            content: format!(
                "{}\n",
                serde_json::to_string_pretty(&serde_json::json!({
                    "version": "1",
                    "name": name,
                    "type": "collection",
                    "ignore": ["node_modules", ".git"]
                }))
                .map_err(|e| crate::ApiConvertError::Conversion(e.to_string()))?
            ),
        },
        ConversionFile {
            format: ClientFormat::Bruno,
            path: "collection.bru".to_string(),
            content: format!("meta {{\n  name: {}\n}}\n", bru_scalar(&name)),
        },
        ConversionFile {
            format: ClientFormat::Bruno,
            path: "environments/development.bru".to_string(),
            content: environment_file(&name, &base),
        },
    ];

    let mut seq = 1usize;
    let mut used_folders = Vec::new();
    for folder in &collection.folders {
        let folder_slug = unique_slug(&folder.name, &mut used_folders);
        files.push(ConversionFile {
            format: ClientFormat::Bruno,
            path: format!("{folder_slug}/folder.bru"),
            content: format!("meta {{\n  name: {}\n}}\n", bru_scalar(&folder.name)),
        });
        let mut used = Vec::new();
        for request in &folder.requests {
            let filename = format!(
                "{}.bru",
                unique_slug(&request_file_base(request), &mut used)
            );
            files.push(ConversionFile {
                format: ClientFormat::Bruno,
                path: format!("{folder_slug}/{filename}"),
                content: request_file(request, seq),
            });
            seq += 1;
        }
    }
    Ok(files)
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
        base_url(collection, options)
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

fn environment_file(name: &str, base: &str) -> String {
    let _ = name;
    format!(
        "vars {{\n  baseUrl: {}\n  accessToken: {{accessToken}}\n  apiKey: {{apiKey}}\n  username: {{username}}\n  password: {{password}}\n}}\n",
        bru_scalar(base)
    )
}

fn request_file(request: &ApiRequest, seq: usize) -> String {
    let method = request.method.to_ascii_lowercase();
    let url = format!("{{{{baseUrl}}}}{}", client_path(&request.path));
    let body_kind = request.body.as_ref().map(body_mode).unwrap_or("none");
    let auth_kind = auth_for_request(request).map(auth_mode).unwrap_or("none");
    let mut out = String::new();
    out.push_str("meta {\n");
    out.push_str(&format!("  name: {}\n", bru_scalar(&request.name)));
    out.push_str("  type: http\n");
    out.push_str(&format!("  seq: {seq}\n"));
    out.push_str("}\n\n");
    out.push_str(&format!("{method} {{\n"));
    out.push_str(&format!("  url: {}\n", bru_scalar(&url)));
    if body_kind != "none" {
        out.push_str(&format!("  body: {body_kind}\n"));
    }
    out.push_str(&format!("  auth: {auth_kind}\n"));
    out.push_str("}\n");

    write_query_params(&mut out, request);
    write_path_params(&mut out, request);
    write_headers(&mut out, request);
    write_auth(&mut out, auth_for_request(request));
    if let Some(body) = request.body.as_ref() {
        write_body(&mut out, body);
    }
    if !request.description.is_empty() {
        out.push_str("\ndocs {\n");
        for line in request.description.lines() {
            out.push_str(&format!("  {}\n", line.trim_end()));
        }
        out.push_str("}\n");
    }
    out
}

fn request_file_base(request: &ApiRequest) -> String {
    let path_slug = slugify(&request.path);
    format!("{}-{}", request.method, path_slug)
}

fn body_mode(body: &ApiBody) -> &'static str {
    match body.kind {
        BodyKind::Json => "json",
        BodyKind::UrlEncoded => "form-urlencoded",
        BodyKind::FormData => "multipart-form",
        BodyKind::Text | BodyKind::Other => "text",
    }
}

fn auth_mode(auth: &ApiAuth) -> &'static str {
    match auth {
        ApiAuth::Bearer { .. } | ApiAuth::OAuth2 { .. } => "bearer",
        ApiAuth::Basic { .. } => "basic",
        ApiAuth::ApiKey { location, .. } if *location == ParameterLocation::Header => "apikey",
        _ => "none",
    }
}

fn write_query_params(out: &mut String, request: &ApiRequest) {
    let mut params = request.query_params.clone();
    if let Some(ApiAuth::ApiKey {
        name,
        location: ParameterLocation::Query,
        value,
    }) = auth_for_request(request)
    {
        params.push(ApiParameter {
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
    out.push_str("\nparams:query {\n");
    for param in &params {
        out.push_str(&bru_field_line(param));
    }
    out.push_str("}\n");
}

fn write_path_params(out: &mut String, request: &ApiRequest) {
    if request.path_params.is_empty() {
        return;
    }
    out.push_str("\nparams:path {\n");
    for param in &request.path_params {
        out.push_str(&format!(
            "  {}: {}\n",
            bru_key(&param.name),
            bru_scalar(&param.value)
        ));
    }
    out.push_str("}\n");
}

fn write_headers(out: &mut String, request: &ApiRequest) {
    let mut headers = request.headers.clone();
    if let Some(body) = request.body.as_ref() {
        if !headers
            .iter()
            .any(|h| h.name.eq_ignore_ascii_case("content-type"))
        {
            headers.push(ApiParameter {
                name: "Content-Type".to_string(),
                location: ParameterLocation::Header,
                required: true,
                description: String::new(),
                value: body.content_type.clone(),
            });
        }
    }
    if headers.is_empty() {
        return;
    }
    out.push_str("\nheaders {\n");
    for header in headers {
        out.push_str(&bru_field_line(&header));
    }
    out.push_str("}\n");
}

fn write_auth(out: &mut String, auth: Option<&ApiAuth>) {
    match auth {
        Some(ApiAuth::Bearer { token }) | Some(ApiAuth::OAuth2 { token }) => {
            out.push_str("\nauth:bearer {\n");
            out.push_str(&format!("  token: {}\n", bru_scalar(token)));
            out.push_str("}\n");
        }
        Some(ApiAuth::Basic { username, password }) => {
            out.push_str("\nauth:basic {\n");
            out.push_str(&format!("  username: {}\n", bru_scalar(username)));
            out.push_str(&format!("  password: {}\n", bru_scalar(password)));
            out.push_str("}\n");
        }
        Some(ApiAuth::ApiKey {
            name,
            value,
            location,
        }) => {
            let placement = match location {
                ParameterLocation::Query => "query",
                _ => "header",
            };
            out.push_str("\nauth:apikey {\n");
            out.push_str(&format!("  key: {}\n", bru_scalar(name)));
            out.push_str(&format!("  value: {}\n", bru_scalar(value)));
            out.push_str(&format!("  placement: {placement}\n"));
            out.push_str("}\n");
        }
        _ => {}
    }
}

fn write_body(out: &mut String, body: &ApiBody) {
    match body.kind {
        BodyKind::Json => {
            out.push_str("\nbody:json {\n");
            if let Some(raw) = body.raw.as_ref() {
                out.push_str(raw);
                if !raw.ends_with('\n') {
                    out.push('\n');
                }
            }
            out.push_str("}\n");
        }
        BodyKind::UrlEncoded => {
            out.push_str("\nbody:form-urlencoded {\n");
            write_fields(out, &body.fields);
            out.push_str("}\n");
        }
        BodyKind::FormData => {
            out.push_str("\nbody:multipart-form {\n");
            write_fields(out, &body.fields);
            out.push_str("}\n");
        }
        BodyKind::Text | BodyKind::Other => {
            out.push_str("\nbody:text {\n");
            if let Some(raw) = body.raw.as_ref() {
                out.push_str(raw);
                if !raw.ends_with('\n') {
                    out.push('\n');
                }
            }
            out.push_str("}\n");
        }
    }
}

fn write_fields(out: &mut String, fields: &[ApiParameter]) {
    for field in fields {
        out.push_str(&bru_field_line(field));
    }
}

fn bru_field_line(param: &ApiParameter) -> String {
    let prefix = if param.required { "" } else { "~" };
    format!(
        "  {}{}: {}\n",
        prefix,
        bru_key(&param.name),
        bru_scalar(&param.value)
    )
}

fn bru_key(key: &str) -> String {
    if key
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
        && !key.starts_with('~')
    {
        key.to_string()
    } else {
        format!("\"{}\"", key.replace('\\', "\\\\").replace('"', "\\\""))
    }
}

fn bru_scalar(value: &str) -> String {
    let value = value.replace('\n', "\\n").replace('\r', "");
    if value.contains(':') || value.contains('#') || value.trim() != value {
        format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        value
    }
}
