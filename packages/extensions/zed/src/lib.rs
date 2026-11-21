// api — Zed extension.
//
// Zed's extension API has no file-write surface for worktrees (only
// read-text-file, process spawn, downloads into the extension dir, and
// slash-command text output), so the extension embeds `api-core` directly and
// returns the converted files' contents as slash-command text — the assistant
// (or the user) can then write them where needed.
//
// Commands:
//   /api <spec.yaml|json> [format|all]  convert and emit files as text
//   /api-formats                        list supported formats

use api_core::{convert_openapi, convert_openapi_all, ClientFormat, ConvertOptions};
use zed_extension_api::{self as zed};

struct ApiExtension;

impl zed::Extension for ApiExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> zed::Result<zed::SlashCommandOutput> {
        match command.name.as_str() {
            "api" => convert(&args, worktree),
            "api-formats" => Ok(formats_output()),
            other => Err(format!("unknown command: {other}")),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
    ) -> zed::Result<Vec<zed::SlashCommandArgumentCompletion>> {
        // /api <spec> [format] — complete format names for the second arg.
        if command.name == "api" && args.len() == 1 {
            return Ok(ClientFormat::ALL
                .iter()
                .map(|format| zed::SlashCommandArgumentCompletion {
                    label: format.as_str().to_string(),
                    new_text: format.as_str().to_string(),
                    run_command: false,
                })
                .collect());
        }
        Ok(vec![])
    }
}

fn formats_output() -> zed::SlashCommandOutput {
    let list = ClientFormat::ALL
        .iter()
        .map(|format| format.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    zed::SlashCommandOutput {
        text: format!(
            "api supports these formats: {list}\n\nUse `/api <spec> [format|all]` to convert."
        ),
        sections: vec![],
    }
}

fn convert(
    args: &[String],
    worktree: Option<&zed::Worktree>,
) -> zed::Result<zed::SlashCommandOutput> {
    let rel = args
        .first()
        .ok_or("usage: /api <spec.json|yaml> [format|all]")?;
    let lower = rel.to_lowercase();
    if !(lower.ends_with(".json") || lower.ends_with(".yaml") || lower.ends_with(".yml")) {
        return Err("expected a .json, .yaml or .yml OpenAPI spec path".into());
    }

    let worktree = worktree.ok_or("no worktree available")?;
    let path = format!("{}/{}", worktree.root_path().trim_end_matches('/'), rel);
    let source = worktree.read_text_file(&path)?;

    let options = ConvertOptions {
        collection_name: None,
        base_url: None,
    };
    let files = match args.get(1).map(String::as_str) {
        None | Some("postman") => {
            convert_openapi(&source, ClientFormat::Postman, &options).map_err(|e| e.to_string())?
        }
        Some("all") => convert_openapi_all(&source, &options).map_err(|e| e.to_string())?,
        Some(name) => {
            let format = ClientFormat::parse(name)
                .ok_or_else(|| format!("unsupported format: {name} (try /api-formats)"))?;
            convert_openapi(&source, format, &options).map_err(|e| e.to_string())?
        }
    };

    let mut text = format!("Converted `{rel}` — {} file(s):\n", files.len());
    for file in &files {
        text.push_str(&format!(
            "\n### {}\n```\n{}\n```\n",
            file.path, file.content
        ));
    }
    Ok(zed::SlashCommandOutput {
        text,
        sections: vec![],
    })
}

zed::register_extension!(ApiExtension);
