use crate::grammar::alpha034::Statement;

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
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
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
        .padded()
        .then_ignore(just(";").or_not())
}
