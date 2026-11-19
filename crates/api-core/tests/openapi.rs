use api_core::{
    convert_openapi, convert_openapi_all, parse_openapi, ApiAuth, BodyKind, ClientFormat,
    ConvertOptions, ParameterLocation,
};

fn folder<'a>(collection: &'a api_core::ApiCollection, name: &str) -> &'a api_core::ApiFolder {
    collection
        .folders
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("missing folder {name}"))
}

fn request<'a>(
    collection: &'a api_core::ApiCollection,
    folder_name: &str,
    method: &str,
) -> &'a api_core::ApiRequest {
    folder(collection, folder_name)
        .requests
        .iter()
        .find(|r| r.method == method)
        .unwrap_or_else(|| panic!("missing {method} in {folder_name}"))
}

#[test]
fn rejects_invalid_yaml() {
    let err = parse_openapi("openapi: [unclosed").unwrap_err();
    assert!(err.to_string().contains("failed to parse"));
}

#[test]
fn rejects_invalid_json() {
    let err = parse_openapi("{not json").unwrap_err();
    assert!(err.to_string().contains("failed to parse"));
}

#[test]
fn rejects_missing_openapi_version() {
    let err = parse_openapi("info:\n  title: X\npaths: {}\n").unwrap_err();
    assert!(err.to_string().contains("missing openapi version"));
}

#[test]
fn rejects_non_3x_openapi() {
    let err = parse_openapi(r##"{"openapi":"2.0","info":{"title":"X"},"paths":{}}"##).unwrap_err();
    assert!(err.to_string().contains("unsupported OpenAPI version"));
}

#[test]
fn rejects_missing_paths() {
    let err = parse_openapi(r##"{"openapi":"3.0.0","info":{"title":"X"}}"##).unwrap_err();
    assert!(err.to_string().contains("missing paths"));
}

#[test]
fn json_input_parses() {
    let collection = parse_openapi(
        r##"{"openapi":"3.0.0","info":{"title":"JSON API","version":"9.9","description":"d"},"paths":{}}"##,
    )
    .unwrap();
    assert_eq!(collection.name, "JSON API");
    assert_eq!(collection.version, "9.9");
    assert_eq!(collection.description, "d");
}

#[test]
fn defaults_for_missing_info_and_servers() {
    let collection = parse_openapi(r##"{"openapi":"3.0.0","paths":{}}"##).unwrap();
    assert_eq!(collection.name, "OpenAPI Collection");
    assert_eq!(collection.version, "1.0.0");
    assert!(collection.servers.is_empty());
    assert!(collection.folders.is_empty());
}

#[test]
fn server_variables_resolve_and_trailing_slash_trimmed() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "servers": [
                {"url": "https://{env}.example.com/{ver}/", "variables": {"env": {"default": "api"}, "ver": {"default": "v1"}}},
                {"url": ""},
                {"url": "https://backup.example.com"}
            ],
            "paths": {}
        }"##,
    )
    .unwrap();
    assert_eq!(
        collection.servers,
        vec!["https://api.example.com/v1", "https://backup.example.com"]
    );
}

#[test]
fn folder_defaults_to_default_and_merges_same_tag() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/a": {"get": {"responses": {"200": {"description": "ok"}}}},
                "/b": {"get": {"tags": ["x"], "responses": {"200": {"description": "ok"}}}},
                "/c": {"get": {"tags": ["x"], "responses": {"200": {"description": "ok"}}}}
            }
        }"##,
    )
    .unwrap();
    assert_eq!(collection.folders.len(), 2);
    assert!(collection.folders.iter().any(|f| f.name == "Default"));
    let x = folder(&collection, "x");
    assert_eq!(x.requests.len(), 2);
}

#[test]
fn request_name_falls_back_to_operation_id_then_method_path() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/a": {"get": {"operationId": "getA", "responses": {"200": {"description": "ok"}}}},
                "/b": {"post": {"responses": {"200": {"description": "ok"}}}}
            }
        }"##,
    )
    .unwrap();
    let get = collection
        .folders
        .iter()
        .flat_map(|f| f.requests.iter())
        .find(|r| r.path == "/a")
        .unwrap();
    assert_eq!(get.name, "getA");
    assert_eq!(get.id, "get-a");
    let post = collection
        .folders
        .iter()
        .flat_map(|f| f.requests.iter())
        .find(|r| r.path == "/b")
        .unwrap();
    assert_eq!(post.name, "POST /b");
}

