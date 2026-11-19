use api_core::{convert_openapi, ClientFormat, ConvertOptions};
use serde_json::{json, Value};

const EDGE_SPEC: &str = r#"
openapi: 3.0.3
info:
  title: Edge API
  version: 1.0.0
servers:
  - url: https://edge.example.com
security:
  - bearerAuth: []
paths:
  /auth/basic:
    get:
      tags: [auth]
      summary: Basic auth op
      security:
        - basicAuth: []
      responses:
        '200': {description: ok}
  /auth/bearer:
    get:
      tags: [auth]
      summary: Bearer auth op
      security:
        - bearerAuth: []
      responses:
        '200': {description: ok}
  /auth/key-query:
    get:
      tags: [auth]
      summary: API key in query
      security:
        - keyQuery: []
      responses:
        '200': {description: ok}
  /auth/key-header:
    get:
      tags: [auth]
      summary: API key in header
      security:
        - keyHeader: []
      responses:
        '200': {description: ok}
  /auth/key-cookie:
    get:
      tags: [auth]
      summary: API key in cookie
      security:
        - keyCookie: []
      responses:
        '200': {description: ok}
  /auth/oauth:
    get:
      tags: [auth]
      summary: OAuth2 op
      security:
        - oauth: [read]
      responses:
        '200': {description: ok}
  /auth/other:
    get:
      tags: [auth]
      summary: Mutual TLS op
      security:
        - mutualTLS: []
      responses:
        '200': {description: ok}
  /auth/none:
    get:
      tags: [auth]
      summary: Public op
      security: []
      responses:
        '200': {description: ok}
  /body/form:
    post:
      tags: [bodies]
      summary: Urlencoded body
      requestBody:
        content:
          application/x-www-form-urlencoded:
            schema:
              type: object
              required: [a]
              properties:
                a:
                  type: string
                  example: va
                b:
                  type: integer
                  default: 5
      responses:
        '200': {description: ok}
  /body/text:
    post:
      tags: [bodies]
      summary: Text body
      requestBody:
        content:
          text/plain:
            example: hello world
      responses:
        '200': {description: ok}
  /body/other:
    post:
      tags: [bodies]
      summary: Binary body
      requestBody:
        content:
          application/octet-stream:
            example: bin-data
      responses:
        '200': {description: ok}
  /body/formdata:
    post:
      tags: [bodies]
      summary: Multipart body
      requestBody:
        content:
          multipart/form-data:
            schema:
              type: object
              required: [file]
              properties:
                file:
                  type: string
                  format: binary
                note:
                  type: string
                  example: hi
      responses:
        '200': {description: ok}
  /body/json:
    post:
      tags: [bodies]
      summary: JSON schema body
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                note: {type: string}
      responses:
        '200': {description: ok}
  /body/jsonexamples:
    post:
      tags: [bodies]
      summary: JSON examples body
      requestBody:
        content:
          application/json:
            examples:
              main: {value: {a: 1, b: two}}
      responses:
        '200': {description: ok}
  /body/jsonnull:
    post:
      tags: [bodies]
      summary: Null JSON example
      requestBody:
        content:
          application/json:
            example: null
      responses:
        '200': {description: ok}
  /body/customjson:
    post:
      tags: [bodies]
      summary: Vendor JSON body
      requestBody:
        content:
          application/vnd.api+json:
            example: {ok: true}
      responses:
        '200': {description: ok}
  /body/xml:
    post:
      tags: [bodies]
      summary: XML body
      requestBody:
        content:
          application/xml:
            example: {a: 1}
      responses:
        '200': {description: ok}
  /things/{id}/parts/{part}:
    parameters:
      - name: id
        in: path
        required: true
        schema: {type: integer}
      - name: X-Shared
        in: header
        schema: {type: string, example: shared}
    post:
      summary: Untagged params op
      parameters:
        - name: part
          in: path
          required: true
          schema: {type: string}
        - name: verbose
          in: query
          schema: {type: boolean, default: false}
        - name: X-Trace
          in: header
          required: true
          examples:
            main: {value: t-2}
          schema: {type: string}
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                note: {type: string}
      responses:
        '200': {description: ok}
components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
    basicAuth:
      type: http
      scheme: basic
    keyQuery:
      type: apiKey
      name: api_key
      in: query
    keyHeader:
      type: apiKey
      name: X-API-Key
      in: header
    keyCookie:
      type: apiKey
      name: session
      in: cookie
    oauth:
      type: oauth2
      flows:
        clientCredentials:
          tokenUrl: https://auth.example.com/token
          scopes:
            read: read access
    mutualTLS:
      type: mutualTLS
