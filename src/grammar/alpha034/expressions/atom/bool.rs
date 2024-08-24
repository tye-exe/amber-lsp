use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned},
    T,
};
use chumsky::prelude::*;
use core::ops::Range;

pub fn bool_parser() -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> {
    just(T!["true"])
        .to(true)
        .or(just(T!["false"]).to(false))
        .map_with_span(|b, span: Range<usize>| (Expression::Boolean((b, span.clone())), span))
}
