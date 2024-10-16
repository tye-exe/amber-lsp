use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{
        lexer::Token, statements::failed::failure_parser, Expression, InterpolatedCommand, Spanned,
        Statement,
    },
    T,
};

pub fn command_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    let escape = just(T!['\\'])
        .ignore_then(any())
        .map(|token| InterpolatedCommand::Escape(token.to_string()));

    let command_option = just(T!["-"])
        .then(just(T!["-"]).or_not())
        .then(any().or_not())
        .map(|((_, second_dash), option)| {
            let dashes = if second_dash.is_some() { "--" } else { "-" };

            InterpolatedCommand::CommandOption(format!(
                "{}{}",
                dashes,
                option.unwrap_or(T![""]).to_string()
            ))
        });

    let interpolated = expr
        .recover_with(skip_parser(
            any()
                .or_not()
                .map_with_span(|_, span| (Expression::Error, span)),
        ))
        .delimited_by(
            just(T!['{']),
            just(T!['}']).recover_with(skip_parser(
                none_of(T!["}"])
                    .repeated()
                    .then(just(T!['}']))
                    .or_not()
                    .map(|_| T!['}']),
            )),
        )
        .map(|expr| InterpolatedCommand::Expression(Box::new(expr)));

    just(T!['$'])
        .ignore_then(
            filter::<_, _, Simple<Token>>(|c: &Token| {
                *c != T!["$"] && *c != T!["{"] && *c != T!["}"] && *c != T!["\\"] && *c != T!["-"]
            })
            .map(|text| InterpolatedCommand::Text(text.to_string()))
            .or(escape)
            .or(command_option)
            .or(interpolated)
            .map_with_span(|cmd, span| (cmd, span))
            .repeated(),
        )
        .then_ignore(just(T!['$']).recover_with(skip_parser(any().or_not().map(|_| T!['$']))))
        .then(failure_parser(stmnts).or_not())
        .map_with_span(|(expr, handler), span| (Expression::Command(expr, handler), span))
}
