//! MCP (Model Context Protocol) server for api.
//!
//! Speaks newline-delimited JSON-RPC 2.0 over stdio, protocol version
//! `2024-11-05` — the same shape MCP clients expect:
//! `initialize` → `notifications/initialized` → `tools/list` → `tools/call`.
//!
//! The core is [`handle_message`], which maps one JSON-RPC message to an
//! optional response — testable without stdio. [`serve`] runs the stdio loop.

use std::io::{BufRead, Write};

use serde_json::{json, Value};

use api_core::{convert_openapi, parse_openapi, ClientFormat, ConvertOptions};

const PROTOCOL_VERSION: &str = "2024-11-05";
const SERVER_NAME: &str = "api";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Tool definitions exposed via `tools/list`.
fn tools() -> Value {
    json!([
        {
            "name": "list_formats",
            "description": "List the API client collection formats api can emit (postman, bruno, insomnia, hoppscotch, rest-client, thunder-client, yaak, hurl, kreya, curl).",
            "inputSchema": {"type": "object", "properties": {}}
        },
        {
            "name": "openapi_info",
            "description": "Parse an OpenAPI 3.x document (JSON or YAML) and return a summary: title, version, description, servers, and folders with their request counts. Provide `source` (spec text) or `path` (file on disk).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "source": {"type": "string", "description": "OpenAPI document text"},
                    "path": {"type": "string", "description": "Path to an OpenAPI JSON/YAML file"}
                }
            }
        },
        {
            "name": "convert_openapi",
            "description": "Convert an OpenAPI 3.x document into API client collection files. Returns the generated files as {format, path, content} entries — the caller decides where to write them. `formats` accepts a comma-separated list or \"all\" (default: postman).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "source": {"type": "string", "description": "OpenAPI document text"},
                    "path": {"type": "string", "description": "Path to an OpenAPI JSON/YAML file"},
                    "formats": {"type": "string", "description": "Comma-separated formats or \"all\" (default \"postman\")"},
                    "name": {"type": "string", "description": "Collection name override (default: info.title)"},
                    "base_url": {"type": "string", "description": "Base URL override for all requests"}
                }
            }
        }
    ])
}

fn tool_result(payload: Value) -> Value {
    json!({
        "content": [{"type": "text", "text": payload.to_string()}],
        "isError": false
    })
}

fn tool_error(message: &str) -> Value {
    json!({
        "content": [{"type": "text", "text": message}],
        "isError": true
    })
}

fn arg_str<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args.get(key).and_then(Value::as_str)
}

/// Resolve the OpenAPI document text from `source` or `path`.
fn spec_source(args: &Value) -> Result<String, String> {
    if let Some(source) = arg_str(args, "source") {
        return Ok(source.to_string());
    }
    if let Some(path) = arg_str(args, "path") {
        return std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e));
    }
    Err("missing required argument: source or path".to_string())
}

fn parse_formats_arg(value: Option<&str>) -> Result<Vec<ClientFormat>, String> {
    let Some(raw) = value else {
        return Ok(vec![ClientFormat::Postman]);
    };
    let mut formats = Vec::new();
    for part in raw.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.eq_ignore_ascii_case("all") {
            for format in ClientFormat::ALL {
                if !formats.contains(&format) {
                    formats.push(format);
                }
            }
            continue;
        }
        let format = ClientFormat::parse(trimmed).ok_or_else(|| {
            format!(
                "unsupported format {trimmed}; expected one of: postman, bruno, insomnia, hoppscotch, rest-client, thunder-client, yaak, hurl, kreya, curl"
            )
        })?;
        if !formats.contains(&format) {
            formats.push(format);
        }
    }
    if formats.is_empty() {
        return Err("no formats given".to_string());
    }
    Ok(formats)
}

fn convert_options(args: &Value) -> ConvertOptions {
    ConvertOptions {
        collection_name: arg_str(args, "name").map(str::to_string),
        base_url: arg_str(args, "base_url").map(str::to_string),
    }
}

fn call_tool(name: &str, args: &Value) -> Value {
    match name {
        "list_formats" => {
            let formats: Vec<&str> = ClientFormat::ALL.iter().map(|f| f.as_str()).collect();
            tool_result(json!({"formats": formats}))
        }
        "openapi_info" => {
            match spec_source(args).and_then(|src| parse_openapi(&src).map_err(|e| e.to_string())) {
                Ok(collection) => {
                    let folders: Vec<Value> = collection
                        .folders
                        .iter()
                        .map(|f| {
                            json!({
                                "name": f.name,
                                "requests": f.requests.len(),
                            })
                        })
                        .collect();
                    let requests: usize = collection.folders.iter().map(|f| f.requests.len()).sum();
                    tool_result(json!({
                        "name": collection.name,
                        "description": collection.description,
                        "version": collection.version,
                        "servers": collection.servers,
                        "folders": folders,
                        "requests": requests,
                    }))
                }
                Err(e) => tool_error(&e),
            }
        }
        "convert_openapi" => {
            let formats = match parse_formats_arg(arg_str(args, "formats")) {
                Ok(f) => f,
                Err(e) => return tool_error(&e),
            };
            let source = match spec_source(args) {
                Ok(s) => s,
                Err(e) => return tool_error(&e),
            };
            let options = convert_options(args);
            let mut files = Vec::new();
            for format in formats {
                match convert_openapi(&source, format, &options) {
                    Ok(mut produced) => files.append(&mut produced),
                    Err(e) => return tool_error(&format!("{}: {}", format.as_str(), e)),
                }
            }
            let files: Vec<Value> = files
                .iter()
                .map(|f| {
                    json!({
                        "format": f.format.as_str(),
                        "path": f.path,
                        "content": f.content,
                    })
                })
                .collect();
            tool_result(json!({"files": files, "count": files.len()}))
        }
        other => tool_error(&format!("unknown tool: {}", other)),
    }
}

/// Handle one JSON-RPC message. Returns `Some(response)` for requests,
/// `None` for notifications (and malformed input).
pub fn handle_message(msg: &Value) -> Option<Value> {
    let id = msg.get("id").cloned();
    let method = msg.get("method").and_then(Value::as_str)?;
    let params = msg.get("params").cloned().unwrap_or_else(|| json!({}));

    let result = match method {
        "initialize" => json!({
            "protocolVersion": PROTOCOL_VERSION,
            "capabilities": {"tools": {}},
            "serverInfo": {"name": SERVER_NAME, "version": SERVER_VERSION}
        }),
        "ping" => json!({}),
        "tools/list" => json!({"tools": tools()}),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            call_tool(name, &args)
        }
        // notifications/initialized and anything else: no response.
        _ if id.is_none() => return None,
        _ => {
            return Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {"code": -32601, "message": format!("method not found: {}", method)}
            }));
        }
    };

    Some(json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    }))
}

/// Run the MCP stdio server loop (newline-delimited JSON-RPC).
pub fn serve() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(msg) = serde_json::from_str::<Value>(trimmed) else {
            continue; // ignore malformed lines
        };
        if let Some(resp) = handle_message(&msg) {
            writeln!(out, "{}", resp)?;
            out.flush()?;
        }
    }
    Ok(())
}
