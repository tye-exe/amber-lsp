use crate::{grammar::alpha034::{
    lexer::Token, parser::ident, statements::failed::failure_parser, Expression, Spanned, Statement
}, T};
use chumsky::prelude::*;

pub fn function_call_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    ident()
        .map_with_span(|name, span| (name, span))
        .then_ignore(just(T!['(']))
        .then(expr.separated_by(just(T![','])))
        .then_ignore(just(T![')']))
        .then(failure_parser(stmnts).or_not())
        .map_with_span(|((name, args), handler), span| {
            (Expression::FunctionInvocation(name, args, handler), span)
        })
}
