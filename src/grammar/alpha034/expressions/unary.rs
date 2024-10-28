use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, Expression, Spanned, Statement},
    T,
};

use super::atom::atom_parser;

pub fn unary_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>> {
    just(T!['-'])
        .to(Expression::Neg as fn(_) -> _)
        .or(just(T!["not"]).to(Expression::Not as fn(_) -> _))
        .or(just(T!["nameof"]).to(Expression::Nameof as fn(_) -> _))
        .repeated()
        .foldr(
            atom_parser(stmnts, expr),
            |op: fn(Box<Spanned<Expression>>) -> Expression, expr| {
                let span = SimpleSpan::new(expr.1.start, expr.1.end);

                (op(Box::new(expr)), span)
            },
        )
}
