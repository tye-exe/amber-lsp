use crate::grammar::alpha034::{
    statements::failed::failure_parser, Expression, Spanned, Statement,
};
use chumsky::prelude::*;
use text::ident;

pub fn function_call_parser<'a>(
    stmnts: Recursive<'a, char, Spanned<Statement>, Simple<char>>,
    expr: Recursive<'a, char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + 'a {
    ident::<_, Simple<char>>()
        .map_with_span(|name, span| (name, span))
        .then_ignore(just('(').padded())
        .then(expr.padded().separated_by(just(',')))
        .then_ignore(just(')'))
        .then(failure_parser(stmnts).or_not())
        .map_with_span(|((name, args), handler), span| {
            (Expression::FunctionInvocation(name, args, handler), span)
        })
}
