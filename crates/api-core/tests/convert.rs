use api_core::{convert_openapi, parse_openapi, BodyKind, ClientFormat, ConvertOptions};
use serde_json::Value;

const SPEC: &str = r#"
openapi: 3.0.3
info:
  title: Pet Store
  description: Pets and users API
  version: 1.2.3
servers:
  - url: https://{environment}.example.com/{version}
    variables:
      environment:
        default: api
      version:
        default: v1
security:
  - bearerAuth: []
paths:
  /users/{userId}:
    parameters:
      - name: userId
        in: path
        required: true
        schema:
          type: string
        example: u_123
    get:
      tags: [Users]
      summary: Get user
      description: Fetch a single user
      parameters:
        - name: verbose
          in: query
          required: false
          schema:
            type: boolean
          example: true
        - name: X-Request-Id
          in: header
          required: false
          schema:
            type: string
          example: req-1
      responses:
        '200':
          description: OK
    post:
      tags: [Users]
      summary: Update user
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/User'
      responses:
        '200':
          description: OK
  /users/{userId}/avatar:
    post:
      tags: [Users]
      summary: Upload avatar
      requestBody:
        required: true
        content:
          multipart/form-data:
            schema:
              type: object
              required: [file]
              properties:
                file:
                  type: string
                  format: binary
                caption:
                  type: string
                  example: me
      responses:
        '200':
          description: OK
components:
  schemas:
    User:
      type: object
      required: [name, email]
      properties:
        name:
          type: string
          example: Ada
        email:
          type: string
          format: email
          example: ada@example.com
        role:
          type: string
          enum: [admin, user]
          default: user
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
"#;

#[test]
fn parses_openapi_yaml_into_collection() {
    let collection = parse_openapi(SPEC).unwrap();

    assert_eq!(collection.name, "Pet Store");
    assert_eq!(collection.version, "1.2.3");
    assert_eq!(collection.servers, vec!["https://api.example.com/v1"]);
    assert_eq!(collection.folders.len(), 1);
    assert_eq!(collection.folders[0].name, "Users");
    assert_eq!(collection.folders[0].requests.len(), 3);

    let get = collection.folders[0]
        .requests
        .iter()
        .find(|request| request.method == "GET")
        .unwrap();
    assert_eq!(get.name, "Get user");
    assert_eq!(get.path_params[0].name, "userId");
    assert_eq!(get.path_params[0].value, "u_123");
    assert_eq!(get.query_params[0].name, "verbose");
    assert_eq!(get.query_params[0].value, "true");
    assert_eq!(get.headers[0].name, "X-Request-Id");
    assert!(matches!(
        get.auth.first(),
        Some(api_core::ApiAuth::Bearer { token }) if token == "{{accessToken}}"
    ));

    let update = collection.folders[0]
        .requests
        .iter()
        .find(|request| request.method == "POST" && request.path == "/users/{userId}")
        .unwrap();
    let body = update.body.as_ref().unwrap();
    assert_eq!(body.kind, BodyKind::Json);
    assert!(body.raw.as_deref().unwrap().contains("Ada"));

    let avatar = collection.folders[0]
        .requests
        .iter()
        .find(|request| request.path.ends_with("/avatar"))
        .unwrap();
    let body = avatar.body.as_ref().unwrap();
    assert_eq!(body.kind, BodyKind::FormData);
    assert!(body
        .fields
        .iter()
        .any(|field| field.name == "file" && field.value == "<file>"));
    assert!(body
        .fields
        .iter()
        .any(|field| field.name == "caption" && field.value == "me"));
}

#[test]
fn converts_to_postman_collection() {
    let files = convert_openapi(SPEC, ClientFormat::Postman, &ConvertOptions::default()).unwrap();
    assert_eq!(files.len(), 1);
    let document: Value = serde_json::from_str(&files[0].content).unwrap();

    assert!(files[0].path.ends_with(".postman_collection.json"));
    assert_eq!(
        document["info"]["schema"],
        "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
    );
    let folder = &document["item"][0];
    assert_eq!(folder["name"], "Users");
    let get = folder["item"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["request"]["method"] == "GET")
        .unwrap();
    assert_eq!(
        get["request"]["url"]["raw"],
        "{{baseUrl}}/users/:userId?verbose=true"
    );
    assert_eq!(get["request"]["auth"]["type"], "bearer");
    assert!(get["request"]["url"]["variable"]
        .as_array()
        .unwrap()
        .iter()
        .any(|variable| variable["key"] == "userId" && variable["value"] == "u_123"));
}

