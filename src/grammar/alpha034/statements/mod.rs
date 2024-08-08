use chumsky::prelude::*;

use super::{expressions::parse_expr, Spanned, Statement};

pub mod block;
pub mod comment;
pub mod failed;
pub mod if_cond;
pub mod keywords;
pub mod loops;
pub mod modifiers;
pub mod shorthands;
pub mod var_init;
pub mod var_set;

pub fn statement_parser() -> impl Parser<char, Spanned<Statement>, Error = Simple<char>> {
    recursive(|stmnt| {
        var_init::var_init_parser(stmnt.clone())
            .or(var_set::var_set_parser(stmnt.clone()))
            .or(block::block_parser_statement(stmnt.clone()))
            .or(if_cond::if_chain_parser(stmnt.clone()))
            .or(if_cond::if_cond_parser(stmnt.clone()))
            .or(shorthands::shorthand_parser(stmnt.clone()))
            .or(loops::inf_loop_parser(stmnt.clone()))
            .or(loops::iter_loop_parser(stmnt.clone()))
            .or(keywords::keywords_parser(stmnt.clone()))
            .or(modifiers::modifier_parser())
            .or(comment::comment_parser())
            .or(parse_expr(stmnt)
                .map_with_span(|expr, span| (Statement::Expression(Box::new(expr)), span)))
    })
}
