use chumsky::prelude::*;

use crate::grammar::alpha034::{Expression, Statement};

use super::is::is_parser;

pub fn product_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    is_parser(stmnts.clone(), expr.clone())
        .then(
            just('*')
                .padded()
                .to(Expression::Multiply as fn(_, _) -> _)
                .or(just('/').padded().to(Expression::Divide as fn(_, _) -> _))
                .or(just('%').padded().to(Expression::Modulo as fn(_, _) -> _))
                .then(is_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}
