use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned, Statement},
    T,
};

use super::atom::atom_parser;

pub fn unary_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    just(T!['-'])
        .to(Expression::Neg as fn(_) -> _)
        .or(just(T!["not"]).to(Expression::Not as fn(_) -> _))
        .or(just(T!["nameof"]).to(Expression::Nameof as fn(_) -> _))
        .repeated()
        .then(atom_parser(stmnts, expr))
        .foldr(
            |op: fn(Box<(Expression, std::ops::Range<usize>)>) -> Expression, expr| {
                let span = expr.1.start..expr.1.end;

                (op(Box::new(expr)), span)
            },
        )
}