"#;

fn convert(format: ClientFormat) -> Vec<api_core::ConversionFile> {
    convert_openapi(EDGE_SPEC, format, &ConvertOptions::default()).unwrap()
}

fn postman_doc() -> Value {
    let files = convert(ClientFormat::Postman);
    assert_eq!(files.len(), 1);
    serde_json::from_str(&files[0].content).unwrap()
}

fn slug(fragment: &str) -> String {
    fragment
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn find_request<'a>(doc: &'a Value, summary_fragment: &str) -> &'a Value {
    let needle = slug(summary_fragment);
    doc["item"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|folder| folder["item"].as_array().unwrap().iter())
        .find(|item| {
            item["name"]
                .as_str()
                .unwrap()
                .to_lowercase()
                .contains(&needle)
        })
        .unwrap_or_else(|| panic!("missing request containing {summary_fragment}"))
}

#[test]
fn postman_covers_auth_and_body_variants() {
    let doc = postman_doc();

    let basic = find_request(&doc, "Basic auth op");
    assert_eq!(basic["request"]["auth"]["type"], "basic");

    let key_query = find_request(&doc, "API key in query");
    assert_eq!(key_query["request"]["auth"]["type"], "apikey");
    let apikey = key_query["request"]["auth"]["apikey"].as_array().unwrap();
    assert!(apikey
        .iter()
        .any(|kv| kv["key"] == "in" && kv["value"] == "query"));

    let key_header = find_request(&doc, "API key in header");
    let headers = key_header["request"]["header"].as_array().unwrap();
    assert!(headers
        .iter()
        .any(|h| h["key"] == "X-API-Key" && h["value"] == "{{apiKey}}"));

    let oauth = find_request(&doc, "OAuth2 op");
    assert_eq!(oauth["request"]["auth"]["type"], "bearer");

    let other = find_request(&doc, "Mutual TLS op");
    assert!(other["request"].get("auth").is_none());

    let public = find_request(&doc, "Public op");
    assert!(public["request"].get("auth").is_none());

    let form = find_request(&doc, "Urlencoded body");
    assert_eq!(form["request"]["body"]["mode"], "urlencoded");

    let text = find_request(&doc, "Text body");
    assert_eq!(text["request"]["body"]["mode"], "raw");
    assert_eq!(
        text["request"]["body"]["options"]["raw"]["language"],
        "text"
    );
    assert_eq!(text["request"]["body"]["raw"], "hello world");

    let binary = find_request(&doc, "Binary body");
    assert_eq!(binary["request"]["body"]["raw"], "bin-data");

    let multipart = find_request(&doc, "Multipart body");
    assert_eq!(multipart["request"]["body"]["mode"], "formdata");
    let formdata = multipart["request"]["body"]["formdata"].as_array().unwrap();
    assert!(formdata
        .iter()
        .any(|f| f["key"] == "file" && f["type"] == "file"));
    assert!(formdata
        .iter()
        .any(|f| f["key"] == "note" && f["value"] == "hi"));
}

#[test]
fn bruno_covers_auth_and_body_variants() {
    let files = convert(ClientFormat::Bruno);
    let get = |fragment: &str| {
        files
            .iter()
            .find(|f| f.path.ends_with(".bru") && f.content.contains(fragment))
            .map(|f| f.content.as_str())
            .unwrap_or_else(|| panic!("missing bru for {fragment}"))
    };

    let basic = get("Basic auth op");
    assert!(basic.contains("auth: basic"));
    assert!(basic.contains("auth:basic {"));
    assert!(basic.contains("username: {{username}}"));

    let key_query = get("API key in query");
    assert!(key_query.contains("auth:apikey"));
    assert!(key_query.contains("placement: query"));
    assert!(key_query.contains("api_key: {{apiKey}}"));

    let key_header = get("API key in header");
    assert!(key_header.contains("placement: header"));

    let oauth = get("OAuth2 op");
    assert!(oauth.contains("auth:bearer"));

    let other = get("Mutual TLS op");
    assert!(other.contains("auth: none"));

    let public = get("Public op");
    assert!(public.contains("auth: none"));

    let form = get("Urlencoded body");
    assert!(form.contains("body:form-urlencoded"));
    assert!(form.contains("a: va"));
    assert!(form.contains("~b: 5"));

    let text = get("Text body");
    assert!(text.contains("body:text"));
    assert!(text.contains("hello world"));

    let binary = get("Binary body");
    assert!(binary.contains("body:text"));
    assert!(binary.contains("bin-data"));

    let multipart = get("Multipart body");
    assert!(multipart.contains("body:multipart-form"));
    assert!(multipart.contains("file: <file>"));
}

