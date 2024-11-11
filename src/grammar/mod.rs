use alpha034::GlobalStatement;
use chumsky::error::Simple;

pub mod alpha034;

#[derive(Debug, PartialEq, Clone)]
pub enum Grammar {
    Alpha034(Option<Vec<Spanned<GlobalStatement>>>),
}

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);
pub type SpannedSemanticToken = Spanned<usize>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(pub String);

impl ToString for Token {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromIterator<Token> for String {
    fn from_iter<I: IntoIterator<Item = Token>>(iter: I) -> Self {
        iter.into_iter().map(|t| t.0).collect()
    }
}

#[macro_export]
macro_rules! T {
    [$text:expr] => {
        Token($text.to_string())
    };
}

pub struct ParserResponse {
    pub ast: Grammar,
    pub errors: Vec<Simple<String>>,
    pub semantic_tokens: Vec<SpannedSemanticToken>,
}

pub trait LSPAnalysis: Sync + Send {
    fn parse(&self, input: &str) -> ParserResponse;
}

#[derive(PartialEq)]
pub enum JumpDefinitionResult {
    InFile(Span),
    OpenFile(String),
    None,
}
