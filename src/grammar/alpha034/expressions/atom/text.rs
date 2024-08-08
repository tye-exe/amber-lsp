use crate::grammar::alpha034::{Expression, InterpolatedText, Spanned};
use chumsky::prelude::*;

pub fn text_parser(
    expr: Recursive<char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + '_ {
    let escaped = just::<_, _, Simple<char>>('\\')
        .ignore_then(any())
        .map_with_span(|char, span| InterpolatedText::Escape((char.to_string(), span)));

    let interpolated = expr
        .padded()
        .delimited_by(just('{'), just('}'))
        .map(|expr| InterpolatedText::Expression(Box::new(expr)));

    just('"')
        .ignore_then(
            filter::<_, _, Simple<char>>(|c: &char| {
                *c != '"' && *c != '{' && *c != '}' && *c != '\\'
            })
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map_with_span(|text, span| InterpolatedText::Text((text, span)))
            .or(escaped)
            .or(interpolated)
            .map_with_span(|expr, span| (expr, span))
            .repeated(),
        )
        .then_ignore(just('"'))
        .map_with_span(|expr, span| (Expression::Text(expr), span))
}
