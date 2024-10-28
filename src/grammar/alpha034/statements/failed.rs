use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, FailureHandler, Spanned, Statement},
    T,
};

pub fn failure_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<FailureHandler>> {
    let handle_parser = just(T!["failed"])
        .ignore_then(just(T!["{"]).recover_with(via_parser(any().or_not().map(|_| T!["{"]))))
        .ignore_then(
            stmnts
                .recover_with(via_parser(
                    none_of([T!["}"]]).map_with(|_, e| (Statement::Error, e.span())),
                ))
                .repeated()
                .collect(),
        )
        .then_ignore(just(T!["}"]).recover_with(via_parser(any().or_not().map(|_| T!["}"]))))
        .map(|block| FailureHandler::Handle(block));

    let prop_parser = just(T!['?']).map(|_| FailureHandler::Propagate);

    handle_parser
        .or(prop_parser)
        .map_with(|handler, e| (handler, e.span()))
}