#[test]
fn insomnia_covers_auth_and_body_variants() {
    let files = convert(ClientFormat::Insomnia);
    assert_eq!(files.len(), 1);
    let doc: Value = serde_json::from_str(&files[0].content).unwrap();
    let requests = doc["resources"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["_type"] == "request")
        .collect::<Vec<_>>();
    assert_eq!(requests.len(), 18);
    let find = |fragment: &str| {
        requests
            .iter()
            .find(|r| r["name"].as_str().unwrap().contains(fragment))
            .unwrap_or_else(|| panic!("missing {fragment}"))
    };

    assert_eq!(find("Basic auth op")["authentication"]["type"], "basic");
    assert_eq!(
        find("API key in query")["authentication"]["addTo"],
        "queryParams"
    );
    assert_eq!(
        find("API key in header")["authentication"]["addTo"],
        "header"
    );
    assert_eq!(
        find("API key in cookie")["authentication"]["addTo"],
        "cookie"
    );
    assert_eq!(find("OAuth2 op")["authentication"]["type"], "bearer");
    assert_eq!(find("Mutual TLS op")["authentication"], json!({}));
    assert_eq!(find("Public op")["authentication"], json!({}));

    let form = find("Urlencoded body");
    assert_eq!(
        form["body"]["mimeType"],
        "application/x-www-form-urlencoded"
    );
    assert!(form["body"]["params"].as_array().unwrap().len() == 2);

    let text = find("Text body");
    assert_eq!(text["body"]["mimeType"], "text/plain");
    assert_eq!(text["body"]["text"], "hello world");

    let multipart = find("Multipart body");
    let params = multipart["body"]["params"].as_array().unwrap();
    assert!(params
        .iter()
        .any(|p| p["name"] == "file" && p["type"] == "file"));
}

#[test]
fn hoppscotch_covers_auth_and_body_variants() {
    let files = convert(ClientFormat::Hoppscotch);
    assert_eq!(files.len(), 1);
    let doc: Value = serde_json::from_str(&files[0].content).unwrap();
    let requests = doc["folders"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|f| f["requests"].as_array().unwrap().iter())
        .collect::<Vec<_>>();
    assert_eq!(requests.len(), 18);
    let find = |fragment: &str| {
        requests
            .iter()
            .find(|r| r["name"].as_str().unwrap().contains(fragment))
            .unwrap_or_else(|| panic!("missing {fragment}"))
    };

    assert_eq!(find("Basic auth op")["auth"]["authType"], "basic");
    let kq = find("API key in query");
    assert_eq!(kq["auth"]["authType"], "api-key");
    assert_eq!(kq["auth"]["addTo"], "QueryParams");
    let kc = find("API key in cookie");
    assert_eq!(kc["auth"]["addTo"], "Cookie");
    let kh = find("API key in header");
    assert_eq!(kh["auth"]["addTo"], "Headers");
    assert_eq!(find("OAuth2 op")["auth"]["authType"], "bearer");
    assert_eq!(find("Mutual TLS op")["auth"]["authType"], "none");
    assert_eq!(find("Public op")["auth"]["authType"], "none");

    let form = find("Urlencoded body");
    assert_eq!(
        form["body"]["contentType"],
        "application/x-www-form-urlencoded"
    );
    assert_eq!(form["body"]["body"], "a=va&b=5");

    let text = find("Text body");
    assert_eq!(text["body"]["contentType"], "text/plain");
    assert_eq!(text["body"]["body"], "hello world");

    let binary = find("Binary body");
    assert_eq!(binary["body"]["contentType"], "text/plain");

    let multipart = find("Multipart body");
    assert_eq!(multipart["body"]["contentType"], "multipart/form-data");
    let body = multipart["body"]["body"].as_array().unwrap();
    assert!(body
        .iter()
        .any(|f| f["key"] == "file" && f["isFile"] == true));
    assert!(body
        .iter()
        .any(|f| f["key"] == "note" && f["value"] == "hi"));
}

