use amber_lsp::grammar::{alpha034::AmberCompiler, LSPAnalysis, ParserResponse};

#[derive(Debug)]
pub enum AmberVersion {
    Alpha034,
}

pub struct Parser {
    pub version: AmberVersion,
    lsp_analysis: Box<dyn LSPAnalysis>,
}

impl Parser {
    pub fn new(version: AmberVersion) -> Self {
        let lsp_analysis = match version {
            AmberVersion::Alpha034 => AmberCompiler::new(),
        };

        Self {
            version,
            lsp_analysis: Box::new(lsp_analysis),
        }
    }

    pub fn parse(&mut self, input: &str) -> ParserResponse {
        self.lsp_analysis.parse(input)
    }
}
