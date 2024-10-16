use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned},
    T,
};

pub fn array_parser(
    expr: Recursive<Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + '_ {
    expr.recover_with(skip_parser(
        none_of([T![']']]).map_with_span(|_, span| (Expression::Error, span)),
    ))
    .separated_by(
        just(T![","]).recover_with(skip_parser(none_of([T!["]"]]).rewind().map(|_| T![","]))),
    )
    .delimited_by(
        just(T!["["]),
        just(T!["]"]).recover_with(skip_parser(
            any().or_not().map(|_| T!["]"]),
        )),
    )
    .map_with_span(|arr, span| (Expression::Array(arr), span))
}
