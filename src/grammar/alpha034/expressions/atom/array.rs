use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned},
    T,
};

pub fn array_parser(
    expr: Recursive<Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + '_ {
    just(T!['['])
        .ignore_then(expr.separated_by(just(T![","])))
        .then_ignore(just(T!["]"]))
        .map_with_span(|arr, span| (Expression::Array(arr), span))
}
