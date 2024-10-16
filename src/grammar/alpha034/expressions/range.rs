use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned, Statement},
    T,
};

use super::or::or_parser;

pub fn range_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    or_parser(stmnts.clone(), expr.clone())
        .then(
            just(T![".."])
                .ignore_then(just(T!["="]).or_not())
                .ignore_then(
                    or_parser(stmnts, expr).recover_with(skip_parser(
                        any()
                            .or_not()
                            .map_with_span(|_, span| (Expression::Error, span)),
                    )),
                )
                .repeated(),
        )
        .foldl(|start, end| {
            let span = start.1.start..end.1.end;

            (Expression::Range(Box::new(start), Box::new(end)), span)
        })
}
