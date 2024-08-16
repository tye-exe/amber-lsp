use chumsky::{error::Simple, Parser};

use super::{global, GlobalStatement, Spanned, SpannedSemanticToken};

pub fn parse(input: &str) -> (Option<Vec<Spanned<GlobalStatement>>>, Vec<Simple<char>>) {
    global::global_statement_parser().parse_recovery(input)
}

fn semantic_tokens_from_ast(ast: &Option<Vec<Spanned<GlobalStatement>>>) -> Vec<SpannedSemanticToken> {
    ast.as_ref().map_or(vec![], |ast| {
        let mut tokens = vec![];

        for (statement, span) in ast {
            match statement {
                GlobalStatement::Import(_, _) => {
                    tokens.push((0, span.clone()));
                }
                GlobalStatement::FunctionDefinition(_, _, _, _) => {
                    tokens.push((0, span.clone()));
                }
                GlobalStatement::Main(_) => {
                    tokens.push((0, span.clone()));
                }
                GlobalStatement::Statement(_) => {
                    tokens.push((0, span.clone()));
                }
            }
        }
        
        tokens
    })
}
