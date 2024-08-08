use std::ops::Range;

use chumsky::prelude::*;

use crate::grammar::alpha034::Spanned;

use super::Expression;

pub fn number_parser() -> impl Parser<char, Spanned<Expression>, Error = Simple<char>> {
    filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .then(
            just('.')
                .chain(
                    filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_digit())
                        .repeated()
                        .at_least(1),
                )
                .or_not(),
        )
        .map(|(int, float)| {
            let int = int.into_iter().collect::<String>();
            let float = float
                .unwrap_or(vec!['.', '0'])
                .into_iter()
                .collect::<String>();

            format!("{}{}", int, float)
        })
        .from_str::<f32>()
        .unwrapped()
        .map_with_span(|num, span: Range<usize>| (Expression::Number((num, span.clone())), span))
}
