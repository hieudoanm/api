use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const SPEC: &str = r#"
openapi: 3.0.3
info:
  title: Cli Petstore
  version: 1.0.0
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
          example: 10
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
"#;

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("api-cli-test-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn spec_file(dir: &Path) -> PathBuf {
    let path = dir.join("spec.yaml");
    std::fs::write(&path, SPEC).unwrap();
    path
}

fn api() -> Command {
    Command::new(env!("CARGO_BIN_EXE_api"))
}

#[test]
fn formats_lists_all_formats() {
    let output = api().arg("formats").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
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
        assert!(
            stdout.lines().any(|line| line == expected),
            "missing {expected}"
        );
    }
}

#[test]
fn convert_writes_postman_by_default() {
    let dir = temp_dir("default");
    let spec = spec_file(&dir);
    let out = dir.join("out");

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let collection = out
        .join("postman")
        .join("cli-petstore.postman_collection.json");
    assert!(collection.exists());
    let content = std::fs::read_to_string(collection).unwrap();
    assert!(content.contains("Cli Petstore"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_writes_multiple_formats() {
    let dir = temp_dir("multi");
    let spec = spec_file(&dir);
    let out = dir.join("out");

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--format")
        .arg("postman,curl")
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(out
        .join("postman")
        .join("cli-petstore.postman_collection.json")
        .exists());
    assert!(out.join("curl").join("cli-petstore.sh").exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_format_all_emits_every_format() {
    let dir = temp_dir("all");
    let spec = spec_file(&dir);
    let out = dir.join("out");

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--format")
        .arg("all")
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();

    assert!(output.status.success());
    for format in [
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
        assert!(out.join(format).exists(), "missing {format} output");
    }

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_dedupes_and_skips_empty_format_parts() {
    let dir = temp_dir("dedupe");
    let spec = spec_file(&dir);
    let out = dir.join("out");

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--format")
        .arg("postman,,postman, curl ")
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();

    assert!(output.status.success());
    let count = std::fs::read_dir(&out)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .count();
    assert_eq!(count, 2);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_rejects_unknown_format() {
    let dir = temp_dir("badformat");
    let spec = spec_file(&dir);

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--format")
        .arg("bogus")
        .arg("--out")
        .arg(dir.join("out"))
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported format"), "stderr: {stderr}");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_rejects_missing_input_file() {
    let dir = temp_dir("missing");

    let output = api()
        .arg("convert")
        .arg(dir.join("nope.yaml"))
        .arg("--out")
        .arg(dir.join("out"))
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("failed to read"), "stderr: {stderr}");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_rejects_invalid_spec() {
    let dir = temp_dir("invalid");
    let spec = dir.join("bad.yaml");
    std::fs::write(&spec, "this is not openapi").unwrap();

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--format")
        .arg("postman")
        .arg("--out")
        .arg(dir.join("out"))
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("api:"), "stderr: {stderr}");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_reads_spec_from_stdin() {
    let dir = temp_dir("stdin");
    let out = dir.join("out");

    let mut child = api()
        .arg("convert")
        .arg("-")
        .arg("--format")
        .arg("rest-client")
        .arg("--out")
        .arg(&out)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(SPEC.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert!(out.join("rest-client").join("cli-petstore.http").exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_applies_name_and_base_url_overrides() {
    let dir = temp_dir("overrides");
    let spec = spec_file(&dir);
    let out = dir.join("out");

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--format")
        .arg("postman")
        .arg("--name")
        .arg("Renamed API")
        .arg("--base-url")
        .arg("https://staging.example.com/v2")
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();

    assert!(output.status.success());
    let collection = out
        .join("postman")
        .join("renamed-api.postman_collection.json");
    assert!(collection.exists());
    let content = std::fs::read_to_string(collection).unwrap();
    assert!(content.contains("Renamed API"));
    assert!(content.contains("https://staging.example.com/v2"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_accepts_output_alias() {
    let dir = temp_dir("alias");
    let spec = spec_file(&dir);
    let out = dir.join("out");

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--format")
        .arg("postman")
        .arg("--output")
        .arg(&out)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(out.join("postman").exists());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn convert_fails_when_output_path_is_a_file() {
    let dir = temp_dir("badout");
    let spec = spec_file(&dir);
    let blocker = dir.join("blocker");
    std::fs::write(&blocker, "not a directory").unwrap();

    let output = api()
        .arg("convert")
        .arg(&spec)
        .arg("--format")
        .arg("postman")
        .arg("--out")
        .arg(&blocker)
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("failed to create"), "stderr: {stderr}");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn mcp_server_exits_cleanly_on_eof() {
    let output = api().arg("mcp").output().unwrap();
    assert!(output.status.success());
}

#[test]
fn mcp_server_responds_to_initialize() {
    let mut child = api()
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let initialize = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
    writeln!(child.stdin.as_mut().unwrap(), "{initialize}").unwrap();
    // Close stdin so the server exits after responding.
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("protocolVersion"), "stdout: {stdout}");
}

#[test]
fn version_and_help_exit_zero() {
    let version = api().arg("--version").output().unwrap();
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains("api"));

    let help = api().arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("convert"));
}
