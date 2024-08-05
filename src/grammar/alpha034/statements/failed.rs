use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{FailureHandler, Statement};

pub fn failure_parser(
    stmnts: Recursive<char, Statement, Simple<char>>,
) -> impl Parser<char, FailureHandler, Error = Simple<char>> + '_ {
    let handle_parser = keyword("failed")
        .padded()
        .ignore_then(just("{").padded())
        .ignore_then(stmnts.repeated())
        .then_ignore(just("}").padded())
        .map(|block| FailureHandler::Handle(block));

    let prop_parser = just('?').padded().map(|_| FailureHandler::Propagate);

    handle_parser.or(prop_parser)
}
