use std::fmt::Display;

use api_core::{convert_openapi, convert_openapi_all, parse_openapi, ClientFormat, ConvertOptions};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

fn convert_options(name: &str, base_url: &str) -> ConvertOptions {
    ConvertOptions {
        collection_name: optional(name),
        base_url: optional(base_url),
    }
}

fn optional(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn js_error(error: impl Display) -> JsError {
    JsError::new(&error.to_string())
}

fn files_value(files: &[api_core::ConversionFile]) -> Value {
    json!({
        "files": files.iter().map(|file| json!({
            "format": file.format.as_str(),
            "path": file.path,
            "content": file.content,
        })).collect::<Vec<_>>()
    })
}

/// Supported client formats as a JSON array of strings.
#[wasm_bindgen]
pub fn formats() -> String {
    json!(ClientFormat::ALL
        .iter()
        .map(|format| format.as_str())
        .collect::<Vec<_>>())
    .to_string()
}

/// Parse an OpenAPI document and return a JSON summary for the UI.
#[wasm_bindgen]
pub fn openapi_info(source: &str) -> Result<String, JsError> {
    let collection = parse_openapi(source).map_err(js_error)?;
    let folders = collection
        .folders
        .iter()
        .map(|folder| {
            json!({
                "name": folder.name,
                "requests": folder.requests.len(),
            })
        })
        .collect::<Vec<_>>();
    let requests = collection
        .folders
        .iter()
        .map(|folder| folder.requests.len())
        .sum::<usize>();
    Ok(json!({
        "name": collection.name,
        "description": collection.description,
        "version": collection.version,
        "servers": collection.servers,
        "folders": folders,
        "requests": requests,
    })
    .to_string())
}

/// Convert an OpenAPI document to one client format. Returns JSON files.
#[wasm_bindgen]
pub fn convert(source: &str, format: &str, name: &str, base_url: &str) -> Result<String, JsError> {
    let format = ClientFormat::parse(format).ok_or_else(|| JsError::new("unsupported format"))?;
    let files =
        convert_openapi(source, format, &convert_options(name, base_url)).map_err(js_error)?;
    Ok(files_value(&files).to_string())
}

/// Convert an OpenAPI document to every supported format. Returns JSON files.
#[wasm_bindgen]
pub fn convert_all(source: &str, name: &str, base_url: &str) -> Result<String, JsError> {
    let files = convert_openapi_all(source, &convert_options(name, base_url)).map_err(js_error)?;
    Ok(files_value(&files).to_string())
}
