use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, parser::ident, AmberParser, Expression, Spanned, Statement},
    T,
};

use super::cast::cast_parser;

pub fn is_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>> {
    cast_parser(stmnts, expr.clone()).foldl(
        just(T!["is"])
            .ignore_then(
                ident("type".to_string())
                    .recover_with(via_parser(any().or_not().map(|_| "".to_string())))
                    .map_with(|txt, e| (txt, e.span())),
            )
            .repeated(),
        |expr, cast| {
            let span = SimpleSpan::new(expr.1.start, cast.1.end);

            (Expression::Is(Box::new(expr), cast), span)
        },
    )
}
