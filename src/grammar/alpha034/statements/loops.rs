use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{
        expressions::parse_expr, lexer::Token, parser::ident, Block, Expression, IterLoopVars,
        Spanned, Statement,
    },
    T,
};

use super::block::block_parser;

pub fn inf_loop_parser(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> + '_ {
    just(T!["loop"])
        .ignore_then(block_parser(stmnts))
        .map_with_span(|block, span| (Statement::InfiniteLoop(block), span))
}

pub fn iter_loop_parser(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> + '_ {
    let single_var = ident("variable".to_string())
        .map_with_span(|txt, span| (IterLoopVars::Single((txt, span.clone())), span));
    let with_index_var = ident("variable".to_string())
        .map_with_span(|txt, span| (txt, span))
        .then_ignore(just(T![","]))
        .then(
            ident("variable".to_string())
                .recover_with(skip_parser(any().or_not().map(|_| "".to_string())))
                .map_with_span(|txt, span| (txt, span)),
        )
        .map_with_span(|(var, index), span| (IterLoopVars::WithIndex(var, index), span));

    just(T!["loop"])
        .ignore_then(with_index_var.or(single_var).recover_with(skip_parser(
            none_of([T!["in"]]).map_with_span(|_, span| (IterLoopVars::Error, span)),
        )))
        .then_ignore(just(T!["in"]).recover_with(skip_parser(any().or_not().map(|_| T!["in"]))))
        .then(
            parse_expr(stmnts.clone()).recover_with(skip_parser(
                any()
                    .or_not()
                    .map_with_span(|_, span| (Expression::Error, span)),
            )),
        )
        .then(block_parser(stmnts).recover_with(skip_parser(
            any().or_not().map_with_span(|_, span| (Block::Error, span)),
        )))
        .map_with_span(|((vars, expr), body), span| {
            (Statement::IterLoop(vars, Box::new(expr), body), span)
        })
}
