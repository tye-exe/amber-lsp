use chumsky::{error::Simple, prelude::filter_map, Error, Parser};

use super::{global, lexer::Token, GlobalStatement, Spanned, SpannedSemanticToken};

pub fn ident() -> impl Parser<Token, String, Error = Simple<Token>> {
    filter_map(|span, token: Token| {
        let word = token.to_string();
        let mut chars = word.chars();

        let first_char = chars.next().unwrap();

        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return Err(Simple::expected_input_found(span, Vec::new(), Some(token)));
        }

        for char in chars {
            if !char.is_ascii_alphanumeric() && char != '_' {
                return Err(Simple::expected_input_found(span, Vec::new(), Some(token)));
            }
        }

        Ok(word)
    })
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
