use chumsky::prelude::*;

use crate::{
    grammar::alpha040::{lexer::Token, AmberParser, CommandModifier, Spanned},
    T,
};

pub fn modifier_parser<'a>() -> impl AmberParser<'a, Spanned<CommandModifier>> {
    choice((
        just(T!["trust"]).to(CommandModifier::Trust),
        just(T!["unsafe"]).to(CommandModifier::Unsafe),
        just(T!["silent"]).to(CommandModifier::Silent),
    ))
    .map_with(|modifier, e| (modifier, e.span()))
    .labelled("modifier")
    .boxed()
}
