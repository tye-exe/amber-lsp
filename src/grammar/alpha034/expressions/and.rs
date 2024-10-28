use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, Expression, Spanned, Statement},
    T,
};

use super::comparison::comparison_parser;

pub fn and_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>> {
    comparison_parser(stmnts.clone(), expr.clone()).foldl(
        just(T!["and"])
            .ignore_then(
                comparison_parser(stmnts, expr).recover_with(via_parser(
                    any()
                        .or_not()
                        .map_with(|_, e| (Expression::Error, e.span())),
                )),
            )
            .repeated(),
        |lhs: Spanned<Expression>, rhs: Spanned<Expression>| {
            let span = SimpleSpan::new(lhs.1.start, rhs.1.end);

            (Expression::And(Box::new(lhs), Box::new(rhs)), span)
        },
    )
}
