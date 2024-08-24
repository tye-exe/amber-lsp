use chumsky::prelude::*;

use crate::grammar::alpha034::{lexer::Token, Spanned, Statement};

pub fn comment_parser() -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> {
    filter(|t: &Token| t.to_string().starts_with("//"))
        .map_with_span(|com, span| (Statement::Comment(com.to_string()[2..].to_string()), span))
}
