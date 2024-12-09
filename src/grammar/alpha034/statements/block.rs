use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, Block, Spanned, Statement},
    T,
};

pub fn block_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<Block>> {
    stmnts
        .recover_with(via_parser(
            none_of([T!['}']]).map_with(|_, e| (Statement::Error, e.span())),
        ))
        .repeated()
        .collect()
        .delimited_by(
            just(T!['{']),
            just(T!['}']).recover_with(via_parser(any().or_not().map(|_| T!['}']))),
        )
        .map_with(move |body, e| (Block::Block(body), e.span()))
        .boxed()
}

pub fn block_parser_statement<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<Statement>> {
    block_parser(stmnts)
        .map_with(|block, e| (Statement::Block(block), e.span()))
        .boxed()
}
