use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{expressions::parse_expr, Spanned, Statement};

pub fn keywords_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    keyword("break")
        .map_with_span(|_, span| (Statement::Break, span))
        .or(keyword("continue").map_with_span(|_, span| (Statement::Continue, span)))
        .or(keyword("return")
            .ignore_then(
                filter(|c: &char| c.is_whitespace())
                    .repeated()
                    .ignore_then(parse_expr(stmnts.clone()))
                    .or_not(),
            )
            .map_with_span(|expr, span| (Statement::Return(expr.map(Box::new)), span)))
        .or(keyword("fail")
            .ignore_then(
                filter(|c: &char| c.is_whitespace())
                    .repeated()
                    .ignore_then(parse_expr(stmnts.clone()))
                    .or_not(),
            )
            .map_with_span(|expr, span| (Statement::Fail(expr.map(Box::new)), span)))
        .or(keyword("echo")
            .ignore_then(
                filter(|c: &char| c.is_whitespace())
                    .repeated()
                    .ignore_then(parse_expr(stmnts)),
            )
            .map_with_span(|expr, span| (Statement::Echo(Box::new(expr)), span)))
}
