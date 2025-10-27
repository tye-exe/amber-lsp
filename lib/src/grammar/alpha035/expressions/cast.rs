use chumsky::prelude::*;

use crate::{
    grammar::alpha035::{
        global::type_parser, lexer::Token, parser::default_recovery, AmberParser, DataType,
        Expression, Spanned, Statement,
    },
    T,
};

use super::unary::unary_parser;

pub fn cast_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
    expr: impl AmberParser<'a, Spanned<Expression>>,
) -> impl AmberParser<'a, Spanned<Expression>> {
    unary_parser(stmnts, expr)
        .foldl(
            just(T!["as"])
                .map_with(|t, e| (t.to_string(), e.span()))
                .then(
                    type_parser().recover_with(via_parser(
                        default_recovery()
                            .or_not()
                            .map_with(|_, e| (DataType::Error, e.span())),
                    )),
                )
                .repeated(),
            |expr, (as_keyword, cast)| {
                let span = SimpleSpan::new(expr.1.start, cast.1.end);

                (Expression::Cast(Box::new(expr), as_keyword, cast), span)
            },
        )
        .boxed()
        .labelled("expression")
}
