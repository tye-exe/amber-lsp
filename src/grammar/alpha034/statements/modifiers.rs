use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, CommandModifier, Spanned, Statement},
    T,
};

pub fn modifier_parser<'a>() -> impl AmberParser<'a, Spanned<Statement>> {
    just(T!["unsafe"])
        .map_with(|_, e| (CommandModifier::Unsafe, e.span()))
        .or(just(T!["silent"]).map_with(|_, e| (CommandModifier::Silent, e.span())))
        .map_with(|modifier, e| (Statement::CommandModifier(modifier), e.span()))
}
