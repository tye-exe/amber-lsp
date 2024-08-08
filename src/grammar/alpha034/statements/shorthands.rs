use chumsky::prelude::*;
use text::ident;

use crate::grammar::alpha034::{expressions::parse_expr, Spanned, Statement};

pub fn shorthand_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    ident()
        .map_with_span(|name, span| (name, span))
        .then(
            just("+=")
                .to(Statement::ShorthandAdd as fn(_, _) -> _)
                .or(just("-=").to(Statement::ShorthandSub as fn(_, _) -> _))
                .or(just("*=").to(Statement::ShorthandMul as fn(_, _) -> _))
                .or(just("/=").to(Statement::ShorthandDiv as fn(_, _) -> _))
                .or(just("%=").to(Statement::ShorthandModulo as fn(_, _) -> _))
                .padded(),
        )
        .then(parse_expr(stmnts))
        .map_with_span(|((name, op), value), span| (op(name, Box::new(value)), span))
}
