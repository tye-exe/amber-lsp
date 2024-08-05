use chumsky::prelude::*;

use crate::grammar::alpha034::{Block, Statement};

pub fn block_parser(stmnts: Recursive<char, Statement, Simple<char>>) -> impl Parser<char, Block, Error = Simple<char>> + '_ {
    just("{")
        .padded()
        .ignore_then(stmnts.clone().repeated())
        .then_ignore(just("}"))
        .padded()
        .map(|body| Block::Block(body))
}

pub fn block_parser_statement(stmnts: Recursive<char, Statement, Simple<char>>) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    block_parser(stmnts)
        .map(|block| Statement::Block(block))
}