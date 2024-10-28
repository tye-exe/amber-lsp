use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{
        expressions::parse_expr, lexer::Token, parser::ident, AmberParser, Block, Expression, IterLoopVars, Spanned, Statement
    },
    T,
};

use super::block::block_parser;

pub fn inf_loop_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<Statement>> {
    just(T!["loop"])
        .ignore_then(block_parser(stmnts))
        .map_with(|block, e| (Statement::InfiniteLoop(block), e.span()))
}

pub fn iter_loop_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<Statement>> {
    let single_var = ident("variable".to_string())
        .map_with(|txt, e| (IterLoopVars::Single((txt, e.span())), e.span()));
    let with_index_var = ident("variable".to_string())
        .map_with(|txt, e| (txt, e.span()))
        .then_ignore(just(T![","]))
        .then(
            ident("variable".to_string())
                .recover_with(via_parser(any().or_not().map(|_| "".to_string())))
                .map_with(|txt, e| (txt, e.span())),
        )
        .map_with(|(var, index), e| (IterLoopVars::WithIndex(var, index), e.span()));

    just(T!["loop"])
        .ignore_then(with_index_var.or(single_var).recover_with(via_parser(
            none_of([T!["in"]]).map_with(|_, e| (IterLoopVars::Error, e.span())),
        )))
        .then_ignore(just(T!["in"]).recover_with(via_parser(any().or_not().map(|_| T!["in"]))))
        .then(
            parse_expr(stmnts.clone()).recover_with(via_parser(
                any()
                    .or_not()
                    .map_with(|_, e| (Expression::Error, e.span())),
            )),
        )
        .then(block_parser(stmnts).recover_with(via_parser(
            any().or_not().map_with(|_, e| (Block::Error, e.span())),
        )))
        .map_with(|((vars, expr), body), e| {
            (Statement::IterLoop(vars, Box::new(expr), body), e.span())
        })
}
