use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, CommandModifier, Spanned, Statement},
    T,
};

pub fn modifier_parser() -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> {
    just(T!["unsafe"])
        .map_with_span(|_, span| (CommandModifier::Unsafe, span))
        .or(just(T!["silent"]).map_with_span(|_, span| (CommandModifier::Silent, span)))
        .map_with_span(|modifier, span| (Statement::CommandModifier(modifier), span))
}
