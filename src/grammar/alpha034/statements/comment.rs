use chumsky::prelude::*;

use crate::grammar::alpha034::Statement;

pub fn comment_parser() -> impl Parser<char, Statement, Error = Simple<char>> {
    just("//")
        .padded()
        .ignore_then(filter(|c: &char| *c != '\n').repeated().collect())
        .map(Statement::Comment)
}
