use heraclitus_compiler::prelude::*;

pub use crate::grammar::Token;

pub fn get_rules() -> Rules {
    let symbols = vec![
        '+', '-', '*', '/', '%', ';', ':', '(', ')', '[', ']', '{', '}', ',', '.', '<', '>', '=',
        '!', '?', '\\', '"', '$', '\n',
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
