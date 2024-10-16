use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{
        expressions::parse_expr, lexer::Token, parser::ident, Expression, Spanned, Statement,
    },
    T,
};

pub fn var_set_parser(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> + '_ {
    ident("variable".to_string())
        .map_with_span(|name, span| (name, span))
        .then_ignore(just(T!["="]))
        .then(
            parse_expr(stmnts).recover_with(skip_parser(
                any()
                    .or_not()
                    .map_with_span(|_, span| (Expression::Error, span)),
            )),
        )
        .map_with_span(|(name, value), span| (Statement::VariableSet(name, Box::new(value)), span))
}
