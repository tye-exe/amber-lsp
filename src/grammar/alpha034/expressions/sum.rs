use chumsky::prelude::*;

use crate::grammar::alpha034::{Expression, Statement};

use super::product::product_parser;

pub fn sum_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    product_parser(stmnts.clone(), expr.clone())
        .then(
            just('+')
                .padded()
                .to(Expression::Add as fn(_, _) -> _)
                .or(just('-').padded().to(Expression::Subtract as fn(_, _) -> _))
                .then(product_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}
