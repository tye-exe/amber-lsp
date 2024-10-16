use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Block, Spanned, Statement},
    T,
};

pub fn block_parser(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Block>, Error = Simple<Token>> + '_ {
    stmnts
        .recover_with(skip_parser(
            none_of([T!['}']])
                .map_with_span(|_, span| (Statement::Error, span)),
        ))
        .repeated()
        .delimited_by(
            just(T!['{']),
            just(T!['}']).recover_with(skip_parser(any().or_not().map(|_| T!['}']))),
        )
        .map_with_span(|body, span| (Block::Block(body), span))
}

pub fn block_parser_statement(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Statement>, Error = Simple<Token>> + '_ {
    block_parser(stmnts).map_with_span(|block, span| (Statement::Block(block), span))
}
