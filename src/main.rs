use std::{os::unix::process::ExitStatusExt, process::{Command, Stdio}};

use amber_lsp::backend::{AmberVersion, Backend};
use clap::Parser;
use tower_lsp::{LspService, Server};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Version of the Amber language to use.
    #[arg(value_enum, long, short, default_value = "auto")]
    amber_version: AmberVersion,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let amber_version = if args.amber_version == AmberVersion::Auto {
        detect_amber_version()
    } else {
        args.amber_version
    };

    let (service, socket) = LspService::new(|client| Backend::new(client, amber_version, None));
    Server::new(stdin, stdout, socket).serve(service).await;
}

fn detect_amber_version() -> AmberVersion {
    let output = String::from_utf8_lossy(
        Command::new("amber")
            .arg("-V")
            .stdout(Stdio::piped())
            .output()
            .unwrap_or(std::process::Output {
                stdout: Vec::new(),
                stderr: Vec::new(),
                status: std::process::ExitStatus::from_raw(0),
            })
            .stdout
            .as_slice(),
    )
    .to_string();

    match output.split_whitespace().last() {
        Some("0.3.4-alpha") => AmberVersion::Alpha034,
        Some("0.3.5-alpha") => AmberVersion::Alpha035,
        Some("0.4.0-alpha") => AmberVersion::Alpha040,
        _ => AmberVersion::Auto,
    }
}
