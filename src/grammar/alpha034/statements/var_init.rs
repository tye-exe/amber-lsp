use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{expressions::parse_expr, lexer::Token, parser::ident, Spanned, Statement},
    T,
};

pub fn var_init_parser(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> + '_ {
    just(T!["let"])
        .ignore_then(ident().map_with_span(|txt, span| (txt, span)))
        .then_ignore(just(T!["="]))
        .then(parse_expr(stmnts))
        .map_with_span(|(name, value), span| (Statement::VariableInit(name, Box::new(value)), span))
}
