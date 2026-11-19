use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use api_core::{convert_openapi, ClientFormat, ConvertOptions};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "api")]
#[command(
    version = concat!(env!("CARGO_PKG_VERSION"), " (build ", env!("BUILD_TIME"), ")")
)]
#[command(about = "Convert OpenAPI specs to API client collections")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert an OpenAPI JSON/YAML document into API client formats
    Convert {
        /// OpenAPI file path, or `-` for stdin
        input: String,
        /// Output directory (default: ./api-collections)
        #[arg(
            short,
            long = "out",
            visible_alias = "output",
            default_value = "api-collections"
        )]
        output: PathBuf,
        /// Comma-separated client formats: postman, bruno, insomnia, hoppscotch, rest-client, thunder-client, yaak, hurl, kreya, curl, all
        #[arg(short, long, value_delimiter = ',')]
        format: Vec<String>,
        /// Collection name override
        #[arg(long)]
        name: Option<String>,
        /// Base URL override
        #[arg(long)]
        base_url: Option<String>,
    },
    /// Generate browsable API documentation (HTML) from an OpenAPI document
    Preview {
        /// OpenAPI file path, or `-` for stdin
        input: String,
        /// Output directory (default: ./api-docs)
        #[arg(
            short,
            long = "out",
            visible_alias = "output",
            default_value = "api-docs"
        )]
        output: PathBuf,
        /// Comma-separated preview formats: swagger-ui, redoc, scalar, stoplight, rapidoc, all
        #[arg(short, long, value_delimiter = ',')]
        format: Vec<String>,
        /// Collection name override
        #[arg(long)]
        name: Option<String>,
    },
    /// List supported output formats
    Formats,
    /// Run the MCP server over stdio (JSON-RPC 2.0)
    Mcp,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("api: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Formats => {
            for format in ClientFormat::ALL {
                println!("{}", format.as_str());
            }
        }
        Commands::Mcp => {
            api_mcp::serve().context("MCP server failed")?;
        }
        Commands::Convert {
            input,
            output,
            format,
            name,
            base_url,
        } => {
            let formats = if format.is_empty() {
                vec![ClientFormat::Postman]
            } else {
                parse_formats(
                    &format,
                    &ClientFormat::CLIENTS,
                    "postman, bruno, insomnia, hoppscotch, rest-client, thunder-client, yaak, hurl, kreya, curl",
                )?
            };
            run_conversion(
                &input,
                &output,
                formats,
                ConvertOptions {
                    collection_name: name,
                    base_url,
                },
            )?;
        }
        Commands::Preview {
            input,
            output,
            format,
            name,
        } => {
            let formats = if format.is_empty() {
                vec![ClientFormat::SwaggerUi]
            } else {
                parse_formats(
                    &format,
                    &ClientFormat::PREVIEWS,
                    "swagger-ui, redoc, scalar, stoplight, rapidoc",
                )?
            };
            run_conversion(
                &input,
                &output,
                formats,
                ConvertOptions {
                    collection_name: name,
                    base_url: None,
                },
            )?;
        }
    }
    Ok(())
}

fn run_conversion(
    input: &str,
    output: &Path,
    formats: Vec<ClientFormat>,
    options: ConvertOptions,
) -> Result<()> {
    let source = read_input(input)?;
    let mut files = Vec::new();
    for format in formats {
        files.extend(convert_openapi(&source, format, &options)?);
    }
    write_files(output, files)
}

fn read_input(input: &str) -> Result<String> {
    if input == "-" {
        let mut source = String::new();
        io::stdin()
            .read_to_string(&mut source)
            .context("failed to read OpenAPI document from stdin")?;
        Ok(source)
    } else {
        fs::read_to_string(input)
            .with_context(|| format!("failed to read OpenAPI document {input}"))
    }
}

fn parse_formats(
    values: &[String],
    allowed: &[ClientFormat],
    hint: &str,
) -> Result<Vec<ClientFormat>> {
    let mut formats = Vec::new();
    for value in values {
        for part in value.split(',') {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.eq_ignore_ascii_case("all") {
                for format in allowed {
                    if !formats.contains(format) {
                        formats.push(*format);
                    }
                }
                continue;
            }
            let format = ClientFormat::parse(trimmed)
                .filter(|format| allowed.contains(format))
                .with_context(|| {
                    format!("unsupported format {trimmed}; expected one of: {hint}")
                })?;
            if !formats.contains(&format) {
                formats.push(format);
            }
        }
    }
    Ok(formats)
}

fn write_files(output: &Path, files: Vec<api_core::ConversionFile>) -> Result<()> {
    let mut count = 0usize;
    for file in files {
        let dir = output.join(file.format.as_str());
        let path = dir.join(&file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(&path, file.content)
            .with_context(|| format!("failed to write {}", path.display()))?;
        eprintln!("wrote {}", path.display());
        count += 1;
    }
    println!("wrote {count} file(s) to {}", output.display());
    Ok(())
}