#[test]
fn converts_to_bruno_collection_files() {
    let files = convert_openapi(SPEC, ClientFormat::Bruno, &ConvertOptions::default()).unwrap();
    let paths = files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<Vec<_>>();

    assert!(paths.contains(&"bruno.json"));
    assert!(paths.contains(&"collection.bru"));
    assert!(paths.contains(&"environments/development.bru"));
    assert!(paths.contains(&"users/folder.bru"));
    assert!(paths
        .iter()
        .any(|path| path.ends_with("get-users-userid.bru")));
    assert!(paths
        .iter()
        .any(|path| path.ends_with("post-users-userid-avatar.bru")));

    let get = files
        .iter()
        .find(|file| file.path.ends_with("get-users-userid.bru"))
        .unwrap();
    assert!(get.content.contains("get {"));
    assert!(get.content.contains("{{baseUrl}}/users/:userId"));
    assert!(get.content.contains("~verbose: true"));
    assert!(get.content.contains("auth:bearer"));

    let avatar = files
        .iter()
        .find(|file| file.path.ends_with("post-users-userid-avatar.bru"))
        .unwrap();
    assert!(avatar.content.contains("body:multipart-form"));
    assert!(avatar.content.contains("file: <file>"));
}

#[test]
fn converts_to_insomnia_export() {
    let files = convert_openapi(SPEC, ClientFormat::Insomnia, &ConvertOptions::default()).unwrap();
    assert_eq!(files.len(), 1);
    let document: Value = serde_json::from_str(&files[0].content).unwrap();

    assert_eq!(document["__export_format"], 4);
    let resources = document["resources"].as_array().unwrap();
    assert!(resources
        .iter()
        .any(|resource| resource["_type"] == "workspace"));
    assert!(resources
        .iter()
        .any(|resource| resource["_type"] == "request_group" && resource["name"] == "Users"));
    let get = resources
        .iter()
        .find(|resource| resource["_type"] == "request" && resource["method"] == "GET")
        .unwrap();
    assert_eq!(get["url"], "{{ _.base_url }}/users/:userId");
    assert_eq!(get["authentication"]["type"], "bearer");
}

#[test]
fn converts_to_hoppscotch_collection() {
    let files =
        convert_openapi(SPEC, ClientFormat::Hoppscotch, &ConvertOptions::default()).unwrap();
    assert_eq!(files.len(), 1);
    let document: Value = serde_json::from_str(&files[0].content).unwrap();

    let folder = &document["folders"][0];
    assert_eq!(folder["name"], "Users");
    let get = folder["requests"]
        .as_array()
        .unwrap()
        .iter()
        .find(|request| request["method"] == "GET")
        .unwrap();
    assert_eq!(get["endpoint"], "https://api.example.com/v1/users/u_123");
    assert_eq!(get["auth"]["authType"], "bearer");
}

#[test]
fn converts_to_rest_client_http_file() {
    let files =
        convert_openapi(SPEC, ClientFormat::RestClient, &ConvertOptions::default()).unwrap();
    assert_eq!(files.len(), 1);
    let content = &files[0].content;

    assert!(content.contains("@baseUrl = https://api.example.com/v1"));
    assert!(content.contains("### Users — Get user"));
    assert!(content.contains("GET {{baseUrl}}/users/{{userId}}?verbose=true"));
    assert!(content.contains("Authorization: Bearer {{accessToken}}"));
    assert!(content.contains("\"Ada\""));
    assert!(content.contains("file=@file(path/to/file)"));
}

