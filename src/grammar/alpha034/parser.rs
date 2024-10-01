use chumsky::{error::Simple, prelude::filter_map, Parser};

use super::lexer::Token;

pub fn ident() -> impl Parser<Token, String, Error = Simple<Token>> {
    filter_map(|span, token: Token| {
        let word = token.to_string();
        let mut chars = word.chars();

        let first_char = chars.next().unwrap();

        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return Err(Simple::custom(
                span,
                "identifier must start with a letter or an underscore",
            ));
        }

        for char in chars {
            if !char.is_ascii_alphanumeric() && char != '_' {
                return Err(Simple::custom(
                    span,
                    "identifier must contain only alphanumeric characters or underscores",
                ));
            }
        }

        Ok(word)
    })
}
