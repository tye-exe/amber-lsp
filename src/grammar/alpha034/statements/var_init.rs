use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{
        expressions::parse_expr, lexer::Token, parser::ident, AmberParser, Expression, Spanned,
        Statement,
    },
    T,
};

pub fn var_init_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<Statement>> {
    just(T!["let"])
        .ignore_then(
            ident("variable".to_string())
                .recover_with(via_parser(any().or_not().map(|_| "".to_string())))
                .map_with(|name, e| (name, e.span())),
        )
        .then_ignore(just(T!["="]).recover_with(via_parser(any().or_not().map(|_| T!["="]))))
        .then(
            parse_expr(stmnts).recover_with(via_parser(
                any()
                    .or_not()
                    .map_with(|_, e| (Expression::Error, e.span())),
            )),
        )
        .map_with(|(name, value), e| (Statement::VariableInit(name, Box::new(value)), e.span()))
        .boxed()
}
