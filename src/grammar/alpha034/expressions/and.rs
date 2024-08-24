use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned, Statement},
    T,
};

use super::comparison::comparison_parser;

pub fn and_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    comparison_parser(stmnts.clone(), expr.clone())
        .then(
            just(T!["and"])
                .ignore_then(comparison_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|lhs: Spanned<Expression>, rhs: Spanned<Expression>| {
            let span = lhs.1.start..rhs.1.end;

            (Expression::And(Box::new(lhs), Box::new(rhs)), span)
        })
}
