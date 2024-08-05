use chumsky::prelude::*;
use text::{ident, keyword};

use crate::grammar::alpha034::{Expression, Statement};

use super::cast::cast_parser;

pub fn is_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    cast_parser(stmnts, expr.clone())
        .then(
            keyword("is")
                .ignore_then(ident::<_, Simple<char>>().padded())
                .repeated(),
        )
        .padded()
        .foldl(|expr, cast| Expression::Is(Box::new(expr), cast))
}
