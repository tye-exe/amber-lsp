use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned, Statement},
    T,
};

use super::is::is_parser;

pub fn product_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    is_parser(stmnts.clone(), expr.clone())
        .then(
            just(T!['*'])
                .to(Expression::Multiply as fn(_, _) -> _)
                .or(just(T!['/']).to(Expression::Divide as fn(_, _) -> _))
                .or(just(T!['%']).to(Expression::Modulo as fn(_, _) -> _))
                .then(is_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.1.start..rhs.1.end;

            (op(Box::new(lhs), Box::new(rhs)), span)
        })
}