#[test]
fn unresolvable_path_item_ref_is_skipped() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/ghost": {"$ref": "#/components/pathItems/Missing"},
                "/real": {"get": {"responses": {"200": {"description": "ok"}}}}
            }
        }"##,
    )
    .unwrap();
    let total: usize = collection.folders.iter().map(|f| f.requests.len()).sum();
    assert_eq!(total, 1);
}

#[test]
fn operation_and_path_level_servers_win_over_root() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "servers": [{"url": "https://root.example.com"}],
            "paths": {
                "/path-level": {
                    "servers": [{"url": "https://path.example.com/"}],
                    "get": {"responses": {"200": {"description": "ok"}}}
                },
                "/op-level": {
                    "get": {
                        "servers": [{"url": "https://{region}.op.example.com", "variables": {"region": {"default": "eu"}}}],
                        "responses": {"200": {"description": "ok"}}
                    }
                },
                "/root": {"get": {"responses": {"200": {"description": "ok"}}}}
            }
        }"##,
    )
    .unwrap();
    let all: Vec<_> = collection
        .folders
        .iter()
        .flat_map(|f| f.requests.iter())
        .collect();
    let by_path = |p: &str| all.iter().find(|r| r.path == p).unwrap();
    assert_eq!(
        by_path("/path-level").server.as_deref(),
        Some("https://path.example.com")
    );
    assert_eq!(
        by_path("/op-level").server.as_deref(),
        Some("https://eu.op.example.com")
    );
    assert_eq!(
        by_path("/root").server.as_deref(),
        Some("https://root.example.com")
    );
}

#[test]
fn parameters_dedupe_prefers_path_level_and_ignores_cookie() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/items/{id}": {
                    "parameters": [
                        {"name": "id", "in": "path", "required": true, "schema": {"type": "string"}, "example": "path-level"},
                        {"name": "shared", "in": "query", "schema": {"type": "string"}, "example": "from-path"}
                    ],
                    "get": {
                        "parameters": [
                            {"name": "id", "in": "path", "required": true, "schema": {"type": "string"}, "example": "op-level"},
                            {"name": "shared", "in": "query", "schema": {"type": "string"}, "example": "from-op"},
                            {"name": "session", "in": "cookie", "schema": {"type": "string"}, "example": "cookie-val"},
                            {"name": "bad", "in": "body", "schema": {"type": "string"}},
                            {"in": "query", "schema": {"type": "string"}}
                        ],
                        "responses": {"200": {"description": "ok"}}
                    }
                }
            }
        }"##,
    )
    .unwrap();
    let get = request(&collection, "Default", "GET");
    assert_eq!(get.path_params.len(), 1);
    assert_eq!(get.path_params[0].value, "path-level");
    assert!(get.path_params[0].required);
    assert_eq!(get.query_params.len(), 1);
    assert_eq!(get.query_params[0].value, "from-path");
    assert!(!get.query_params[0].required);
}

