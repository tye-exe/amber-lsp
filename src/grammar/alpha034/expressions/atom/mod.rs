use crate::{
    grammar::alpha034::{lexer::Token, Spanned, Statement},
    T,
};

use super::super::Expression;
use chumsky::prelude::*;

mod array;
mod bool;
mod call;
mod command;
mod null;
mod number;
mod parentheses;
mod status;
mod text;
mod var;

pub fn atom_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    bool::bool_parser()
        .or(null::null_parser())
        .or(status::status_var_parser())
        .or(call::function_call_parser(stmnts.clone(), expr.clone()))
        .or(var::var_parser())
        .or(text::text_parser(expr.clone()))
        .or(array::array_parser(expr.clone()))
        .or(command::command_parser(stmnts, expr.clone()))
        .or(number::number_parser())
        .or(parentheses::parentheses_parser(expr))
        .then_ignore(just(T![';']).or_not())
}
