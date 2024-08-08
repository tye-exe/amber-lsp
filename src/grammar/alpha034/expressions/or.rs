use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{Spanned, Statement};

use super::{and::and_parser, Expression};

pub fn or_parser<'a>(
    stmnts: Recursive<'a, char, Spanned<Statement>, Simple<char>>,
    expr: Recursive<'a, char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + 'a {
    and_parser(stmnts.clone(), expr.clone())
        .then(
            keyword("or")
                .padded()
                .ignore_then(and_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|lhs, rhs| {
            let span = lhs.1.start..rhs.1.end;

            (Expression::Or(Box::new(lhs), Box::new(rhs)), span)
        })
}
