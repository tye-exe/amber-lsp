use dashmap::DashMap;
use parser::Parser;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod grammar;
mod parser;

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, Rope>,
    parser: Parser,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            document_map: DashMap::new(),
            parser: Parser::new(parser::AmberVersion::Alpha034),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.document_map.insert(params.text_document.uri.to_string(), Rope::from_str(&params.text_document.text));
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if params.content_changes.iter().any(|text| text.range_length.is_some()) {
            self.client
                .log_message(MessageType::ERROR, "range length changes are not supported")
                .await;
        }

        if let Some(change) = params.content_changes.iter().find(|change| change.range.is_none() && change.range_length.is_none()) {
            self.document_map.insert(params.text_document.uri.to_string(), Rope::from_str(&change.text));
            return;
        }

        let mut document = self.document_map.get_mut(&params.text_document.uri.to_string()).unwrap();

        params.content_changes.iter()
            .filter(|change| change.range.is_some())
            .for_each(|change| {
                let range = change.range.as_ref().unwrap();
                let start = document.line_to_char(range.start.line as usize) + range.start.character as usize;
                let end = document.line_to_char(range.end.line as usize) + range.end.character as usize;

                document.remove(start..end);
                document.insert(start, &change.text);
            });
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.document_map.remove(&params.text_document.uri.to_string());
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}