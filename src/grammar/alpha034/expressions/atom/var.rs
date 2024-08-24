use crate::grammar::alpha034::{lexer::Token, parser::ident, Spanned};

use super::Expression;
use chumsky::prelude::*;

const KEYWORDS: [&str; 24] = [
    "if", "else", "loop", "in", "return", "break", "continue", "true", "false", "null", "fun",
    "as", "is", "or", "and", "not", "nameof", "status", "fail", "echo", "let", "unsafe", "silent",
    "main",
];

pub fn var_parser() -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> {
    ident().try_map(move |id, span| {
        if KEYWORDS.contains(&id.as_str()) {
            return Err(Simple::custom(span, "keyword used as variable name"));
        }

        Ok((Expression::Var((id, span.clone())), span))
    })
}
