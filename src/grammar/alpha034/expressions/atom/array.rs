use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, Expression, Spanned},
    T,
};

pub fn array_parser<'a>(
    expr: impl AmberParser<'a, Spanned<Expression>>
) -> impl AmberParser<'a, Spanned<Expression>> {
    expr.recover_with(via_parser(
        none_of([T![']']]).map_with(|_, e| (Expression::Error, e.span())),
    ))
    .separated_by(
        just(T![","]).recover_with(via_parser(none_of([T!["]"]]).rewind().map(|_| T![","]))),
    )
    .collect()
    .delimited_by(
        just(T!["["]),
        just(T!["]"]).recover_with(via_parser(any().or_not().map(|_| T!["]"]))),
    )
    .map_with(move |arr, e| (Expression::Array(arr), e.span()))
}
