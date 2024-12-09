use chumsky::prelude::*;

use crate::grammar::alpha034::lexer::Token;
use crate::grammar::alpha034::{AmberParser, Spanned, Statement};
use crate::T;

use super::range::range_parser;
use super::Expression;

pub fn ternary_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>> {
    range_parser(stmnts, expr.clone())
        .foldl(
            just(T!["then"])
                .ignore_then(
                    expr.clone().recover_with(via_parser(
                        any()
                            .or_not()
                            .map_with(|_, e| (Expression::Error, e.span())),
                    )),
                )
                .then_ignore(
                    just(T!["else"]).recover_with(via_parser(any().or_not().map(|_| T![""]))),
                )
                .then(
                    expr.recover_with(via_parser(
                        any()
                            .or_not()
                            .map_with(|_, e| (Expression::Error, e.span())),
                    )),
                )
                .repeated(),
            |cond, (if_true, if_false)| {
                let span = SimpleSpan::new(cond.1.start, if_false.1.end);

                (
                    Expression::Ternary(Box::new(cond), Box::new(if_true), Box::new(if_false)),
                    span,
                )
            },
        )
        .boxed()
}
