use chumsky::prelude::*;
use text::ident;

use crate::grammar::alpha034::{expressions::parse_expr, Statement};

pub fn shorthand_parser(stmnts: Recursive<char, Statement, Simple<char>>) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    ident()
        .padded()
        .then(
            just("+=").to(Statement::ShorthandAdd as fn(_, _) -> _)
                .or(just("-=").to(Statement::ShorthandSub as fn(_, _) -> _))
                .or(just("*=").to(Statement::ShorthandMul as fn(_, _) -> _))
                .or(just("/=").to(Statement::ShorthandDiv as fn(_, _) -> _))
                .or(just("%=").to(Statement::ShorthandModulo as fn(_, _) -> _)),
        )
        .padded()
        .then(parse_expr(stmnts))
        .map(|((name, op), value)| op(name, Box::new(value)))
}
