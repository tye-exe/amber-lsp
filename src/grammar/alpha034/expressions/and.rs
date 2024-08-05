use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{Expression, Statement};

use super::comparison::comparison_parser;

pub fn and_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    comparison_parser(stmnts.clone(), expr.clone())
        .then(
            keyword("and")
                .ignore_then(comparison_parser(stmnts, expr))
                .padded()
                .repeated(),
        )
        .foldl(|lhs, rhs| Expression::And(Box::new(lhs), Box::new(rhs)))
}
