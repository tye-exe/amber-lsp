use chumsky::prelude::*;
use crate::grammar::alpha034::{InterpolatedText, Expression};

pub fn text_parser(expr: Recursive<char, Expression, Simple<char>>) -> impl Parser<char, Expression, Error = Simple<char>> + '_ {
    let escaped = just::<_, _, Simple<char>>('\\')
        .ignore_then(any())
        .map(|char| InterpolatedText::Escape(char.to_string()));

    let interpolated = expr.clone()
        .padded()
        .delimited_by(just('{'), just('}'))
        .map(|expr| InterpolatedText::Expression(Box::new(expr)));

    let text_literal = just('"')
        .ignore_then(
            filter::<_, _, Simple<char>>(|c: &char| *c != '"' && *c != '{' && *c != '}' && *c != '\\')
                .repeated()
                .at_least(1)
                .collect::<String>()
                .map(InterpolatedText::Text)
                .or(escaped)
                .or(interpolated)
                .repeated()
        )
        .then_ignore(just('"'))
        .map(|expr| Expression::Text(expr));

    text_literal
}