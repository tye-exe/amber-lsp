use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Spanned, Statement},
    T,
};

use super::{and::and_parser, Expression};

pub fn or_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    and_parser(stmnts.clone(), expr.clone())
        .then(
            just(T!["or"])
                .ignore_then(and_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|lhs, rhs| {
            let span = lhs.1.start..rhs.1.end;

            (Expression::Or(Box::new(lhs), Box::new(rhs)), span)
        })
}
