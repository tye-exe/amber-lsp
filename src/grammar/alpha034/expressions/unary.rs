use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{Expression, Spanned, Statement};

use super::atom::atom_parser;

pub fn unary_parser<'a>(
    stmnts: Recursive<'a, char, Spanned<Statement>, Simple<char>>,
    expr: Recursive<'a, char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + 'a {
    just('-')
        .to(Expression::Neg as fn(_) -> _)
        .or(keyword("not").to(Expression::Not as fn(_) -> _))
        .or(keyword("nameof").to(Expression::Nameof as fn(_) -> _))
        .then_ignore(filter(|c: &char| c.is_whitespace()).repeated())
        .repeated()
        .then(atom_parser(stmnts, expr))
        .foldr(|op, expr| {
            let span = expr.1.start..expr.1.end;

            (op(Box::new(expr)), span)
        })
}
