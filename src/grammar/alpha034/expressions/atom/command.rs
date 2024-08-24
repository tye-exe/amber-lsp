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
        .then(any())
        .map(|((_, second_dash), option)| {
            let dashes = if second_dash.is_some() { "--" } else { "-" };

            InterpolatedCommand::CommandOption(format!("{}{}", dashes, option.to_string()))
        });
    
    let interpolated = expr
        .delimited_by(just(T!['{']), just(T!['}']))
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
        .then_ignore(just(T!['$']))
        .then(failure_parser(stmnts).or_not())
        .map_with_span(|(expr, handler), span| (Expression::Command(expr, handler), span))
}