#[test]
fn parameter_required_defaults_and_examples() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/p": {
                    "get": {
                        "parameters": [
                            {"name": "q1", "in": "query", "example": "direct"},
                            {"name": "q2", "in": "query", "examples": {"a": {"value": "first"}, "b": {"value": "second"}}},
                            {"name": "q3", "in": "query", "schema": {"type": "integer", "example": 42}},
                            {"name": "q4", "in": "query", "schema": {"type": "boolean"}},
                            {"name": "q5", "in": "query", "schema": {"type": "array", "items": {"type": "string", "example": "x"}}},
                            {"name": "h1", "in": "header", "schema": {"type": "string", "format": "date"}},
                            {"name": "h2", "in": "header", "schema": {"type": "string", "format": "date-time"}},
                            {"name": "h3", "in": "header", "schema": {"type": "string", "format": "uuid"}},
                            {"name": "h4", "in": "header", "schema": {"type": "string", "format": "binary"}},
                            {"name": "h5", "in": "header", "schema": {"type": "number"}},
                            {"name": "h6", "in": "header", "schema": {"enum": ["a", "b"]}},
                            {"name": "h7", "in": "header", "schema": {"default": "dflt"}},
                            {"name": "h8", "in": "header"}
                        ],
                        "responses": {"200": {"description": "ok"}}
                    }
                }
            }
        }"##,
    )
    .unwrap();
    let get = request(&collection, "Default", "GET");
    let q = |n: &str| get.query_params.iter().find(|p| p.name == n).unwrap();
    assert_eq!(q("q1").value, "direct");
    assert_eq!(q("q2").value, "first");
    assert_eq!(q("q3").value, "42");
    assert_eq!(q("q4").value, "true");
    assert_eq!(q("q5").value, "[\"x\"]");
    let h = |n: &str| get.headers.iter().find(|p| p.name == n).unwrap();
    assert_eq!(h("h1").value, "2026-01-01");
    assert_eq!(h("h2").value, "2026-01-01T00:00:00Z");
    assert_eq!(h("h3").value, "00000000-0000-0000-0000-000000000000");
    assert_eq!(h("h4").value, "<file>");
    assert_eq!(h("h5").value, "0");
    assert_eq!(h("h6").value, "a");
    assert_eq!(h("h7").value, "dflt");
    assert_eq!(h("h8").value, "");
}

#[test]
fn parameter_value_to_string_for_complex_examples() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/p": {
                    "get": {
                        "parameters": [
                            {"name": "arr", "in": "query", "example": [1, "two"]},
                            {"name": "obj", "in": "query", "example": {"a": 1}},
                            {"name": "nul", "in": "query", "example": null}
                        ],
                        "responses": {"200": {"description": "ok"}}
                    }
                }
            }
        }"##,
    )
    .unwrap();
    let get = request(&collection, "Default", "GET");
    let q = |n: &str| get.query_params.iter().find(|p| p.name == n).unwrap();
    assert_eq!(q("arr").value, "[1,\"two\"]");
    assert_eq!(q("obj").value, "{\"a\":1}");
    assert_eq!(q("nul").value, "");
}

#[test]
fn schema_example_variants() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/p": {
                    "get": {
                        "parameters": [
                            {"name": "allof", "in": "query", "schema": {"allOf": [{"type": "object", "properties": {"a": {"type": "string", "example": "A"}}}, {"type": "object", "properties": {"b": {"type": "integer"}, "a": {"type": "string", "example": "SHADOWED"}}}]}},
                            {"name": "oneof", "in": "query", "schema": {"oneOf": [{"type": "string", "example": "first-variant"}, {"type": "integer"}]}},
                            {"name": "anyof", "in": "query", "schema": {"anyOf": [{"type": "string", "example": "any-first"}]}},
                            {"name": "nullable-type", "in": "query", "schema": {"type": ["null", "string"], "example": "typed"}},
                            {"name": "nulltype", "in": "query", "schema": {"type": "null"}},
                            {"name": "weird", "in": "query", "schema": {"type": "uuid-kind"}}
                        ],
                        "responses": {"200": {"description": "ok"}}
                    }
                }
            }
        }"##,
    )
    .unwrap();
    let get = request(&collection, "Default", "GET");
    let q = |n: &str| get.query_params.iter().find(|p| p.name == n).unwrap();
    assert_eq!(q("allof").value, "{\"a\":\"A\",\"b\":0}");
    assert_eq!(q("oneof").value, "first-variant");
    assert_eq!(q("anyof").value, "any-first");
    assert_eq!(q("nullable-type").value, "typed");
    assert_eq!(q("nulltype").value, "");
    assert_eq!(q("weird").value, "");
}

