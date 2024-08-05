use amber_lsp::grammar::alpha034::{parse as parse_grammar, GlobalStatement};
use chumsky::error::Simple;
use chumsky::Parser as ChumskyParser;

#[derive(Debug)]
pub enum AmberVersion {
    Alpha034,
}


#[derive(Debug)]
pub struct ParserResult {
    // pub ast: Option<HashMap<String, Func>>,
    // pub parse_errors: Vec<Simple<String>>,
    // pub semantic_tokens: Vec<ImCompleteSemanticToken>,
}

#[derive(Debug)]
pub struct Parser {
    pub version: AmberVersion,
}

impl Parser {
    pub fn new(version: AmberVersion) -> Self {
        Self { version }
    }

    pub fn parse(&self, input: &str) -> (Option<GlobalStatement>, Vec<Simple<char>>) {
        match self.version {
            AmberVersion::Alpha034 => {
                parse_grammar().parse_recovery(input)
            }
        }
    }
}