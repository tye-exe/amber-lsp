use chumsky::prelude::*;
use text::keyword;

use crate::grammar::alpha034::{
    expressions::parse_expr, ElseCondition, IfChainContent, IfCondition, Spanned, Statement,
};

use super::block::block_parser;

fn else_cond_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<ElseCondition>, Error = Simple<char>> + '_ {
    let else_condition = keyword("else")
        .ignore_then(filter(|c: &char| c.is_whitespace()).repeated())
        .ignore_then(block_parser(stmnts.clone()))
        .map_with_span(|body, span| (ElseCondition::Else(body), span));

    let else_inline = keyword("else")
        .ignore_then(just(":").padded())
        .ignore_then(stmnts.clone())
        .map_with_span(|body, span| (ElseCondition::InlineElse(Box::new(body)), span));

    else_inline.or(else_condition)
}

fn cond_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<IfCondition>, Error = Simple<char>> + '_ {
    let inline_if = parse_expr(stmnts.clone())
        .then_ignore(just(":").padded())
        .then(stmnts.clone())
        .map_with_span(|(condition, body), span| {
            (
                IfCondition::InlineIfCondition(Box::new(condition), Box::new(body)),
                span,
            )
        });

    let if_condition = parse_expr(stmnts.clone())
        .then_ignore(filter(|c: &char| c.is_whitespace()).repeated())
        .then(block_parser(stmnts))
        .map_with_span(|(cond, body), span| (IfCondition::IfCondition(Box::new(cond), body), span));

    inline_if.or(if_condition)
}

pub fn if_cond_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    just("if")
        .ignore_then(filter(|c: &char| c.is_whitespace()).repeated())
        .ignore_then(cond_parser(stmnts.clone()))
        .then(
            filter(|c: &char| c.is_whitespace())
                .repeated()
                .ignore_then(else_cond_parser(stmnts))
                .or_not(),
        )
        .map_with_span(|(if_cond, else_cond), span| {
            (Statement::IfCondition(if_cond, else_cond), span)
        })
}

pub fn if_chain_parser(
    stmnts: Recursive<char, Spanned<Statement>, Simple<char>>,
) -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> + '_ {
    just("if")
        .ignore_then(just("{").padded())
        .ignore_then(cond_parser(stmnts.clone()).padded().repeated())
        .then(else_cond_parser(stmnts).padded().or_not())
        .then_ignore(just("}"))
        .map_with_span(|(if_conds, else_cond), span| {
            let mut if_chain: Vec<Spanned<IfChainContent>> = if_conds
                .into_iter()
                .map(|if_cond| (IfChainContent::IfCondition(if_cond.clone()), if_cond.1))
                .collect::<Vec<_>>();
            if let Some(else_cond) = else_cond {
                if_chain.push((IfChainContent::Else(else_cond.clone()), else_cond.1));
            }

            (Statement::IfChain(if_chain), span)
        })
}
