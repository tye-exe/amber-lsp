use chumsky::prelude::*;

use super::{Expression, Spanned, Statement};

mod and;
mod atom;
mod cast;
mod comparison;
mod is;
mod or;
mod product;
mod range;
mod sum;
mod ternary;
mod unary;

pub fn parse_expr(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + '_ {
    recursive(|expr: Recursive<char, Spanned<Expression>, Simple<char>>| {
        ternary::ternary_parser(stmnts, expr)
    })
}
