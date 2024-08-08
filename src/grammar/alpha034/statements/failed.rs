use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{FailureHandler, Spanned, Statement};

pub fn failure_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<FailureHandler>, Error = Simple<char>> + '_ {
    let handle_parser = keyword("failed")
        .ignore_then(just("{").padded())
        .ignore_then(stmnts.padded().repeated())
        .then_ignore(just("}"))
        .map(|block| FailureHandler::Handle(block));

    let prop_parser = just('?').map(|_| FailureHandler::Propagate);

    handle_parser
        .or(prop_parser)
        .map_with_span(|handler, span| (handler, span))
}
