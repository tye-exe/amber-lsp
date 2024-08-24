use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned},
    T,
};
use chumsky::prelude::*;

pub fn status_var_parser() -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> {
    just(T!["status"]).map_with_span(|_, span| (Expression::Status, span))
}
