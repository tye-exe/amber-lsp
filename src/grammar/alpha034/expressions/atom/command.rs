use chumsky::prelude::*;

use crate::grammar::alpha034::{
    statements::failed::failure_parser, Expression, InterpolatedCommand, Spanned, Statement,
};

pub fn command_parser<'a>(
    stmnts: Recursive<'a, char, Spanned<Statement>, Simple<char>>,
    expr: Recursive<'a, char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + 'a {
    let escape = just('\\')
        .ignore_then(any())
        .map_with_span(|char, span| InterpolatedCommand::Escape((char.to_string(), span)));

    let command_option = just("--")
        .or(just("-"))
        .then(
            filter(|c: &char| c.is_ascii_alphabetic())
                .chain(
                    filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                        .repeated(),
                )
                .collect::<String>(),
        )
        .map_with_span(|(dashes, option), span| {
            InterpolatedCommand::CommandOption((format!("{}{}", dashes, option), span))
        });

    let interpolated = expr
        .padded()
        .delimited_by(just('{'), just('}'))
        .map(|expr| InterpolatedCommand::Expression(Box::new(expr)));

    just('$')
        .ignore_then(
            filter::<_, _, Simple<char>>(|c: &char| {
                *c != '$' && *c != '{' && *c != '}' && *c != '\\' && *c != '-'
            })
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map_with_span(|text, span| InterpolatedCommand::Text((text, span)))
            .or(escape)
            .or(command_option)
            .or(interpolated)
            .map_with_span(|cmd, span| (cmd, span))
            .repeated(),
        )
        .then_ignore(just('$'))
        .then(failure_parser(stmnts).or_not())
        .map_with_span(|(expr, handler), span| (Expression::Command(expr, handler), span))
}
