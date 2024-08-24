use amber_lsp::grammar::alpha034::GlobalStatement;
use chumsky::error::Simple;
use tower_lsp::lsp_types::SemanticTokenType;

use crate::grammar::alpha034::Spanned;

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

pub const LEGEND_TYPE: &[SemanticTokenType] = &[
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRING,
    SemanticTokenType::COMMENT,
    SemanticTokenType::NUMBER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::TYPE,
    SemanticTokenType::MODIFIER,
];

#[derive(Debug)]
pub struct Parser {
    pub version: AmberVersion,
}

impl Parser {
    pub fn new(version: AmberVersion) -> Self {
        Self { version }
    }

    pub fn parse(&self, input: &str) -> (Option<Vec<Spanned<GlobalStatement>>>, Vec<Simple<char>>) {
        // let parsing_result = match self.version {
        //     AmberVersion::Alpha034 => parse_grammar(input),
        // };

        // parsing_result
        (None, vec![])
    }
}
