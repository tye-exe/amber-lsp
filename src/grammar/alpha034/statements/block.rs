use chumsky::prelude::*;

use crate::grammar::alpha034::{Block, Spanned, Statement};

pub fn block_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Block>, Error = Simple<char>> + '_ {
    just("{")
        .ignore_then(stmnts.clone().padded().repeated())
        .then_ignore(just("}"))
        .map_with_span(|body, span| (Block::Block(body), span))
}

pub fn block_parser_statement(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    block_parser(stmnts).map_with_span(|block, span| (Statement::Block(block), span))
}