#[test]
fn converts_to_thunder_client_collection() {
    let files = convert_openapi(
        SPEC,
        ClientFormat::ThunderClient,
        &ConvertOptions::default(),
    )
    .unwrap();
    assert_eq!(files.len(), 1);
    let document: Value = serde_json::from_str(&files[0].content).unwrap();

    assert_eq!(document["client"], "Thunder Client");
    assert_eq!(document["collectionName"], "Pet Store");
    assert_eq!(document["version"], "1.1");
    let folder = &document["folders"][0];
    assert_eq!(folder["name"], "Users");
    let requests = document["requests"].as_array().unwrap();
    assert_eq!(requests.len(), 3);
    let get = requests
        .iter()
        .find(|request| request["method"] == "GET")
        .unwrap();
    assert_eq!(get["containerId"], folder["_id"]);
    assert_eq!(
        get["url"],
        "https://api.example.com/v1/users/{{userId}}?verbose=true"
    );
    assert!(get["params"]
        .as_array()
        .unwrap()
        .iter()
        .any(|param| param["name"] == "userId" && param["isPath"] == true));
    assert!(get["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|header| header["name"] == "Authorization"));
}

#[test]
fn converts_to_yaak_sync_files() {
    let files = convert_openapi(SPEC, ClientFormat::Yaak, &ConvertOptions::default()).unwrap();
    let paths = files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<Vec<_>>();

    assert!(paths.contains(&"workspace.yaml"));
    assert!(paths.contains(&"environment-development.yaml"));
    assert!(paths.contains(&"folder-users.yaml"));
    assert!(paths
        .iter()
        .any(|path| path.starts_with("request-users-") && path.contains("get-user")));

    let workspace: Value = serde_yml::from_str(
        &files
            .iter()
            .find(|file| file.path == "workspace.yaml")
            .unwrap()
            .content,
    )
    .unwrap();
    assert_eq!(workspace["model"], "workspace");
    assert_eq!(workspace["name"], "Pet Store");

    let get = files
        .iter()
        .find(|file| file.path.contains("get-user"))
        .unwrap();
    let request: Value = serde_yml::from_str(&get.content).unwrap();
    assert_eq!(request["model"], "http_request");
    assert_eq!(request["method"], "GET");
    assert_eq!(request["url"], "${[ base_url ]}/users/:userId");
    assert!(request["urlParameters"]
        .as_array()
        .unwrap()
        .iter()
        .any(|param| param["name"] == "verbose"));
}

#[test]
fn converts_to_hurl_files() {
    let files = convert_openapi(SPEC, ClientFormat::Hurl, &ConvertOptions::default()).unwrap();
    let get = files
        .iter()
        .find(|file| file.path.ends_with("get-user.hurl"))
        .unwrap();

    assert!(get
        .content
        .contains("GET https://api.example.com/v1/users/u_123"));
    assert!(get.content.contains("[QueryStringParams]"));
    assert!(get.content.contains("verbose: true"));
    assert!(get
        .content
        .contains("Authorization: Bearer {{accessToken}}"));

    let avatar = files
        .iter()
        .find(|file| file.path.ends_with("post-upload-avatar.hurl"))
        .unwrap();
    assert!(avatar.content.contains("[MultipartFormData]"));
    assert!(avatar.content.contains("file: file,path=path/to/file"));
}

#[test]
fn converts_to_kreya_project() {
    let files = convert_openapi(SPEC, ClientFormat::Kreya, &ConvertOptions::default()).unwrap();
    let paths = files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<Vec<_>>();

    assert!(paths.iter().any(|path| path.ends_with(".krproj")));
    assert!(paths.contains(&"pet-store/directory.krpref"));
    assert!(paths.contains(&"pet-store/development.krenv"));
    assert!(paths.iter().any(|path| path.ends_with(".krop")));

    let krop = files
        .iter()
        .find(|file| file.path.ends_with("Get user.krop"))
        .unwrap();
    let operation: Value = serde_json::from_str(&krop.content).unwrap();
    assert_eq!(operation["invokerName"], "rest");
    assert_eq!(operation["details"]["method"], "GET");
    assert_eq!(operation["details"]["path"], "/users/{userId}");
    assert!(operation["details"]["headers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|header| header["key"] == "Authorization"));

    let update = files
        .iter()
        .find(|file| file.path.ends_with("Update user.krop"))
        .unwrap();
    let operation: Value = serde_json::from_str(&update.content).unwrap();
    assert_eq!(operation["requests"][0]["contentType"], "json");
    assert!(paths
        .iter()
        .any(|path| path.ends_with("Update user-request.json")));
}

#[test]
fn converts_to_curl_script() {
    let files = convert_openapi(SPEC, ClientFormat::Curl, &ConvertOptions::default()).unwrap();
    assert_eq!(files.len(), 1);
    let content = &files[0].content;

    assert!(content.starts_with("#!/usr/bin/env bash"));
    assert!(content.contains("BASE_URL='https://api.example.com/v1'"));
    assert!(content.contains("curl -X GET \"$BASE_URL/users/u_123?verbose=true\""));
    assert!(content.contains("-H 'Authorization: Bearer {{accessToken}}'"));
    assert!(content.contains("--data '{"));
    assert!(content.contains("file=@path/to/file"));
}

#[test]
fn supports_all_formats() {
    for format in ClientFormat::ALL {
        let files = convert_openapi(SPEC, format, &ConvertOptions::default()).unwrap();
        assert!(!files.is_empty(), "{format} produced no files");
    }
}
