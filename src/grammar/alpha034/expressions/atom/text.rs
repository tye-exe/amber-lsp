use crate::{
    grammar::alpha034::{lexer::Token, AmberParser, Expression, InterpolatedText, Spanned},
    T,
};
use chumsky::prelude::*;

pub fn text_parser<'a>(
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>> {
    let escaped = just(T!['\\'])
        .ignore_then(any())
        .map_with(|char: Token, e| InterpolatedText::Escape((char.to_string(), e.span())));

    let interpolated = expr
        .recover_with(via_parser(
            any()
                .or_not()
                .map_with(|_, e| (Expression::Error, e.span())),
        ))
        .delimited_by(
            just(T!['{']),
            just(T!['}']).recover_with(via_parser(
                none_of(T!["}"])
                    .repeated()
                    .then(just(T!['}']))
                    .or_not()
                    .map(|_| T!['}']),
            )),
        )
        .map(|expr| InterpolatedText::Expression(Box::new(expr)));

    just(T!['"'])
        .ignore_then(
            any()
                .filter(|c: &Token| {
                    *c != T!['"'] && *c != T!['{'] && *c != T!['}'] && *c != T!['\\']
                })
                .map_with(|text, e| InterpolatedText::Text((text.to_string(), e.span())))
                .or(escaped)
                .or(interpolated)
                .map_with(|expr, e| (expr, e.span()))
                .repeated()
                .collect(),
        )
        .then_ignore(just(T!['"']).recover_with(via_parser(any().or_not().map(|_| T!['"']))))
        .map_with(|expr, e| (Expression::Text(expr), e.span()))
}
