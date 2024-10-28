use chumsky::prelude::*;

use super::{lexer::Token, AmberParser};

const KEYWORDS: [&str; 24] = [
    "if", "else", "loop", "in", "return", "break", "continue", "true", "false", "null", "fun",
    "as", "is", "or", "and", "not", "nameof", "status", "fail", "echo", "let", "unsafe", "silent",
    "main",
];

pub fn ident<'a>(ident_name: String) -> impl AmberParser<'a, String> {
    any()
        .try_map(move |token: Token, span| {
            let word = token.to_string();
            let mut chars = word.chars();

            let first_char = chars.next().unwrap();

            if !first_char.is_ascii_alphabetic() && first_char != '_' {
                return Err(Rich::custom(
                    span,
                    "identifier must start with a letter or an underscore",
                ));
            }

            for char in chars {
                if !char.is_ascii_alphanumeric() && char != '_' {
                    return Err(Rich::custom(
                        span,
                        "identifier must contain only alphanumeric characters or underscores",
                    ));
                }
            }

            if KEYWORDS.contains(&word.as_str()) {
                return Err(Rich::custom(
                    span,
                    format!("keyword used as {ident_name} name"),
                ));
            }

            Ok(word)
        })
}
