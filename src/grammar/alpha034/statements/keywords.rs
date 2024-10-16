use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{expressions::parse_expr, lexer::Token, Expression, Spanned, Statement},
    T,
};

pub fn keywords_parser(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> + '_ {
    just(T!["break"])
        .map_with_span(|_, span| (Statement::Break, span))
        .or(just(T!["continue"]).map_with_span(|_, span| (Statement::Continue, span)))
        .or(just(T!["return"])
            .ignore_then(parse_expr(stmnts.clone()).or_not())
            .map_with_span(|expr, span| (Statement::Return(expr.map(Box::new)), span)))
        .or(just(T!["fail"])
            .ignore_then(parse_expr(stmnts.clone()).or_not())
            .map_with_span(|expr, span| (Statement::Fail(expr.map(Box::new)), span)))
        .or(just(T!["echo"])
            .ignore_then(
                parse_expr(stmnts).recover_with(skip_parser(
                    any()
                        .or_not()
                        .map_with_span(|_, span| Spanned::new(Expression::Error, span)),
                )),
            )
            .map_with_span(|expr, span| (Statement::Echo(Box::new(expr)), span)))
}
