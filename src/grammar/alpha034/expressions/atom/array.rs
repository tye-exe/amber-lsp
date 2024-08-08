use chumsky::prelude::*;

use crate::grammar::alpha034::{Expression, Spanned};

pub fn array_parser(
    expr: Recursive<char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + '_ {
    just('[')
        .ignore_then(expr.padded().separated_by(just(',')))
        .then_ignore(just(']'))
        .map_with_span(|arr, span| (Expression::Array(arr), span))
}
