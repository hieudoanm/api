use api_core::{convert_openapi, ClientFormat, ConvertOptions};

const SPEC: &str = r##"
openapi: 3.0.3
info:
  title: Docs API
  version: 2.0.0
  description: "line one </script><script>alert(1)</script>"
servers:
  - url: https://docs.example.com
paths:
  /pets/{petId}:
    get:
      tags: [pet]
      summary: Get pet
      parameters:
        - name: petId
          in: path
          required: true
          schema: {type: integer}
      responses:
        '200': {description: ok}
"##;

fn convert(format: ClientFormat) -> api_core::ConversionFile {
    let files = convert_openapi(SPEC, format, &ConvertOptions::default()).unwrap();
    assert_eq!(files.len(), 1, "{format} should emit one file");
    files.into_iter().next().unwrap()
}

#[test]
fn swagger_ui_emits_html_with_embedded_spec() {
    let file = convert(ClientFormat::SwaggerUi);
    assert_eq!(file.format, ClientFormat::SwaggerUi);
    assert!(file.path.ends_with(".swagger-ui.html"), "{}", file.path);
    assert!(file.content.contains("SwaggerUIBundle"));
    assert!(file.content.contains("swagger-ui-bundle.js"));
    assert!(file.content.contains("\"openapi\":\"3.0.3\""));
    assert!(file.content.contains("Docs API"));
}

#[test]
fn redoc_emits_html_with_embedded_spec() {
    let file = convert(ClientFormat::Redoc);
    assert!(file.path.ends_with(".redoc.html"), "{}", file.path);
    assert!(file.content.contains("Redoc.init"));
    assert!(file.content.contains("redoc.standalone.js"));
    assert!(file.content.contains("\"openapi\":\"3.0.3\""));
}

#[test]
fn scalar_emits_html_with_embedded_spec() {
    let file = convert(ClientFormat::Scalar);
    assert!(file.path.ends_with(".scalar.html"), "{}", file.path);
    assert!(file.content.contains("api-reference"));
    assert!(file.content.contains("@scalar/api-reference"));
    assert!(file.content.contains("createApiReference"));
}

#[test]
fn stoplight_emits_html_with_embedded_spec() {
    let file = convert(ClientFormat::Stoplight);
    assert!(file.path.ends_with(".stoplight.html"), "{}", file.path);
    assert!(file.content.contains("elements-api"));
    assert!(file.content.contains("@stoplight/elements"));
    assert!(file.content.contains("apiDescriptionDocument"));
}

#[test]
fn rapidoc_emits_html_with_embedded_spec() {
    let file = convert(ClientFormat::RapiDoc);
    assert!(file.path.ends_with(".rapidoc.html"), "{}", file.path);
    assert!(file.content.contains("rapi-doc"));
    assert!(file.content.contains("unpkg.com/rapidoc@"));
    assert!(file.content.contains("spec-url"));
}

#[test]
fn spec_json_escapes_script_close() {
    let file = convert(ClientFormat::SwaggerUi);
    assert!(file.content.contains("<\\/script>"));
    assert!(!file.content.contains("description\":\"line one </script>"));
}

#[test]
fn docs_formats_parse_aliases() {
    for (alias, expected) in [
        ("swagger-ui", ClientFormat::SwaggerUi),
        ("swaggerui", ClientFormat::SwaggerUi),
        ("swagger", ClientFormat::SwaggerUi),
        ("redoc", ClientFormat::Redoc),
        ("scalar", ClientFormat::Scalar),
        ("stoplight", ClientFormat::Stoplight),
        ("stoplight-elements", ClientFormat::Stoplight),
        ("elements", ClientFormat::Stoplight),
        ("rapidoc", ClientFormat::RapiDoc),
    ] {
        assert_eq!(ClientFormat::parse(alias), Some(expected), "{alias}");
    }
}

#[test]
fn docs_file_names_embed_format() {
    for format in [
        ClientFormat::SwaggerUi,
        ClientFormat::Redoc,
        ClientFormat::Scalar,
        ClientFormat::Stoplight,
        ClientFormat::RapiDoc,
    ] {
        let name = format.file_name("Acme");
        assert!(name.starts_with("Acme."), "{name}");
        assert!(name.ends_with(".html"), "{name}");
    }
}
