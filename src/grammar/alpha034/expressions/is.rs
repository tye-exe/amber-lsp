use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, parser::ident, Expression, Spanned, Statement},
    T,
};

use super::cast::cast_parser;

pub fn is_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    cast_parser(stmnts, expr.clone())
        .then(
            just(T!["is"])
                .ignore_then(ident().map_with_span(|txt, span| (txt, span)))
                .repeated(),
        )
        .foldl(|expr, cast| {
            let span = expr.1.start..cast.1.end;

            (Expression::Is(Box::new(expr), cast), span)
        })
}
