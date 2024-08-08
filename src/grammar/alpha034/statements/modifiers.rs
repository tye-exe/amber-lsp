use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{CommandModifier, Spanned, Statement};

pub fn modifier_parser() -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> {
    keyword("unsafe")
        .map_with_span(|_, span| (CommandModifier::Unsafe, span))
        .or(keyword("silent").map_with_span(|_, span| (CommandModifier::Silent, span)))
        .map_with_span(|modifier, span| (Statement::CommandModifier(modifier), span))
}
