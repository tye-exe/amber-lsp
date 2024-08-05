use crate::grammar::alpha034::{statements::failed::failure_parser, Expression, Statement};
use chumsky::prelude::*;
use text::ident;

pub fn function_call_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    ident::<_, Simple<char>>()
        .padded()
        .then_ignore(just('(').padded())
        .then(expr.clone().padded().separated_by(just(',')))
        .then_ignore(just(')').padded())
        .then(failure_parser(stmnts).or_not().padded())
        .map(|((name, args), handler)| Expression::FunctionInvocation(name, args, handler))
}
