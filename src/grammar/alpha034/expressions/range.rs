use chumsky::prelude::*;

use crate::grammar::alpha034::{Expression, Spanned, Statement};

use super::or::or_parser;

pub fn range_parser<'a>(
    stmnts: Recursive<'a, char, Spanned<Statement>, Simple<char>>,
    expr: Recursive<'a, char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + 'a {
    or_parser(stmnts.clone(), expr.clone())
        .then(
            just("..")
                .ignore_then(just("=").or_not())
                .ignore_then(or_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|start, end| {
            let span = start.1.start..end.1.end;

            (Expression::Range(Box::new(start), Box::new(end)), span)
        })
}
