use crate::grammar::alpha034::{Expression, Spanned};
use chumsky::prelude::*;
use text::keyword;

pub fn null_parser() -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> {
    keyword::<_, _, Simple<char>>("null").map_with_span(|_, span| (Expression::Null, span))
}
