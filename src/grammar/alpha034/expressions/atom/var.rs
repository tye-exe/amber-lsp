use crate::grammar::alpha034::{lexer::Token, parser::ident, Spanned};

use super::Expression;
use chumsky::prelude::*;

pub fn var_parser() -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> {
    ident("variable".to_string())
        .map_with_span(|name, span| (Expression::Var((name, span.clone())), span))
}