#[test]
fn body_kind_selection_prefers_json_then_form_then_text() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/json": {"post": {"requestBody": {"content": {"application/xml": {"schema": {"type": "object"}}, "application/json": {"schema": {"type": "object", "properties": {"a": {"type": "string", "example": "A"}}}}}}, "responses": {"200": {"description": "ok"}}}},
                "/suffix": {"post": {"requestBody": {"content": {"application/vnd.api+json": {"example": {"ok": true}}}}, "responses": {"200": {"description": "ok"}}}},
                "/problem": {"post": {"requestBody": {"content": {"application/problem+json": {"example": {"detail": "bad"}}}}, "responses": {"200": {"description": "ok"}}}},
                "/form": {"post": {"requestBody": {"content": {"application/x-www-form-urlencoded": {"schema": {"type": "object", "required": ["a"], "properties": {"a": {"type": "string", "example": "va"}, "b": {"type": "integer", "default": 5}}}}}}, "responses": {"200": {"description": "ok"}}}},
                "/text": {"post": {"requestBody": {"content": {"text/plain": {"example": "hello"}}}, "responses": {"200": {"description": "ok"}}}},
                "/other": {"post": {"requestBody": {"content": {"application/octet-stream": {"example": "bin"}}}, "responses": {"200": {"description": "ok"}}}},
                "/empty": {"post": {"requestBody": {"content": {}}, "responses": {"200": {"description": "ok"}}}},
                "/nobody": {"post": {"responses": {"200": {"description": "ok"}}}}
            }
        }"##,
    )
    .unwrap();

    let body = |path: &str| {
        collection
            .folders
            .iter()
            .flat_map(|f| f.requests.iter())
            .find(|r| r.path == path)
            .and_then(|r| r.body.as_ref())
    };

    let json = body("/json").unwrap();
    assert_eq!(json.kind, BodyKind::Json);
    assert_eq!(json.content_type, "application/json");
    assert!(json.raw.as_deref().unwrap().contains("\"a\": \"A\""));

    let suffix = body("/suffix").unwrap();
    assert_eq!(suffix.kind, BodyKind::Json);
    assert_eq!(suffix.content_type, "application/vnd.api+json");

    let problem = body("/problem").unwrap();
    assert_eq!(problem.kind, BodyKind::Json);
    assert_eq!(problem.content_type, "application/problem+json");

    let form = body("/form").unwrap();
    assert_eq!(form.kind, BodyKind::UrlEncoded);
    assert_eq!(form.fields.len(), 2);
    let a = form.fields.iter().find(|f| f.name == "a").unwrap();
    assert_eq!(a.value, "va");
    assert!(a.required);
    let b = form.fields.iter().find(|f| f.name == "b").unwrap();
    assert_eq!(b.value, "5");
    assert!(!b.required);

    let text = body("/text").unwrap();
    assert_eq!(text.kind, BodyKind::Text);
    assert_eq!(text.raw.as_deref(), Some("hello"));

    let other = body("/other").unwrap();
    assert_eq!(other.kind, BodyKind::Other);
    assert_eq!(other.content_type, "application/octet-stream");
    assert_eq!(other.raw.as_deref(), Some("bin"));

    assert!(body("/empty").is_none());
    assert!(body("/nobody").is_none());
}

#[test]
fn json_body_raw_variants() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/str-json": {"post": {"requestBody": {"content": {"application/json": {"example": "{\"already\": \"json\"}"}}}, "responses": {"200": {"description": "ok"}}}},
                "/str-plain": {"post": {"requestBody": {"content": {"application/json": {"example": "plain text"}}}, "responses": {"200": {"description": "ok"}}}},
                "/null": {"post": {"requestBody": {"content": {"application/json": {"example": null}}}, "responses": {"200": {"description": "ok"}}}},
                "/empty": {"post": {"requestBody": {"content": {"application/json": {}}}, "responses": {"200": {"description": "ok"}}}},
                "/examples": {"post": {"requestBody": {"content": {"application/json": {"examples": {"one": {"value": {"n": 1}}}}}}, "responses": {"200": {"description": "ok"}}}}
            }
        }"##,
    )
    .unwrap();
    let raw = |path: &str| {
        collection
            .folders
            .iter()
            .flat_map(|f| f.requests.iter())
            .find(|r| r.path == path)
            .and_then(|r| r.body.as_ref())
            .and_then(|b| b.raw.clone())
            .unwrap()
    };
    assert_eq!(raw("/str-json"), "\"{\\\"already\\\": \\\"json\\\"}\"");
    assert_eq!(raw("/str-plain"), "plain text");
    assert_eq!(raw("/null"), "{}");
    assert_eq!(raw("/empty"), "{}");
    assert!(raw("/examples").contains("\"n\": 1"));
}

