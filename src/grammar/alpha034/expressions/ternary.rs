use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::Statement;

use super::range::range_parser;
use super::Expression;

pub fn ternary_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    range_parser(stmnts, expr.clone())
        .then(
            keyword("then")
                .padded()
                .ignore_then(expr.clone())
                .then_ignore(keyword("else").padded())
                .then(expr)
                .padded()
                .repeated(),
        )
        .foldl(|cond, (if_true, if_false)| {
            Expression::Ternary(Box::new(cond), Box::new(if_true), Box::new(if_false))
        })
}
