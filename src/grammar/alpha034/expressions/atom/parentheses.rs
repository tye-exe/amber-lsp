use crate::{
    grammar::alpha034::{lexer::Token, Spanned},
    T,
};

use super::super::Expression;
use chumsky::prelude::*;

pub fn parentheses_parser(
    expr: Recursive<Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + '_ {
    expr.delimited_by(just(T!['(']), just(T![')']))
        .map_with_span(|expr, span| (Expression::Parentheses(Box::new(expr)), span))
}
