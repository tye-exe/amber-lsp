use chumsky::prelude::*;
use text::{ident, keyword};

use crate::grammar::alpha034::{expressions::parse_expr, Spanned, Statement};

pub fn var_init_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    keyword("let")
        .ignore_then(ident().map_with_span(|txt, span| (txt, span)).padded())
        .then_ignore(just("=").padded())
        .then(parse_expr(stmnts))
        .map_with_span(|(name, value), span| (Statement::VariableInit(name, Box::new(value)), span))
}
