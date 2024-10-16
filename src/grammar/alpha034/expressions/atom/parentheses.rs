use crate::{
    grammar::alpha034::{lexer::Token, Spanned},
    T,
};

use super::super::Expression;
use chumsky::prelude::*;

pub fn parentheses_parser(
    expr: Recursive<Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + '_ {
    expr.recover_with(skip_parser(
        any()
            .or_not()
            .map_with_span(|_, span| (Expression::Error, span)),
    ))
    .delimited_by(
        just(T!['(']),
        just(T![')']).recover_with(skip_parser(
            none_of(T![')'])
                .repeated()
                .then(just(T![')']))
                .or_not()
                .map(|_| T![')']),
        )),
    )
    .map_with_span(|expr, span| (Expression::Parentheses(Box::new(expr)), span))
}
