use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{
        expressions::parse_expr, lexer::Token, AmberParser, ElseCondition, IfChainContent,
        IfCondition, Spanned, Statement,
    },
    T,
};

use super::block::block_parser;

fn else_cond_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<ElseCondition>> {
    let else_condition = just(T!["else"])
        .ignore_then(block_parser(stmnts.clone()))
        .map_with(|body, e| (ElseCondition::Else(body), e.span()));

    let else_inline = just(T!["else"])
        .ignore_then(just(T![":"]))
        .ignore_then(stmnts.clone().recover_with(via_parser(
            any().or_not().map_with(|_, e| (Statement::Error, e.span())),
        )))
        .map_with(|body, e| (ElseCondition::InlineElse(Box::new(body)), e.span()));

    choice((else_condition, else_inline)).boxed()
}

fn cond_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<IfCondition>> {
    let inline_if = parse_expr(stmnts.clone())
        .then_ignore(just(T![":"]))
        .then(stmnts.clone().recover_with(via_parser(
            any().or_not().map_with(|_, e| (Statement::Error, e.span())),
        )))
        .map_with(|(condition, body), e| {
            (
                IfCondition::InlineIfCondition(Box::new(condition), Box::new(body)),
                e.span(),
            )
        });

    let if_condition = parse_expr(stmnts.clone())
        .then(block_parser(stmnts))
        .map_with(|(cond, body), e| (IfCondition::IfCondition(Box::new(cond), body), e.span()));

    choice((inline_if, if_condition)).boxed()
}

pub fn if_cond_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<Statement>> {
    just(T!["if"])
        .ignore_then(cond_parser(stmnts.clone()))
        .then(else_cond_parser(stmnts).or_not())
        .map_with(|(if_cond, else_cond), e| (Statement::IfCondition(if_cond, else_cond), e.span()))
        .boxed()
}

pub fn if_chain_parser<'a>(
    stmnts: impl AmberParser<'a, Spanned<Statement>>,
) -> impl AmberParser<'a, Spanned<Statement>> {
    just(T!["if"])
        .ignore_then(just(T!["{"]))
        .ignore_then(
            cond_parser(stmnts.clone())
                .recover_with(via_parser(
                    none_of([T!["}"], T!["else"]]).map_with(|_, e| (IfCondition::Error, e.span())),
                ))
                .repeated()
                .collect::<Vec<Spanned<IfCondition>>>(),
        )
        .then(else_cond_parser(stmnts).or_not())
        .then_ignore(just(T!["}"]).recover_with(via_parser(any().or_not().map(|_| T!["}"]))))
        .map_with(
            |(if_conds, else_cond): (Vec<Spanned<IfCondition>>, Option<Spanned<ElseCondition>>),
             e| {
                let mut if_chain: Vec<Spanned<IfChainContent>> = if_conds
                    .into_iter()
                    .map(|if_cond| (IfChainContent::IfCondition(if_cond.clone()), if_cond.1))
                    .collect::<Vec<_>>();

                if let Some(else_cond) = else_cond {
                    if_chain.push((IfChainContent::Else(else_cond.clone()), else_cond.1));
                }

                (Statement::IfChain(if_chain), e.span())
            },
        )
        .boxed()
}
