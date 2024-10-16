use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{
        expressions::parse_expr, lexer::Token, parser::ident, Expression, Spanned, Statement,
    },
    T,
};

pub fn shorthand_parser(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> + '_ {
    ident("variable".to_string())
        .map_with_span(|name, span| (name, span))
        .then(
            just(T!["+="])
                .to(Statement::ShorthandAdd as fn(_, _) -> _)
                .or(just(T!["-="]).to(Statement::ShorthandSub as fn(_, _) -> _))
                .or(just(T!["*="]).to(Statement::ShorthandMul as fn(_, _) -> _))
                .or(just(T!["/="]).to(Statement::ShorthandDiv as fn(_, _) -> _))
                .or(just(T!["%="]).to(Statement::ShorthandModulo as fn(_, _) -> _)),
        )
        .then(
            parse_expr(stmnts).recover_with(skip_parser(
                any()
                    .or_not()
                    .map_with_span(|_, span| (Expression::Error, span)),
            )),
        )
        .map_with_span(|((name, op), value), span| (op(name, Box::new(value)), span))
}
