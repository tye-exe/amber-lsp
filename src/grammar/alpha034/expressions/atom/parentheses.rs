use chumsky::prelude::*;
use super::super::Expression;

pub fn parentheses_parser(expr: Recursive<char, Expression, Simple<char>>) -> impl Parser<char, Expression, Error = Simple<char>> + '_ {
    just('(')
        .ignore_then(expr)
        .then_ignore(just(')'))
        .map(|expr| Expression::Parentheses(Box::new(expr)))
}
