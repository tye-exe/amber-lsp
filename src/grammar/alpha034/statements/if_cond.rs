use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{
    expressions::parse_expr, ElseCondition, IfChainContent, IfCondition, Statement,
};

use super::block::block_parser;

fn else_cond_parser(
    stmnts: Recursive<char, Statement, Simple<char>>,
) -> impl Parser<char, Option<ElseCondition>, Error = Simple<char>> + '_ {
    let else_condition = keyword("else")
        .padded()
        .ignore_then(block_parser(stmnts.clone()))
        .padded()
        .map(|body| ElseCondition::Else(body));

    let else_inline = keyword("else")
        .padded()
        .ignore_then(just(":"))
        .padded()
        .ignore_then(stmnts.clone())
        .map(|body| ElseCondition::InlineElse(Box::new(body)));

    // else_condition
    //     .or(else_inline)
    //     .or_not()
    else_inline.or(else_condition).or_not()
}

fn cond_parser(
    stmnts: Recursive<char, Statement, Simple<char>>,
) -> impl Parser<char, IfCondition, Error = Simple<char>> + '_ {
    let inline_if = parse_expr(stmnts.clone())
        .padded()
        .then_ignore(just(":"))
        .then(stmnts.clone())
        .map(|(condition, body)| {
            IfCondition::InlineIfCondition(Box::new(condition), Box::new(body))
        });

    let if_condition = parse_expr(stmnts.clone())
        .padded()
        .then(block_parser(stmnts))
        .map(|(cond, body)| IfCondition::IfCondition(Box::new(cond), body));

    // if_condition.or(inline_if)
    inline_if.or(if_condition)
}

pub fn if_cond_parser(
    stmnts: Recursive<char, Statement, Simple<char>>,
) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    just("if")
        .padded()
        .ignore_then(cond_parser(stmnts.clone()))
        .then(else_cond_parser(stmnts))
        .map(|(if_cond, else_cond)| Statement::IfCondition(if_cond, else_cond))
}

pub fn if_chain_parser(
    stmnts: Recursive<char, Statement, Simple<char>>,
) -> impl Parser<char, Statement, Error = Simple<char>> + '_ {
    just("if")
        .padded()
        .ignore_then(just("{"))
        .padded()
        .ignore_then(cond_parser(stmnts.clone()).repeated())
        .padded()
        .then(else_cond_parser(stmnts))
        .padded()
        .then_ignore(just("}"))
        .map(|(if_conds, else_cond)| {
            let mut if_chain: Vec<IfChainContent> = if_conds
                .into_iter()
                .map(|if_cond| IfChainContent::IfCondition(if_cond))
                .collect::<Vec<_>>();
            if let Some(else_cond) = else_cond {
                if_chain.push(IfChainContent::Else(else_cond));
            }
            Statement::IfChain(if_chain)
        })
}
