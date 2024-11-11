use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{
        expressions::parse_expr, lexer::Token, AmberParser, Expression, Spanned, Statement,
    },
    T,
};

pub fn keywords_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<Statement>> {
    choice((
        just(T!["break"]).map_with(|_, e| (Statement::Break, e.span())),
        just(T!["continue"]).map_with(|_, e| (Statement::Continue, e.span())),
        just(T!["return"])
            .ignore_then(parse_expr(stmnts.clone()).or_not())
            .map_with(|expr, e| (Statement::Return(expr.map(Box::new)), e.span())),
        just(T!["fail"])
            .ignore_then(parse_expr(stmnts.clone()).or_not())
            .map_with(|expr, e| (Statement::Fail(expr.map(Box::new)), e.span())),
        just(T!["echo"])
            .ignore_then(
                parse_expr(stmnts).recover_with(via_parser(
                    any()
                        .or_not()
                        .map_with(|_, e| (Expression::Error, e.span())),
                )),
            )
            .map_with(|expr, e| (Statement::Echo(Box::new(expr)), e.span())),
    ))
    .boxed()
}