#[test]
fn form_fields_resolve_refs_and_binary() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/upload": {
                    "post": {
                        "requestBody": {
                            "content": {
                                "multipart/form-data": {
                                    "schema": {"$ref": "#/components/schemas/Upload"}
                                }
                            }
                        },
                        "responses": {"200": {"description": "ok"}}
                    }
                }
            },
            "components": {
                "schemas": {
                    "Upload": {
                        "type": "object",
                        "required": ["file"],
                        "properties": {
                            "file": {"type": "string", "format": "binary"},
                            "note": {"type": "string", "default": "hi"},
                            "kind": {"type": "string", "enum": ["a", "b"]}
                        }
                    }
                }
            }
        }"##,
    )
    .unwrap();
    let body = request(&collection, "Default", "POST")
        .body
        .as_ref()
        .unwrap();
    assert_eq!(body.kind, BodyKind::FormData);
    let f = |n: &str| body.fields.iter().find(|p| p.name == n).unwrap();
    assert_eq!(f("file").value, "<file>");
    assert!(f("file").required);
    assert_eq!(f("note").value, "hi");
    assert_eq!(f("kind").value, "a");
}

#[test]
fn auth_variants() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "security": [{"rootAuth": []}],
            "paths": {
                "/basic": {"get": {"security": [{"basicAuth": []}], "responses": {"200": {"description": "ok"}}}},
                "/bearer": {"get": {"security": [{"bearerAuth": []}], "responses": {"200": {"description": "ok"}}}},
                "/key-q": {"get": {"security": [{"keyQuery": []}], "responses": {"200": {"description": "ok"}}}},
                "/key-c": {"get": {"security": [{"keyCookie": []}], "responses": {"200": {"description": "ok"}}}},
                "/key-h": {"get": {"security": [{"keyHeader": []}], "responses": {"200": {"description": "ok"}}}},
                "/oauth": {"get": {"security": [{"oauth": ["read"]}], "responses": {"200": {"description": "ok"}}}},
                "/oidc": {"get": {"security": [{"oidc": []}], "responses": {"200": {"description": "ok"}}}},
                "/digest": {"get": {"security": [{"digestAuth": []}], "responses": {"200": {"description": "ok"}}}},
                "/mutual": {"get": {"security": [{"mutualTLS": []}], "responses": {"200": {"description": "ok"}}}},
                "/unknown-scheme": {"get": {"security": [{"missingScheme": []}], "responses": {"200": {"description": "ok"}}}},
                "/public": {"get": {"security": [], "responses": {"200": {"description": "ok"}}}},
                "/root": {"get": {"responses": {"200": {"description": "ok"}}}}
            },
            "components": {
                "securitySchemes": {
                    "rootAuth": {"type": "http", "scheme": "bearer"},
                    "basicAuth": {"type": "http", "scheme": "Basic"},
                    "bearerAuth": {"type": "http", "scheme": "bearer"},
                    "keyQuery": {"type": "apiKey", "name": "api_key", "in": "query"},
                    "keyCookie": {"type": "apiKey", "name": "session", "in": "cookie"},
                    "keyHeader": {"type": "apiKey", "name": "X-Key", "in": "header"},
                    "keyDefault": {"type": "apiKey"},
                    "oauth": {"type": "oauth2", "flows": {"clientCredentials": {"tokenUrl": "https://auth.example.com/token", "scopes": {"read": "r"}}}},
                    "oidc": {"type": "openIdConnect", "openIdConnectUrl": "https://auth.example.com/.well-known"},
                    "digestAuth": {"type": "http", "scheme": "digest"},
                    "mutualTLS": {"type": "mutualTLS"}
                }
            }
        }"##,
    )
    .unwrap();
    let auth = |path: &str| {
        collection
            .folders
            .iter()
            .flat_map(|f| f.requests.iter())
            .find(|r| r.path == path)
            .unwrap()
            .auth
            .first()
            .cloned()
    };

    assert!(matches!(auth("/basic"), Some(ApiAuth::Basic { .. })));
    assert!(matches!(auth("/bearer"), Some(ApiAuth::Bearer { .. })));
    assert!(matches!(
        auth("/key-q"),
        Some(ApiAuth::ApiKey { location: ParameterLocation::Query, name, .. }) if name == "api_key"
    ));
    assert!(matches!(
        auth("/key-c"),
        Some(ApiAuth::ApiKey {
            location: ParameterLocation::Cookie,
            ..
        })
    ));
    assert!(matches!(
        auth("/key-h"),
        Some(ApiAuth::ApiKey { location: ParameterLocation::Header, name, .. }) if name == "X-Key"
    ));
    assert!(matches!(auth("/oauth"), Some(ApiAuth::OAuth2 { .. })));
    assert!(matches!(auth("/oidc"), Some(ApiAuth::OAuth2 { .. })));
    assert!(matches!(
        auth("/digest"),
        Some(ApiAuth::Other { kind, name }) if kind == "http" && name == "digest"
    ));
    assert!(matches!(
        auth("/mutual"),
        Some(ApiAuth::Other { kind, name }) if kind == "mutualTLS" && name == "mutualTLS"
    ));
    assert!(auth("/unknown-scheme").is_none());
    assert!(auth("/public").is_none());
    assert!(matches!(auth("/root"), Some(ApiAuth::Bearer { .. })));
}

