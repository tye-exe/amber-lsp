use crate::{
    grammar::alpha034::{
        lexer::Token, parser::ident, statements::failed::failure_parser, Expression, Spanned,
        Statement,
    },
    T,
};
use chumsky::prelude::*;

pub fn function_call_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    ident("function".to_string())
        .map_with_span(|name, span| (name, span))
        .then_ignore(just(T!["("]))
        .then(
            expr.recover_with(skip_parser(
                none_of([T![")"]]).map_with_span(|_, span| (Expression::Error, span)),
            ))
            .separated_by(
                just(T![","])
                    .recover_with(skip_parser(none_of([T![")"]]).rewind().map(|_| T![","]))),
            ),
        )
        .then_ignore(just(T![")"]).recover_with(skip_parser(any().or_not().map(|_| T![")"]))))
        .then(failure_parser(stmnts).or_not())
        .map_with_span(|((name, args), failure), span| {
            (Expression::FunctionInvocation(name, args, failure), span)
        })
}
