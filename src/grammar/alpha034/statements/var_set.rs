use chumsky::prelude::*;
use chumsky::text::ident;

use crate::grammar::alpha034::{expressions::parse_expr, Spanned, Statement};

pub fn var_set_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    ident()
        .map_with_span(|txt, span| (txt, span))
        .then_ignore(just("=").padded())
        .then(parse_expr(stmnts))
        .map_with_span(|(name, value), span| (Statement::VariableSet(name, Box::new(value)), span))
}