#[test]
fn auth_default_apikey_location_and_ref_scheme() {
    let collection = parse_openapi(
        r##"{
            "openapi": "3.0.0",
            "info": {"title": "T"},
            "paths": {
                "/default-key": {"get": {"security": [{"keyDefault": []}], "responses": {"200": {"description": "ok"}}}},
                "/ref-scheme": {"get": {"security": [{"alias": []}], "responses": {"200": {"description": "ok"}}}}
            },
            "components": {
                "securitySchemes": {
                    "keyDefault": {"type": "apiKey"},
                    "real": {"type": "http", "scheme": "bearer"},
                    "alias": {"$ref": "#/components/securitySchemes/real"}
                }
            }
        }"##,
    )
    .unwrap();
    let auth = |path: &str| {
        collection
            .folders
            .iter()
            .flat_map(|f| f.requests.iter())
            .find(|r| r.path == path)
            .unwrap()
            .auth
            .first()
            .cloned()
    };
    assert!(matches!(
        auth("/default-key"),
        Some(ApiAuth::ApiKey { location: ParameterLocation::Header, name, .. }) if name == "apiKey"
    ));
    assert!(matches!(auth("/ref-scheme"), Some(ApiAuth::Bearer { .. })));
}

#[test]
fn convert_openapi_all_returns_files_for_every_format() {
    let files = convert_openapi_all(
        r##"{"openapi":"3.0.0","info":{"title":"All"},"paths":{"/x":{"get":{"responses":{"200":{"description":"ok"}}}}}}"##,
        &ConvertOptions::default(),
    )
    .unwrap();
    let formats: Vec<_> = files.iter().map(|f| f.format).collect();
    for format in ClientFormat::ALL {
        assert!(formats.contains(&format), "missing {format}");
    }
}

#[test]
fn convert_openapi_propagates_parse_errors() {
    assert!(convert_openapi("garbage", ClientFormat::Postman, &ConvertOptions::default()).is_err());
}

#[test]
fn empty_server_urls_are_filtered() {
    let collection = parse_openapi(
        r##"{"openapi":"3.0.0","info":{"title":"S"},"servers":[{"url":"/"},{"url":"https://ok.example.com/"}],"paths":{}}"##,
    )
    .unwrap();
    assert_eq!(collection.servers, vec!["https://ok.example.com"]);
}

#[test]
fn operation_server_empty_url_falls_back() {
    let collection = parse_openapi(
        r##"{"openapi":"3.0.0","info":{"title":"S"},"servers":[{"url":"https://root.example.com"}],"paths":{"/x":{"get":{"servers":[{"url":"/"}],"responses":{"200":{"description":"ok"}}}}}}"##,
    )
    .unwrap();
    let request = &collection.folders[0].requests[0];
    assert_eq!(request.server.as_deref(), Some("https://root.example.com"));
}
