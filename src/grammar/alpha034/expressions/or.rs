use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, Spanned, Statement},
    T,
};

use super::{and::and_parser, Expression};

pub fn or_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>>  {
    and_parser(stmnts.clone(), expr.clone()).foldl(
        just(T!["or"])
            .ignore_then(
                and_parser(stmnts, expr).recover_with(via_parser(
                    any()
                        .or_not()
                        .map_with(|_, e| (Expression::Error, e.span())),
                )),
            )
            .repeated(),
        |lhs, rhs| {
            let span = SimpleSpan::new(lhs.1.start, rhs.1.end);

            (Expression::Or(Box::new(lhs), Box::new(rhs)), span)
        },
    )
}
