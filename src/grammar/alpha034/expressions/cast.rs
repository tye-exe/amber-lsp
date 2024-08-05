use chumsky::prelude::*;
use text::{ident, keyword};

use crate::grammar::alpha034::{Expression, Statement};

use super::unary::unary_parser;

pub fn cast_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    unary_parser(stmnts, expr)
        .then(
            keyword("as")
                .ignore_then(ident::<_, Simple<char>>().padded())
                .repeated(),
        )
        .padded()
        .foldl(|expr, cast| Expression::Cast(Box::new(expr), cast))
}
