use chumsky::prelude::*;
use text::{ident, keyword};

use crate::grammar::alpha034::{expressions::parse_expr, IterLoopVars, Span, Spanned, Statement};

use super::block::block_parser;

pub fn inf_loop_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    keyword("loop")
        .ignore_then(filter(|c: &char| c.is_whitespace()).repeated())
        .ignore_then(block_parser(stmnts))
        .map_with_span(|block, span| (Statement::InfiniteLoop(block), span))
}

pub fn iter_loop_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    let single_var =
        ident().map_with_span(|txt, span: Span| (IterLoopVars::Single((txt, span.clone())), span));
    let with_index_var = ident()
        .map_with_span(|txt, span| (txt, span))
        .then_ignore(just(",").padded())
        .then(ident().map_with_span(|txt, span| (txt, span)))
        .map_with_span(|(var, index), span| (IterLoopVars::WithIndex(var, index), span));

    keyword("loop")
        .ignore_then(filter(|c: &char| c.is_whitespace()).repeated())
        .ignore_then(with_index_var.or(single_var))
        .then_ignore(keyword("in").padded())
        .then(parse_expr(stmnts.clone()).padded())
        .then(block_parser(stmnts))
        .map_with_span(|((vars, expr), body), span| {
            (Statement::IterLoop(vars, Box::new(expr), body), span)
        })
}
