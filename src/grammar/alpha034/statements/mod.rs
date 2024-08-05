use chumsky::prelude::*;

use super::{expressions::parse_expr, Statement};

pub mod block;
pub mod if_cond;
pub mod keywords;
pub mod loops;
pub mod modifiers;
pub mod shorthands;
pub mod var_init;
pub mod var_set;
pub mod failed;
pub mod comment;

pub fn statement_parser() -> impl Parser<char, Statement, Error = Simple<char>> {
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
            .or(parse_expr(stmnt).map(|expr| Statement::Expression(Box::new(expr))))
    })
}
