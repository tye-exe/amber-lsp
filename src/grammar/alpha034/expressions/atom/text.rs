use crate::{
    grammar::alpha034::{lexer::Token, Expression, InterpolatedText, Spanned},
    T,
};
use chumsky::prelude::*;

pub fn text_parser(
    expr: Recursive<Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + '_ {
    let escaped = just(T!['\\'])
        .ignore_then(any())
        .map_with_span(|char, span| InterpolatedText::Escape((char.to_string(), span)));

    let interpolated = expr
        .delimited_by(just(T!['{']), just(T!['}']))
        .map(|expr| InterpolatedText::Expression(Box::new(expr)));

    just(T!['"'])
        .ignore_then(
            filter::<_, _, Simple<Token>>(|c: &Token| {
                *c != T!['"'] && *c != T!['{'] && *c != T!['}'] && *c != T!['\\']
            })
            .map_with_span(|text, span| InterpolatedText::Text((text.to_string(), span)))
            .or(escaped)
            .or(interpolated)
            .map_with_span(|expr, span| (expr, span))
            .repeated(),
        )
        .then_ignore(just(T!['"']))
        .map_with_span(|expr, span| (Expression::Text(expr), span))
}
