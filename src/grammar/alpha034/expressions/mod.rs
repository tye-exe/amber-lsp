use chumsky::prelude::*;

use super::{lexer::Token, Expression, Spanned, Statement};

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
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + '_ {
    recursive(
        |expr: Recursive<Token, Spanned<Expression>, Simple<Token>>| {
            ternary::ternary_parser(stmnts, expr)
        },
    ).labelled("expression")
}
