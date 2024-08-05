use chumsky::prelude::*;

use crate::grammar::alpha034::{
    statements::failed::failure_parser, Expression, InterpolatedCommand, Statement,
};

pub fn command_parser<'a>(
    stmnts: Recursive<'a, char, Statement, Simple<char>>,
    expr: Recursive<'a, char, Expression, Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    let escape = just('\\')
        .ignore_then(any())
        .map(|char| InterpolatedCommand::Escape(char.to_string()));

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
        .map(|(dashes, option)| {
            InterpolatedCommand::CommandOption(format!("{}{}", dashes, option))
        });

    let interpolated = expr
        .clone()
        .padded()
        .delimited_by(just('{'), just('}'))
        .map(|expr| InterpolatedCommand::Expression(Box::new(expr)));

    just('$')
        .padded()
        .ignore_then(
            filter::<_, _, Simple<char>>(|c: &char| {
                *c != '$' && *c != '{' && *c != '}' && *c != '\\' && *c != '-'
            })
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(InterpolatedCommand::Text)
            .or(escape)
            .or(command_option)
            .or(interpolated)
            .repeated(),
        )
        .then_ignore(just('$'))
        .padded()
        .then(failure_parser(stmnts).or_not())
        .map(|(expr, handler)| Expression::Command(expr, handler))
}
