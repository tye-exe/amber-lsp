use crate::grammar::alpha034::Spanned;

use super::super::Expression;
use chumsky::prelude::*;

pub fn parentheses_parser(
    expr: Recursive<char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + '_ {
    just('(')
        .ignore_then(expr)
        .then_ignore(just(')'))
        .map_with_span(|expr, span| (Expression::Parentheses(Box::new(expr)), span))
}
