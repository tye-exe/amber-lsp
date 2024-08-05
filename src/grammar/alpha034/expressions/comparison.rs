use chumsky::prelude::*;

use crate::grammar::alpha034::{Expression, Statement};

use super::sum::sum_parser;

pub fn comparison_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    sum_parser(stmnts.clone(), expr.clone())
        .then(
            just(">=")
                .padded()
                .to(Expression::Ge as fn(_, _) -> _)
                .or(just(">").padded().to(Expression::Gt as fn(_, _) -> _))
                .or(just("<=").padded().to(Expression::Le as fn(_, _) -> _))
                .or(just("<").padded().to(Expression::Lt as fn(_, _) -> _))
                .or(just("==").padded().to(Expression::Eq as fn(_, _) -> _))
                .or(just("!=").padded().to(Expression::Neq as fn(_, _) -> _))
                .then(sum_parser(stmnts.clone(), expr.clone()))
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}
