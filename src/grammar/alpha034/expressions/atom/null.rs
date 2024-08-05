use crate::grammar::alpha034::Expression;
use chumsky::prelude::*;
use text::keyword;

pub fn null_parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    keyword::<_, _, Simple<char>>("null").to(Expression::Null)
}
