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

    let (service, socket) =
        LspService::new(|client| Backend::new(client, args.amber_version, None));
    Server::new(stdin, stdout, socket).serve(service).await;
}
