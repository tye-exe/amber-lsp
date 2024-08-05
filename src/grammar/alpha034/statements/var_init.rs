use chumsky::prelude::*;
use text::{ident, keyword};

use crate::grammar::alpha034::{expressions::parse_expr, Statement};

pub fn var_init_parser(stmnts: Recursive<char, Statement, Simple<char>>) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    keyword("let")
        .padded()
        .ignore_then(ident())
        .padded()
        .then_ignore(just("="))
        .padded()
        .then(parse_expr(stmnts))
        .map(|(name, value)| Statement::VariableInit(name, Box::new(value)))
}