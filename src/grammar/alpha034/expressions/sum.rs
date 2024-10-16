use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned, Statement},
    T,
};

use super::product::product_parser;

pub fn sum_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    product_parser(stmnts.clone(), expr.clone())
        .then(
            just(T!['+'])
                .to(Expression::Add as fn(_, _) -> _)
                .or(just(T!['-']).to(Expression::Subtract as fn(_, _) -> _))
                .then(
                    product_parser(stmnts, expr).recover_with(skip_parser(
                        any()
                            .or_not()
                            .map_with_span(|_, span| (Expression::Error, span)),
                    )),
                )
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.1.start..rhs.1.end;
            (op(Box::new(lhs), Box::new(rhs)), span)
        })
}
