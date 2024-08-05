use chumsky::prelude::*;

use crate::grammar::alpha034::{Expression, Statement};

use super::or::or_parser;

pub fn range_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    or_parser(stmnts.clone(), expr.clone())
        .then(
            just("..")
                .ignore_then(just("=").or_not())
                .ignore_then(or_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|start, end| Expression::Range(Box::new(start), Box::new(end)))
}
