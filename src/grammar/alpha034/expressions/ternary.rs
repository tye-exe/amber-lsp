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
                .ignore_then(expr.clone())
                .then_ignore(just(T!["else"]))
                .then(expr)
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
