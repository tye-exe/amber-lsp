use chumsky::prelude::*;
use text::{ident, keyword};

use crate::grammar::alpha034::{Expression, Spanned, Statement};

use super::unary::unary_parser;

pub fn cast_parser<'a>(
    stmnts: Recursive<'a, char, Spanned<Statement>, Simple<char>>,
    expr: Recursive<'a, char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + 'a {
    unary_parser(stmnts, expr)
        .then(
            keyword("as")
                .padded()
                .ignore_then(ident::<_, Simple<char>>().map_with_span(|txt, span| (txt, span)))
                .repeated(),
        )
        .foldl(|expr, cast| {
            let span = expr.1.start..cast.1.end;

            (Expression::Cast(Box::new(expr), cast), span)
        })
}
