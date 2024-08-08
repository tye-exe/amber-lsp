use chumsky::prelude::*;

use crate::grammar::alpha034::{Spanned, Statement};

pub fn comment_parser() -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> {
    just("//")
        .ignore_then(
            filter(|c: &char| *c != '\n')
                .repeated()
                .collect()
                .map_with_span(|text, span| (text, span)),
        )
        .map_with_span(|com, span| (Statement::Comment(com), span))
}
