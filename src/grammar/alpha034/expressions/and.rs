use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{Expression, Spanned, Statement};

use super::comparison::comparison_parser;

pub fn and_parser<'a>(
    stmnts: Recursive<'a, char, Spanned<Statement>, Simple<char>>,
    expr: Recursive<'a, char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + 'a {
    comparison_parser(stmnts.clone(), expr.clone())
        .then(
            keyword("and")
                .ignore_then(comparison_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|lhs: Spanned<Expression>, rhs: Spanned<Expression>| {
            let span = lhs.1.start..rhs.1.end;

            (Expression::And(Box::new(lhs), Box::new(rhs)), span)
        })
}
