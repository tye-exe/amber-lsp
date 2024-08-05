use chumsky::prelude::*;

use super::{Expression, Statement};

mod and;
mod ternary;
mod or;
mod atom;
mod cast;
mod comparison;
mod is;
mod product;
mod range;
mod sum;
mod unary;

pub fn parse_expr(stmnts: Recursive<char, Statement, Simple<char>>) -> impl Parser<char, Expression, Error = Simple<char>> + '_ {
    recursive(|expr: Recursive<char, Expression, Simple<char>>| {
        ternary::ternary_parser(stmnts, expr)
    })
}