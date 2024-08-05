use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{CommandModifier, Statement};

pub fn modifier_parser() -> impl Parser<char, Statement, Error = Simple<char>> {
    keyword("unsafe").to(CommandModifier::Unsafe)
        .or(keyword("silent").to(CommandModifier::Silent))
        .padded()
        .map(Statement::CommandModifier)
}