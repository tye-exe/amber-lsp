use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{Expression, Statement};

use super::atom::atom_parser;

pub fn unary_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    just('-')
        .padded()
        .to(Expression::Neg as fn(_) -> _)
        .or(keyword("not").padded().to(Expression::Not as fn(_) -> _))
        .or(keyword("nameof")
            .padded()
            .to(Expression::Nameof as fn(_) -> _))
        .repeated()
        .then(atom_parser(stmnts, expr.clone()))
        .foldr(|op, expr| op(Box::new(expr)))
}
