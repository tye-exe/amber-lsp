use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, parser::ident, AmberParser, Expression, Spanned, Statement},
    T,
};

use super::unary::unary_parser;

pub fn cast_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>> {
    unary_parser(stmnts, expr).foldl(
        just(T!["as"])
            .ignore_then(
                ident("type".to_string())
                    .recover_with(via_parser(any().or_not().map(|_| "".to_string())))
                    .map_with(|txt, e| (txt, e.span())),
            )
            .repeated(),
        |expr, cast| {
            let span = SimpleSpan::new(expr.1.start, cast.1.end);

            (Expression::Cast(Box::new(expr), cast), span)
        },
    )
}
