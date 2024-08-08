use chumsky::prelude::*;

use crate::grammar::alpha034::{Expression, Spanned, Statement};

use super::is::is_parser;

pub fn product_parser<'a>(
    stmnts: Recursive<'a, char, Spanned<Statement>, Simple<char>>,
    expr: Recursive<'a, char, Spanned<Expression>, Simple<char>>,
) -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> + 'a {
    is_parser(stmnts.clone(), expr.clone())
        .then(
            just('*')
                .padded()
                .to(Expression::Multiply as fn(_, _) -> _)
                .or(just('/').padded().to(Expression::Divide as fn(_, _) -> _))
                .or(just('%').padded().to(Expression::Modulo as fn(_, _) -> _))
                .then(is_parser(stmnts, expr))
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.1.start..rhs.1.end;

            (op(Box::new(lhs), Box::new(rhs)), span)
        })
}
