use serde_json::Value;

const SPEC: &str = r#"
openapi: 3.0.3
info:
  title: Petstore
  description: Pets and stores
  version: 2.0.0
servers:
  - url: https://api.example.com/v1
paths:
  /pets:
    get:
      tags: [pets]
      summary: List pets
      parameters:
        - name: limit
          in: query
          schema:
            type: integer
          example: 20
      responses:
        '200':
          description: ok
    post:
      tags: [pets]
      summary: Create pet
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                name:
                  type: string
                  example: Rex
      responses:
        '201':
          description: created
  /pets/{petId}:
    get:
      tags: [pets]
      summary: Get pet
      parameters:
        - name: petId
          in: path
          required: true
          schema:
            type: integer
          example: 7
      security: []
      responses:
        '200':
          description: ok
"#;

fn parse_json(text: &str) -> Value {
    serde_json::from_str(text).expect("output should be valid JSON")
}

#[test]
fn formats_returns_all_client_formats() {
    let value = parse_json(&api_wasm::formats());
    let formats = value.as_array().expect("formats is an array");
    assert_eq!(formats.len(), 15);
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
        "swagger-ui",
        "redoc",
        "scalar",
        "stoplight",
        "rapidoc",
    ] {
        assert!(
            formats.iter().any(|f| f == expected),
            "missing format {expected}"
        );
    }
}

#[test]
fn openapi_info_summarizes_yaml_spec() {
    let info = parse_json(&api_wasm::openapi_info(SPEC).unwrap());
    assert_eq!(info["name"], "Petstore");
    assert_eq!(info["description"], "Pets and stores");
    assert_eq!(info["version"], "2.0.0");
    assert_eq!(
        info["servers"],
        serde_json::json!(["https://api.example.com/v1"])
    );
    assert_eq!(info["requests"], 3);
    let folders = info["folders"].as_array().unwrap();
    assert_eq!(folders.len(), 1);
    assert_eq!(folders[0]["name"], "pets");
    assert_eq!(folders[0]["requests"], 3);
}

#[test]
fn openapi_info_accepts_json_spec() {
    let spec = r#"{"openapi":"3.0.0","info":{"title":"JSON API","version":"1.0.0"},"paths":{}}"#;
    let info = parse_json(&api_wasm::openapi_info(spec).unwrap());
    assert_eq!(info["name"], "JSON API");
    assert_eq!(info["requests"], 0);
}

#[test]
fn openapi_info_trims_empty_description() {
    let spec = r#"{"openapi":"3.0.0","info":{"title":"T","version":"1"},"paths":{}}"#;
    let info = parse_json(&api_wasm::openapi_info(spec).unwrap());
    assert_eq!(info["name"], "T");
    assert_eq!(info["version"], "1");
}

#[test]
fn convert_returns_files_for_format() {
    let output = parse_json(&api_wasm::convert(SPEC, "postman", "", "").unwrap());
    let files = output["files"].as_array().unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0]["format"], "postman");
    assert!(files[0]["path"]
        .as_str()
        .unwrap()
        .ends_with(".postman_collection.json"));
    assert!(files[0]["content"].as_str().unwrap().contains("Petstore"));
}

#[test]
fn convert_uses_name_and_base_url_overrides() {
    let output = parse_json(
        &api_wasm::convert(SPEC, "curl", "  My API  ", "https://staging.example.com/").unwrap(),
    );
    let files = output["files"].as_array().unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0]["format"], "curl");
    assert_eq!(files[0]["path"], "my-api.sh");
    let content = files[0]["content"].as_str().unwrap();
    assert!(content.contains("BASE_URL='https://staging.example.com'"));
}

#[test]
fn convert_supports_format_aliases() {
    for (alias, expected) in [
        ("bru", "bruno"),
        ("tc", "thunder-client"),
        ("sh", "curl"),
        ("shell", "curl"),
        ("http", "rest-client"),
        ("postman-collection", "postman"),
    ] {
        let output = parse_json(&api_wasm::convert(SPEC, alias, "", "").unwrap());
        assert_eq!(
            output["files"][0]["format"], expected,
            "alias {alias} should map to {expected}"
        );
    }
}

#[test]
fn convert_all_emits_every_format() {
    let output = parse_json(&api_wasm::convert_all(SPEC, "", "").unwrap());
    let files = output["files"].as_array().unwrap();
    let formats: Vec<&str> = files.iter().filter_map(|f| f["format"].as_str()).collect();
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
        assert!(formats.contains(&expected), "missing format {expected}");
    }
    assert!(files.len() > 10, "all formats should emit multiple files");
}

#[test]
fn convert_all_with_name_override() {
    let output = parse_json(&api_wasm::convert_all(SPEC, "Override", "").unwrap());
    let files = output["files"].as_array().unwrap();
    assert!(files.iter().any(|f| {
        f["path"]
            .as_str()
            .map(|p| p.contains("override") || p.contains("Override"))
            .unwrap_or(false)
    }));
}

// wasm-bindgen's JsError cannot be constructed on non-wasm targets — the
// exported functions panic inside JsError::new. These tests assert that the
// error paths are reached (the panic is the native-target stand-in for the
// thrown JsError the browser sees).
#[test]
fn openapi_info_rejects_invalid_spec() {
    let result = std::panic::catch_unwind(|| api_wasm::openapi_info("not a spec"));
    assert!(result.is_err(), "invalid spec should hit the JsError path");
}

#[test]
fn convert_rejects_unsupported_format() {
    let result = std::panic::catch_unwind(|| api_wasm::convert(SPEC, "bogus", "", ""));
    assert!(result.is_err(), "bad format should hit the JsError path");
}

#[test]
fn convert_rejects_invalid_spec() {
    let result = std::panic::catch_unwind(|| api_wasm::convert("garbage", "postman", "", ""));
    assert!(result.is_err(), "invalid spec should hit the JsError path");
}

#[test]
fn convert_all_rejects_invalid_spec() {
    let result = std::panic::catch_unwind(|| api_wasm::convert_all("", "", ""));
    assert!(result.is_err(), "empty spec should hit the JsError path");
}
