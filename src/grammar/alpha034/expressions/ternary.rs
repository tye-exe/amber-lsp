use chumsky::prelude::*;

use crate::grammar::alpha034::lexer::Token;
use crate::grammar::alpha034::{Spanned, Statement};
use crate::T;

use super::range::range_parser;
use super::Expression;

pub fn ternary_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    range_parser(stmnts, expr.clone())
        .then(
            just(T!["then"])
                .ignore_then(
                    expr.clone().recover_with(skip_parser(
                        any()
                            .or_not()
                            .map_with_span(|_, span| (Expression::Error, span)),
                    )),
                )
                .then_ignore(
                    just(T!["else"]).recover_with(skip_parser(any().or_not().map(|_| T![""]))),
                )
                .then(
                    expr.recover_with(skip_parser(
                        any()
                            .or_not()
                            .map_with_span(|_, span| (Expression::Error, span)),
                    )),
                )
                .repeated(),
        )
        .foldl(|cond, (if_true, if_false)| {
            let span = cond.1.start..if_false.1.end;

            (
                Expression::Ternary(Box::new(cond), Box::new(if_true), Box::new(if_false)),
                span,
            )
        })
}
