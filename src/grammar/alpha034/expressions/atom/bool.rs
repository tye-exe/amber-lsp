use crate::grammar::alpha034::Expression;
use chumsky::prelude::*;
use text::keyword;

pub fn bool_parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    keyword("true")
        .padded()
        .to(true)
        .or(keyword("false").padded().to(false))
        .map(Expression::Boolean)
}