#[test]
fn rest_client_covers_auth_and_body_variants() {
    let files = convert(ClientFormat::RestClient);
    assert_eq!(files.len(), 1);
    let content = &files[0].content;

    assert!(content.contains("@accessToken = accessToken"));
    assert!(content.contains("@username = username"));
    assert!(content.contains("@password = password"));
    assert!(content.contains("@api_key = apiKey"));
    assert!(content.contains("@X-API-Key = apiKey"));
    assert!(content.contains("@session = apiKey"));

    assert!(content.contains("Authorization: Basic {{username}}:{{password}}"));
    assert!(content.contains("Authorization: Bearer {{accessToken}}"));
    assert!(content.contains("X-API-Key: {{apiKey}}"));
    assert!(content.contains("session={{apiKey}}"));
    assert!(content.contains("api_key={{apiKey}}"));

    assert!(content.contains("a=va&b=5"));
    assert!(content.contains("file=@file(path/to/file)"));
    assert!(content.contains("note=hi"));
    assert!(content.contains("hello world"));
    assert!(content.contains("bin-data"));
}

#[test]
fn thunder_client_covers_auth_and_body_variants() {
    let files = convert(ClientFormat::ThunderClient);
    assert_eq!(files.len(), 1);
    let doc: Value = serde_json::from_str(&files[0].content).unwrap();
    let requests = doc["requests"].as_array().unwrap();
    assert_eq!(requests.len(), 18);
    let find = |fragment: &str| {
        requests
            .iter()
            .find(|r| r["name"].as_str().unwrap().contains(fragment))
            .unwrap_or_else(|| panic!("missing {fragment}"))
    };

    let basic = find("Basic auth op");
    assert!(basic["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|h| h["name"] == "Authorization" && h["value"] == "Basic {{username}}:{{password}}"));

    let kq = find("API key in query");
    assert!(kq["params"]
        .as_array()
        .unwrap()
        .iter()
        .any(|p| p["name"] == "api_key" && p["isPath"] == false));

    let kh = find("API key in header");
    assert!(kh["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|h| h["name"] == "X-API-Key" && h["value"] == "{{apiKey}}"));

    let kc = find("API key in cookie");
    assert!(kc["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|h| h["name"] == "Cookie" && h["value"] == "session={{apiKey}}"));

    let oauth = find("OAuth2 op");
    assert!(oauth["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|h| h["name"] == "Authorization" && h["value"] == "Bearer {{accessToken}}"));

    let form = find("Urlencoded body");
    assert_eq!(form["body"]["type"], "formUrlEncoded");
    assert!(form["body"]["form"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["key"] == "a" && f["value"] == "va"));

    let text = find("Text body");
    assert_eq!(text["body"]["type"], "text");
    assert_eq!(text["body"]["raw"], "hello world");

    let multipart = find("Multipart body");
    assert_eq!(multipart["body"]["type"], "formData");
    let form = multipart["body"]["form"].as_array().unwrap();
    assert!(form
        .iter()
        .any(|f| f["key"] == "file" && f["type"] == "file" && f["value"] == "file"));
    assert!(form
        .iter()
        .any(|f| f["key"] == "note" && f["type"] == "text"));
}

#[test]
fn yaak_covers_auth_and_body_variants() {
    let files = convert(ClientFormat::Yaak);
    let request_file = |fragment: &str| {
        files
            .iter()
            .filter(|f| f.path.starts_with("request-") && f.path.ends_with(".yaml"))
            .map(|f| serde_yml::from_str::<Value>(&f.content).unwrap())
            .find(|value| value["name"].as_str().unwrap().contains(fragment))
            .unwrap_or_else(|| panic!("missing yaak request {fragment}"))
    };

    let kq = request_file("API key in query");
    assert!(kq["urlParameters"]
        .as_array()
        .unwrap()
        .iter()
        .any(|p| p["name"] == "api_key"));

    let kh = request_file("API key in header");
    assert!(kh["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|h| h["name"] == "X-API-Key" && h["value"] == "{{apiKey}}"));

    let kc = request_file("API key in cookie");
    assert!(kc["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|h| h["name"] == "Cookie" && h["value"] == "session={{apiKey}}"));

    let basic = request_file("Basic auth op");
    assert!(basic["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|h| h["value"] == "Basic {{username}}:{{password}}"));

    let form = request_file("Urlencoded body");
    assert_eq!(form["body"]["text"], "a=va&b=5");
    assert_eq!(form["bodyType"], "application/x-www-form-urlencoded");

    let multipart = request_file("Multipart body");
    assert_eq!(multipart["bodyType"], "multipart/form-data");
    assert_eq!(multipart["body"]["text"], "file=<file>\nnote=hi");

    let text = request_file("Text body");
    assert_eq!(text["body"]["text"], "hello world");
}

#[test]
fn hurl_covers_auth_and_body_variants() {
    let files = convert(ClientFormat::Hurl);
    let content_of = |fragment: &str| {
        files
            .iter()
            .find(|f| f.content.contains(fragment))
            .map(|f| f.content.as_str())
            .unwrap_or_else(|| panic!("missing hurl file for {fragment}"))
            .to_string()
    };

    let basic = content_of("Basic auth op");
    assert!(basic.contains("Authorization: Basic {{username}}:{{password}}"));

    let kq = content_of("API key in query");
    assert!(kq.contains("[QueryStringParams]"));
    assert!(kq.contains("api_key: {{apiKey}}"));

    let kh = content_of("API key in header");
    assert!(kh.contains("X-API-Key: {{apiKey}}"));

    let kc = content_of("API key in cookie");
    assert!(kc.contains("Cookie: session={{apiKey}}"));

    let oauth = content_of("OAuth2 op");
    assert!(oauth.contains("Authorization: Bearer {{accessToken}}"));

    let form = content_of("Urlencoded body");
    assert!(form.contains("[FormParams]"));
    assert!(form.contains("a: va"));

    let text = content_of("Text body");
    assert!(text.contains("hello world"));

    let multipart = content_of("Multipart body");
    assert!(multipart.contains("[MultipartFormData]"));
    assert!(multipart.contains("file: file,path=path/to/file"));
    assert!(multipart.contains("note: hi"));
}

#[test]
fn curl_covers_auth_and_body_variants() {
    let files = convert(ClientFormat::Curl);
    assert_eq!(files.len(), 1);
    let content = &files[0].content;

    assert!(content.contains("Authorization: Basic {{username}}:{{password}}"));
    assert!(content.contains("Authorization: Bearer {{accessToken}}"));
    assert!(content.contains("X-API-Key: {{apiKey}}"));
    assert!(content.contains("Cookie: session={{apiKey}}"));
    assert!(content.contains("api_key={{apiKey}}"));

    assert!(content.contains("--data 'a=va&b=5'"));
    assert!(content.contains("--data 'hello world'"));
    assert!(content.contains("--data 'bin-data'"));
    assert!(content.contains("-F 'file'='file=@path/to/file'"));
    assert!(content.contains("-F 'note'='hi'"));
}

const NO_SERVER_SPEC: &str = r#"
openapi: 3.0.0
info: {title: NoServer, version: 1}
paths:
  /ping:
    get:
      responses:
        '200': {description: ok}
"#;

#[test]
fn no_server_falls_back_to_default_base() {
    for &format in ClientFormat::ALL.iter().take(10) {
        let files = convert_openapi(NO_SERVER_SPEC, format, &ConvertOptions::default()).unwrap();
        let combined = files
            .iter()
            .map(|f| f.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            combined.contains("api.example.com"),
            "{format} output missing default base url"
        );
    }
}

#[test]
fn base_url_override_applies_to_all_formats() {
    let options = ConvertOptions {
        base_url: Some("https://override.example.com/".to_string()),
        ..ConvertOptions::default()
    };
    for &format in ClientFormat::ALL.iter().take(10) {
        let files = convert_openapi(EDGE_SPEC, format, &options).unwrap();
        let combined = files
            .iter()
            .map(|f| f.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            combined.contains("override.example.com"),
            "{format} output missing base url override"
        );
        assert!(
            !combined.contains("edge.example.com"),
            "{format} output still contains spec server"
        );
    }
}

#[test]
fn schemeless_base_url_is_wrapped() {
    let options = ConvertOptions {
        base_url: Some("internal.local".to_string()),
        ..ConvertOptions::default()
    };
    for format in [
        ClientFormat::Bruno,
        ClientFormat::Hoppscotch,
        ClientFormat::RestClient,
    ] {
        let files = convert_openapi(NO_SERVER_SPEC, format, &options).unwrap();
        let combined = files
            .iter()
            .map(|f| f.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            combined.contains("api.example.com/internal.local"),
            "{format} did not wrap schemeless base url"
        );
    }
}
