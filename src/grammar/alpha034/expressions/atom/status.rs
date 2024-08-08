use crate::grammar::alpha034::{Expression, Spanned};
use chumsky::prelude::*;
use text::keyword;

pub fn status_var_parser() -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> {
    keyword("status").map_with_span(|_, span| (Expression::Status, span))
}
