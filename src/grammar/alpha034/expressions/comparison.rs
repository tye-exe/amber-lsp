use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Expression, Spanned, Statement},
    T,
};

use super::sum::sum_parser;

pub fn comparison_parser<'a>(
    stmnts: Recursive<'a, Token, Spanned<Statement>, Simple<Token>>,
    expr: Recursive<'a, Token, Spanned<Expression>, Simple<Token>>,
) -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> + 'a {
    sum_parser(stmnts.clone(), expr.clone())
        .then(
            just(T![">="])
                .to(Expression::Ge as fn(_, _) -> _)
                .or(just(T![">"]).to(Expression::Gt as fn(_, _) -> _))
                .or(just(T!["<="]).to(Expression::Le as fn(_, _) -> _))
                .or(just(T!["<"]).to(Expression::Lt as fn(_, _) -> _))
                .or(just(T!["=="]).to(Expression::Eq as fn(_, _) -> _))
                .or(just(T!["!="]).to(Expression::Neq as fn(_, _) -> _))
                .then(sum_parser(stmnts.clone(), expr.clone()))
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| {
            let span = lhs.1.start..rhs.1.end;

            (op(Box::new(lhs), Box::new(rhs)), span)
        })
}
