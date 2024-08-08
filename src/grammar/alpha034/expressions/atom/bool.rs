use crate::grammar::alpha034::{Expression, Spanned};
use chumsky::prelude::*;
use core::ops::Range;
use text::keyword;

pub fn bool_parser() -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> {
    keyword("true")
        .to(true)
        .or(keyword("false").to(false))
        .map_with_span(|b, span: Range<usize>| (Expression::Boolean((b, span.clone())), span))
        .padded()
}
