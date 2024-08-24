use heraclitus_compiler::prelude::*;

pub fn get_rules() -> Rules {
    let symbols = vec![
        '+', '-', '*', '/', '%', '\n', ';', ':', '(', ')', '[', ']', '{', '}', ',', '.', '<', '>',
        '=', '!', '?', '\\', '"', '$'
    ];
    let compounds = vec![
        ('<', '='),
        ('>', '='),
        ('!', '='),
        ('=', '='),
        ('+', '='),
        ('-', '='),
        ('*', '='),
        ('/', '='),
        ('%', '='),
        ('.', '.'),
        ('/', '/'),
    ];
    let region = reg![
        reg!(string as "string literal" => {
            begin: "\"",
            end: "\"",
            tokenize: true,
            allow_left_open: true
        } => [
            reg!(str_interp as "string interpolation" => {
                begin: "{",
                end: "}",
                tokenize: true,
                allow_left_open: true
            } ref global)
        ]),
        reg!(command as "command literal" => {
            begin: "$",
            end: "$",
            tokenize: true,
            allow_left_open: true
        } => [
            reg!(com_interp as "command interpolation" => {
                begin: "{",
                end: "}",
                tokenize: true,
                allow_left_open: true
            } ref global)
        ]),
        reg!(cc_flag as "compiler flag" => {
            begin: "#[",
            end: "]",
            allow_left_open: true
        }),
        reg!(comment as "comment" => {
            begin: "//",
            end: "\n",
            allow_left_open: true
        }),
        reg!(interp as "interpolation" => {
            begin: "{",
            end: "}",
            tokenize: true,
            allow_left_open: true
        } ref global)
    ];
    Rules::new(symbols, compounds, region)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(pub String);

impl ToString for Token {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromIterator<Token> for String {
    fn from_iter<I: IntoIterator<Item = Token>>(iter: I) -> Self {
        iter.into_iter().map(|t| t.0).collect()
    }
}

#[macro_export]
macro_rules! T {
    [$text:expr] => {
        Token($text.to_string())
    };
}
