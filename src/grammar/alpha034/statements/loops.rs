use chumsky::prelude::*;
use text::{ident, keyword};

use crate::grammar::alpha034::{expressions::parse_expr, IterLoopVars, Statement};

use super::block::block_parser;

pub fn inf_loop_parser(
    stmnts: Recursive<char, Statement, Simple<char>>,
) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    keyword("loop")
        .padded()
        .ignore_then(block_parser(stmnts))
        .map(Statement::InfiniteLoop)
}

pub fn iter_loop_parser(
    stmnts: Recursive<char, Statement, Simple<char>>,
) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    let single_var = ident().map(IterLoopVars::Single);
    let with_index_var = ident()
        .padded()
        .then_ignore(just(","))
        .padded()
        .then(ident())
        .padded()
        .map(|(var, index)| IterLoopVars::WithIndex(var, index));

    keyword("loop")
        .padded()
        .ignore_then(with_index_var.or(single_var))
        .padded()
        .then_ignore(keyword("in"))
        .padded()
        .then(parse_expr(stmnts.clone()))
        .padded()
        .then(block_parser(stmnts))
        .map(|((vars, expr), body)| Statement::IterLoop(vars, Box::new(expr), body))
}
