use std::ops::Range;

use chumsky::prelude::*;

use crate::{
    grammar::alpha034::{lexer::Token, Spanned},
    T,
};

use super::Expression;

pub fn number_parser() -> impl Parser<Token, Spanned<Expression>, Error = Simple<Token>> {
    let int = filter_map(|span, token: Token| {
        let word = token.to_string();

        for char in word.chars() {
            if !char.is_ascii_digit() {
                return Err(Simple::custom(span, "int must contain only digits"));
            }
        }

        Ok(word)
    });

    int.then(just(T!['.']).ignore_then(int).or_not())
        .map(|(int, float)| {
            let float = float.unwrap_or('0'.to_string());

            format!("{}.{}", int, float)
        })
        .from_str::<f32>()
        .unwrapped()
        .map_with_span(|num, span: Range<usize>| (Expression::Number((num, span.clone())), span))
}
