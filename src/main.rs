use dashmap::DashMap;
use grammar::alpha034::semantic_tokens::LEGEND_TYPE;
use grammar::alpha034::AmberCompiler;
use grammar::{Grammar, LSPAnalysis, ParserResponse, SpannedSemanticToken};
use ropey::Rope;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod grammar;

struct Backend {
    client: Client,
    /// A map from document URI to the parsed AST.
    ast_map: DashMap<String, Grammar>,
    /// A map from document URI to the document content and version.
    document_map: DashMap<String, (Rope, i32)>,
    /// A map from document URI to the semantic tokens.
    semantic_token_map: DashMap<String, Vec<SpannedSemanticToken>>,
    lsp_analysis: Box<dyn LSPAnalysis>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            ast_map: DashMap::new(),
            document_map: DashMap::new(),
            semantic_token_map: DashMap::new(),
            lsp_analysis: Box::new(AmberCompiler::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: {
                                TextDocumentRegistrationOptions {
                                    document_selector: Some(vec![DocumentFilter {
                                        language: Some("amber".to_string()),
                                        scheme: Some("file".to_string()),
                                        pattern: None,
                                    }]),
                                }
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                                legend: SemanticTokensLegend {
                                    token_types: LEGEND_TYPE.into(),
                                    token_modifiers: vec![].into(),
                                },
                                range: Some(true),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                            },
                            static_registration_options: StaticRegistrationOptions::default(),
                        },
                    ),
                ),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
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
        self.document_map.insert(
            params.text_document.uri.to_string(),
            (
                Rope::from_str(&params.text_document.text),
                params.text_document.version,
            ),
        );

        self.analize_document(params.text_document.uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {

        let uri = params.text_document.uri.to_string();

        if !self.document_map.contains_key(&uri) {
            return self
                .client
                .log_message(MessageType::ERROR, format!("document {uri} is not open"))
                .await;
        }

        if params
            .content_changes
            .iter()
            .any(|text| text.range_length.is_some())
        {
            return self
                .client
                .log_message(MessageType::ERROR, "range length changes are not supported")
                .await;
        }

        if let Some(change) = params
            .content_changes
            .iter()
            .find(|change| change.range.is_none() && change.range_length.is_none())
        {
            self.document_map.insert(
                uri,
                (Rope::from_str(&change.text), params.text_document.version),
            );
        } else {
            let mut document = self.document_map.get_mut(&uri).unwrap();

            params
                .content_changes
                .iter()
                .filter(|change| change.range.is_some())
                .for_each(|change| {
                    let range = change.range.as_ref().unwrap();
                    let start = document.0.line_to_char(range.start.line as usize)
                        + range.start.character as usize;
                    let end = document.0.line_to_char(range.end.line as usize)
                        + range.end.character as usize;

                    document.0.remove(start..end);
                    document.0.insert(start, &change.text);
                });

            document.1 = params.text_document.version;
        }

        self.analize_document(params.text_document.uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.document_map
            .remove(&params.text_document.uri.to_string());
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri.to_string();

        let semantic_tokens = match self.semantic_token_map.get(&uri) {
            Some(tokens) => tokens.clone(),
            None => {
                return Ok(None);
            }
        };

        let (rope, _) = match self.get_document(&params.text_document.uri).await {
            Some(document) => document,
            None => return Ok(None),
        };

        let mut pre_line = 0;
        let mut pre_start = 0;

        let data = semantic_tokens
            .iter()
            .filter_map(|(token, span)| {
                let length = span.end - span.start;
                // Get the line number of the token
                let line = rope.try_byte_to_line(span.start).ok()? as u32;
                // Get the first character of the line
                let first = rope.try_line_to_char(line as usize).ok()? as u32;
                // Get the start position of the token relative to the line
                let start = rope.try_byte_to_char(span.start).ok()? as u32 - first;

                // Calculate the delta line and delta start
                let delta_line = line - pre_line;

                // If the token is on the same line as the previous token
                // calculate the delta start relative to the previous token
                // otherwise calculate the delta start relative to the first character of the line
                let delta_start = if delta_line == 0 {
                    start - pre_start
                } else {
                    start
                };

                let ret = Some(SemanticToken {
                    delta_line,
                    delta_start,
                    length: length as u32,
                    token_type: *token as u32,
                    token_modifiers_bitset: 0,
                });
                pre_line = line;
                pre_start = start;
                ret
            })
            .collect();

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data,
        })))
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let uri = params.text_document.uri.to_string();
        let requested_range = params.range;

        let semantic_tokens = match self.semantic_token_map.get(&uri) {
            Some(tokens) => tokens.clone(),
            None => {
                return Ok(None);
            }
        };

        let (rope, _) = match self.get_document(&params.text_document.uri).await {
            Some(document) => document,
            None => return Ok(None),
        };

        let mut pre_line = 0;
        let mut pre_start = 0;

        let data = semantic_tokens
            .iter()
            .filter_map(|(token, span)| {
                let line = rope.try_byte_to_line(span.start).ok()? as u32;
                let first = rope.try_line_to_char(line as usize).ok()? as u32;
                let start = rope.try_byte_to_char(span.start).ok()? as u32 - first;

                if !(line >= requested_range.start.line
                    && (line < requested_range.end.line
                        || (line == requested_range.end.line
                            && start <= requested_range.end.character)))
                {
                    return None;
                }

                let length = span.end - span.start;
                let delta_line = line - pre_line;
                let delta_start = if delta_line == 0 {
                    start - pre_start
                } else {
                    start
                };

                let ret = Some(SemanticToken {
                    delta_line,
                    delta_start,
                    length: length as u32,
                    token_type: *token as u32,
                    token_modifiers_bitset: 0,
                });
                pre_line = line;
                pre_start = start;
                ret
            })
            .collect();

        Ok(Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
            result_id: None,
            data,
        })))
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }
}

impl Backend {
    async fn get_document(&self, uri: &Url) -> Option<(Rope, i32)> {
        match self.document_map.get(&uri.to_string()) {
            Some(document) => Some(document.clone()),
            None => {
                self.client
                    .log_message(MessageType::ERROR, format!("document {uri} is not open"))
                    .await;

                return None;
            }
        }
    }

    async fn analize_document(&self, uri: Url) {
        let (rope, version) = match self.get_document(&uri).await {
            Some(document) => document,
            None => return,
        };

        let tokens = self.lsp_analysis.tokenize(&rope.to_string());

        let ParserResponse {
            ast,
            errors,
            semantic_tokens,
        } = self.lsp_analysis.parse(&tokens);

        let diagnostics = errors
            .iter()
            .filter_map(|error| {
                let msg = error.to_string();
                let span = error.span();

                || -> Option<Diagnostic> {
                    let start_position = self.offset_to_position(span.start, &rope)?;
                    let end_position = self.offset_to_position(span.end, &rope)?;

                    Some(Diagnostic::new_simple(
                        Range::new(start_position, end_position),
                        msg.to_string(),
                    ))
                }()
            })
            .collect::<Vec<_>>();

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, Some(version))
            .await;

        self.ast_map.insert(uri.to_string(), ast);
        self.semantic_token_map
            .insert(uri.to_string(), semantic_tokens);
    }

    fn offset_to_position(&self, offset: usize, rope: &Rope) -> Option<Position> {
        let line = rope
            .try_char_to_line(offset)
            .ok()
            .unwrap_or(rope.len_lines());
        let first_char_of_line = rope.try_line_to_char(line).ok().unwrap_or(rope.len_chars());
        let column = offset - first_char_of_line;
        Some(Position::new(line as u32, column as u32))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
