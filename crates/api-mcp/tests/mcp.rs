use api_mcp::handle_message;
use serde_json::{json, Value};

const SPEC: &str = r#"{
    "openapi": "3.0.0",
    "info": {"title": "Petstore", "version": "1.0.0"},
    "servers": [{"url": "https://api.example.com/v1"}],
    "paths": {
        "/pets": {
            "get": {
                "tags": ["pets"],
                "summary": "List pets",
                "responses": {"200": {"description": "ok"}}
            }
        }
    }
}"#;

const YAML_SPEC: &str = r#"
openapi: 3.0.3
info:
  title: YAML Petstore
  version: 1.2.0
paths:
  /pets:
    get:
      summary: List pets YAML
      responses:
        '200':
          description: ok
"#;

fn rpc(method: &str, params: Value, id: u64) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params})
}

fn call_tool(name: &str, arguments: Value, id: u64) -> Value {
    handle_message(&rpc(
        "tools/call",
        json!({"name": name, "arguments": arguments}),
        id,
    ))
    .unwrap()
}

fn tool_payload(resp: &Value) -> Value {
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    serde_json::from_str(text).unwrap()
}

#[test]
fn initialize_handshake() {
    let resp = handle_message(&rpc("initialize", json!({}), 1)).unwrap();
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["id"], 1);
    assert_eq!(resp["result"]["protocolVersion"], "2024-11-05");
    assert_eq!(resp["result"]["serverInfo"]["name"], "api");
    assert!(resp["result"]["capabilities"]["tools"].is_object());
}

#[test]
fn ping_returns_empty_result() {
    let resp = handle_message(&rpc("ping", json!({}), 2)).unwrap();
    assert_eq!(resp["result"], json!({}));
}

#[test]
fn notifications_get_no_response() {
    let msg = json!({"jsonrpc": "2.0", "method": "notifications/initialized"});
    assert!(handle_message(&msg).is_none());
}

#[test]
fn unknown_notification_gets_no_response() {
    let msg = json!({"jsonrpc": "2.0", "method": "bogus/method"});
    assert!(handle_message(&msg).is_none());
}

#[test]
fn unknown_method_errors() {
    let resp = handle_message(&rpc("bogus/method", json!({}), 8)).unwrap();
    assert_eq!(resp["error"]["code"], -32601);
    assert!(resp["error"]["message"]
        .as_str()
        .unwrap()
        .contains("bogus/method"));
}

#[test]
fn malformed_message_gets_no_response() {
    assert!(handle_message(&json!({"no_method": true})).is_none());
    assert!(handle_message(&json!(null)).is_none());
}

#[test]
fn tools_list_has_expected_tools() {
    let resp = handle_message(&rpc("tools/list", json!({}), 2)).unwrap();
    let names: Vec<&str> = resp["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    for expected in ["list_formats", "openapi_info", "convert_openapi"] {
        assert!(names.contains(&expected), "missing tool {}", expected);
    }
    for tool in resp["result"]["tools"].as_array().unwrap() {
        assert!(tool["inputSchema"]["type"] == "object");
        assert!(tool["description"].as_str().unwrap().len() > 10);
    }
}

#[test]
fn list_formats_returns_all() {
    let resp = call_tool("list_formats", json!({}), 3);
    let parsed = tool_payload(&resp);
    let formats = parsed["formats"].as_array().unwrap();
    assert_eq!(formats.len(), 15);
    assert!(formats.contains(&json!("postman")));
    assert!(formats.contains(&json!("curl")));
    assert!(formats.contains(&json!("redoc")));
}

#[test]
fn openapi_info_summarizes_spec() {
    let resp = call_tool("openapi_info", json!({"source": SPEC}), 4);
    let parsed = tool_payload(&resp);
    assert_eq!(parsed["name"], "Petstore");
    assert_eq!(parsed["version"], "1.0.0");
    assert_eq!(parsed["requests"], 1);
    assert_eq!(parsed["servers"], json!(["https://api.example.com/v1"]));
    assert_eq!(parsed["folders"][0]["name"], "pets");
    assert_eq!(parsed["folders"][0]["requests"], 1);
}

#[test]
fn openapi_info_accepts_yaml_source() {
    let resp = call_tool("openapi_info", json!({"source": YAML_SPEC}), 5);
    let parsed = tool_payload(&resp);
    assert_eq!(parsed["name"], "YAML Petstore");
    assert_eq!(parsed["requests"], 1);
}

