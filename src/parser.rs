use std::collections::HashMap;

use amber_lsp::grammar::{self, alpha034::grammar::Language};

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

    pub fn parse(&self, input: &str) -> Result<Language, Vec<rust_sitter::errors::ParseError>> {
        match self.version {
            AmberVersion::Alpha034 => {
                return Ok(grammar::alpha034::grammar::parse(input)?);
            }
        }
    }
}