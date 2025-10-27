use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, Expression, Spanned},
    T,
};
use chumsky::prelude::*;

pub fn bool_parser<'a>() -> impl AmberParser<'a, Spanned<Expression>> {
    choice((just(T!["true"]).to(true), just(T!["false"]).to(false)))
        .map_with(|b, e| (Expression::Boolean((b, e.span())), e.span()))
        .boxed()
        .labelled("boolean")
}