#[test]
fn openapi_info_reads_from_path() {
    let dir = std::env::temp_dir().join(format!("api-mcp-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let spec_path = dir.join("spec.yaml");
    std::fs::write(&spec_path, YAML_SPEC).unwrap();

    let resp = call_tool(
        "openapi_info",
        json!({"path": spec_path.to_string_lossy()}),
        6,
    );
    let parsed = tool_payload(&resp);
    assert_eq!(parsed["name"], "YAML Petstore");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn openapi_info_missing_source_errors() {
    let resp = call_tool("openapi_info", json!({}), 7);
    assert_eq!(resp["result"]["isError"], true);
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("source or path"));
}

#[test]
fn openapi_info_bad_path_errors() {
    let resp = call_tool("openapi_info", json!({"path": "/nonexistent/spec.yaml"}), 8);
    assert_eq!(resp["result"]["isError"], true);
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("cannot read"));
}

#[test]
fn openapi_info_invalid_spec_errors() {
    let resp = call_tool("openapi_info", json!({"source": "not a spec at all"}), 9);
    assert_eq!(resp["result"]["isError"], true);
}

#[test]
fn convert_openapi_returns_files() {
    let resp = call_tool(
        "convert_openapi",
        json!({"source": SPEC, "formats": "postman,rest-client"}),
        10,
    );
    let parsed = tool_payload(&resp);
    assert_eq!(parsed["count"], 2);
    let formats: Vec<&str> = parsed["files"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|f| f["format"].as_str())
        .collect();
    assert!(formats.contains(&"postman"));
    assert!(formats.contains(&"rest-client"));
    assert!(parsed["files"]
        .as_array()
        .unwrap()
        .iter()
        .all(|f| f["path"].is_string() && f["content"].is_string()));
}

#[test]
fn convert_openapi_defaults_to_postman() {
    let resp = call_tool("convert_openapi", json!({"source": SPEC}), 11);
    let parsed = tool_payload(&resp);
    assert_eq!(parsed["count"], 1);
    assert_eq!(parsed["files"][0]["format"], "postman");
}

#[test]
fn convert_openapi_all_formats() {
    let resp = call_tool(
        "convert_openapi",
        json!({"source": SPEC, "formats": "all"}),
        12,
    );
    let parsed = tool_payload(&resp);
    let formats: Vec<&str> = parsed["files"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|f| f["format"].as_str())
        .collect();
    for expected in [
        "postman",
        "bruno",
        "insomnia",
        "hoppscotch",
        "rest-client",
        "thunder-client",
        "yaak",
        "hurl",
        "kreya",
        "curl",
    ] {
        assert!(formats.contains(&expected), "missing {expected}");
    }
}

#[test]
fn convert_openapi_dedupes_and_trims_formats() {
    let resp = call_tool(
        "convert_openapi",
        json!({"source": SPEC, "formats": " postman , postman ,curl"}),
        13,
    );
    let parsed = tool_payload(&resp);
    assert_eq!(parsed["count"], 2);
}

#[test]
fn convert_openapi_rejects_bad_format() {
    let resp = call_tool(
        "convert_openapi",
        json!({"source": SPEC, "formats": "bogus"}),
        14,
    );
    assert_eq!(resp["result"]["isError"], true);
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("unsupported format"));
}

#[test]
fn convert_openapi_rejects_empty_formats() {
    let resp = call_tool(
        "convert_openapi",
        json!({"source": SPEC, "formats": " , ,"}),
        15,
    );
    assert_eq!(resp["result"]["isError"], true);
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("no formats"));
}

#[test]
fn convert_openapi_missing_source_errors() {
    let resp = call_tool("convert_openapi", json!({"formats": "postman"}), 16);
    assert_eq!(resp["result"]["isError"], true);
}

#[test]
fn convert_openapi_invalid_spec_errors() {
    let resp = call_tool(
        "convert_openapi",
        json!({"source": "garbage", "formats": "postman"}),
        17,
    );
    assert_eq!(resp["result"]["isError"], true);
}

#[test]
fn convert_openapi_accepts_name_and_base_url() {
    let resp = call_tool(
        "convert_openapi",
        json!({
            "source": SPEC,
            "formats": "postman",
            "name": "Custom Name",
            "base_url": "https://override.example.com"
        }),
        18,
    );
    let parsed = tool_payload(&resp);
    assert_eq!(parsed["count"], 1);
    assert!(parsed["files"][0]["content"]
        .as_str()
        .unwrap()
        .contains("Custom Name"));
}

#[test]
fn unknown_tool_errors() {
    let resp = call_tool("does_not_exist", json!({}), 19);
    assert_eq!(resp["result"]["isError"], true);
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("unknown tool"));
}

#[test]
fn tools_call_without_arguments_defaults_to_empty() {
    let resp = handle_message(&json!({
        "jsonrpc": "2.0",
        "id": 20,
        "method": "tools/call",
        "params": {"name": "openapi_info"}
    }))
    .unwrap();
    assert_eq!(resp["result"]["isError"], true);
}

#[test]
fn tools_call_without_name_errors() {
    let resp = handle_message(&json!({
        "jsonrpc": "2.0",
        "id": 21,
        "method": "tools/call",
        "params": {}
    }))
    .unwrap();
    assert_eq!(resp["result"]["isError"], true);
    let text = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("unknown tool"));
}
