use chumsky::prelude::*;
use text::keyword;
use crate::grammar::alpha034::Expression;

pub fn status_var_parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    keyword("status").to(Expression::Status)
}