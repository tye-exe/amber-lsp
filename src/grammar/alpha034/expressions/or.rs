use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::Statement;

use super::{and::and_parser, Expression};

pub fn or_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    and_parser(stmnts.clone(), expr.clone())
        .then(
            keyword("or")
                .ignore_then(and_parser(stmnts, expr))
                .padded()
                .repeated(),
        )
        .foldl(|lhs, rhs| Expression::Or(Box::new(lhs), Box::new(rhs)))
}
