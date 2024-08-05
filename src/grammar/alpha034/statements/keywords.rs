use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{expressions::parse_expr, Statement};

pub fn keywords_parser(
    stmnts: Recursive<char, Statement, Simple<char>>,
) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    keyword("break")
        .to(Statement::Break)
        .or(keyword("continue").to(Statement::Continue))
        .or(keyword("return")
            .padded()
            .ignore_then(parse_expr(stmnts.clone()).or_not())
            .map(|expr| Statement::Return(expr.map(Box::new))))
        .or(keyword("fail")
            .padded()
            .ignore_then(parse_expr(stmnts.clone()).or_not())
            .map(|expr| Statement::Fail(expr.map(Box::new))))
        .or(keyword("echo")
            .padded()
            .ignore_then(parse_expr(stmnts))
            .map(|expr| Statement::Echo(Box::new(expr))))
        .padded()
}
