use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned},
    T,
};
use chumsky::prelude::*;

pub fn null_parser() -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> {
    just(T!["null"]).map_with_span(|_, span| (Expression::Null, span))
}
