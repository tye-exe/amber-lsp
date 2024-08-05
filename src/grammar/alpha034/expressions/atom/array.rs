use chumsky::prelude::*;

use crate::grammar::alpha034::Expression;

pub fn array_parser(
    expr: Recursive<char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + '_ {
    just('[')
        .padded()
        .ignore_then(expr.clone().padded().separated_by(just(',')))
        .then_ignore(just(']').padded())
        .map(Expression::Array)
}
