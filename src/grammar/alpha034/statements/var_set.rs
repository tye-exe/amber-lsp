use chumsky::prelude::*;
use chumsky::text::ident;

use crate::grammar::alpha034::{expressions::parse_expr, Statement};

pub fn var_set_parser(stmnts: Recursive<char, Statement, Simple<char>>) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    ident()
        .padded()
        .then_ignore(just("="))
        .padded()
        .then(parse_expr(stmnts))
        .map(|(name, value)| Statement::VariableSet(name, Box::new(value)))
}