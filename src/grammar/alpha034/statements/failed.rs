use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, FailureHandler, Spanned, Statement},
    T,
};

pub fn failure_parser(
    stmnts: Recursive<Token, Spanned<Statement>, Simple<Token>>,
) -> impl Parser<Token, Spanned<FailureHandler>, Error = Simple<Token>> + '_ {
    let handle_parser = just(T!["failed"])
        .ignore_then(just(T!["{"]).recover_with(skip_parser(any().or_not().map(|_| T!["{"]))))
        .ignore_then(
            stmnts
                .recover_with(skip_parser(
                    none_of([T!["}"]])
                        .map_with_span(|_, span| (Statement::Error, span)),
                ))
                .repeated(),
        )
        .then_ignore(just(T!["}"]).recover_with(skip_parser(any().or_not().map(|_| T!["}"]))))
        .map(|block| FailureHandler::Handle(block));

    let prop_parser = just(T!['?']).map(|_| FailureHandler::Propagate);

    handle_parser
        .or(prop_parser)
        .map_with_span(|handler, span| (handler, span))
}
