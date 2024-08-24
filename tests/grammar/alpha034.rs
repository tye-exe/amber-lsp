use chumsky::error::Simple;
use insta::assert_debug_snapshot;

use amber_lsp::grammar::alpha034::{lexer::Token, AmberCompiler, Spanned};

fn parse(
    input: &str,
) -> (
    Option<Vec<Spanned<amber_lsp::grammar::alpha034::GlobalStatement>>>,
    Vec<Simple<Token>>,
) {
    AmberCompiler::new().parse(input)
}

fn parse_unwrap(input: &str) -> Vec<Spanned<amber_lsp::grammar::alpha034::GlobalStatement>> {
    parse(input).0.unwrap()
}

#[test]
fn test_text() {
    assert_debug_snapshot!(parse_unwrap("\"Hello, world!\""));
    assert_debug_snapshot!(parse_unwrap("\"Hello, {name}!\""));
    assert_debug_snapshot!(parse_unwrap("\"Hello, {name}! How are you?\""));
    assert_debug_snapshot!(parse_unwrap(r#""\"text in quotes\" \\""#));
}

#[test]
fn test_variable_get() {
    assert_debug_snapshot!(parse_unwrap("name"));
    assert_debug_snapshot!(parse_unwrap("name1"));
    assert_debug_snapshot!(parse_unwrap("name_1"));
    assert_debug_snapshot!(parse_unwrap("name_1_"));
    assert_debug_snapshot!(parse_unwrap("_var"));
}

#[test]
fn test_number() {
    assert_debug_snapshot!(parse_unwrap("1"));
    assert_debug_snapshot!(parse_unwrap("1.0"));
    assert_debug_snapshot!(parse_unwrap("-1.0"));
    assert_debug_snapshot!(parse_unwrap("-1.24"));
    assert_debug_snapshot!(parse_unwrap("-5"));
    assert_debug_snapshot!(parse_unwrap("001.00004"));
}

#[test]
fn test_whitespace() {
    assert_debug_snapshot!(parse_unwrap(" 0"));
    assert_debug_snapshot!(parse_unwrap("  0"));
    assert_debug_snapshot!(parse_unwrap("  0 "));
}

#[test]
fn test_bool() {
    assert_debug_snapshot!(parse_unwrap("true"));
    assert_debug_snapshot!(parse_unwrap("false"));
}

#[test]
fn test_add() {
    assert_debug_snapshot!(parse_unwrap("1 + 2"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 + 3"));
}

#[test]
fn test_subtract() {
    assert_debug_snapshot!(parse_unwrap("1 - 2"));
    assert_debug_snapshot!(parse_unwrap("1 - 2 - 3"));
}

#[test]
fn test_add_and_subtract() {
    assert_debug_snapshot!(parse_unwrap("1 + 2 - 3"));
    assert_debug_snapshot!(parse_unwrap("1 - 2 + 3"));
}

#[test]
fn test_multiply() {
    assert_debug_snapshot!(parse_unwrap("1 * 2"));
    assert_debug_snapshot!(parse_unwrap("1 * 2 * 3"));
}

#[test]
fn test_divide() {
    assert_debug_snapshot!(parse_unwrap("1 / 2"));
    assert_debug_snapshot!(parse_unwrap("1 / 2 / 3"));
}

#[test]
fn test_multiply_and_divide() {
    assert_debug_snapshot!(parse_unwrap("1 * 2 / 3"));
    assert_debug_snapshot!(parse_unwrap("1 / 2 * 3"));
}

#[test]
fn test_mults_and_adds() {
    assert_debug_snapshot!(parse_unwrap("1 + 2 * 3"));
    assert_debug_snapshot!(parse_unwrap("1 * 2 + 3"));
    assert_debug_snapshot!(parse_unwrap("1 / 2 + 3"));
    assert_debug_snapshot!(parse_unwrap("1 - 2 / 3"));
    assert_debug_snapshot!(parse_unwrap("3 * 2 - --2 * 2"));
}

#[test]
fn test_modulo() {
    assert_debug_snapshot!(parse_unwrap("1 % 2"));
    assert_debug_snapshot!(parse_unwrap("1 % 2 % 3"));
}

#[test]
fn test_neg() {
    assert_debug_snapshot!(parse_unwrap("-(1)"));
    assert_debug_snapshot!(parse_unwrap("-(1 - 2)"));
}

#[test]
fn test_and() {
    assert_debug_snapshot!(parse_unwrap("true and false"));
    assert_debug_snapshot!(parse_unwrap("true and false and true"));
}

#[test]
fn test_or() {
    assert_debug_snapshot!(parse_unwrap("false or false"));
    assert_debug_snapshot!(parse_unwrap("false or false or true"));
}

#[test]
fn test_gt() {
    assert_debug_snapshot!(parse_unwrap("1 > 2"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 > 2 + 1"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 > 2 + 1 > 5"));
}

#[test]
fn test_ge() {
    assert_debug_snapshot!(parse_unwrap("1 >= 2"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 >= 2 + 1"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 >= 2 + 1 >= 5"));
}

#[test]
fn test_lt() {
    assert_debug_snapshot!(parse_unwrap("1 < 2"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 < 2 + 1"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 < 2 + 1 < 5"));
}

#[test]
fn test_le() {
    assert_debug_snapshot!(parse_unwrap("1 <= 2"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 <= 2 + 1"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 <= 2 + 1 <= 5"));
}

#[test]
fn test_eq() {
    assert_debug_snapshot!(parse_unwrap("1 == 2"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 == 2 + 1"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 == 2 + 1 + 5"));
}

#[test]
fn test_neq() {
    assert_debug_snapshot!(parse_unwrap("1 != 2"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 != 2 + 1"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 != 2 + 1 + 5"));
}

#[test]
fn test_not() {
    assert_debug_snapshot!(parse_unwrap("not true"));
    assert_debug_snapshot!(parse_unwrap("not not true"));
}

#[test]
fn test_ternary() {
    assert_debug_snapshot!(parse_unwrap("   true then 1 else 2   "));
    assert_debug_snapshot!(parse_unwrap("true then 1 + 2 else 2 + 1"));
    assert_debug_snapshot!(parse_unwrap("true then 1 + 2 else false then 5 else 6"));
}

#[test]
fn test_command() {
    assert_debug_snapshot!(parse_unwrap(r#"$echo \"Hello, world!\"$"#));
    assert_debug_snapshot!(parse_unwrap("$echo \"Hello, {name}!\"$"));
    assert_debug_snapshot!(parse("$command --arg1 -v$"));
}

#[test]
fn test_array() {
    assert_debug_snapshot!(parse_unwrap("[1, 2, 3]"));
    assert_debug_snapshot!(parse_unwrap("[1]"));
}

#[test]
fn test_null() {
    assert_debug_snapshot!(parse_unwrap("null"));
}

#[test]
fn test_range() {
    assert_debug_snapshot!(parse_unwrap("1..2"));
    assert_debug_snapshot!(parse_unwrap("1..=2"));
    assert_debug_snapshot!(parse_unwrap("1..2..3"));
}

#[test]
fn test_function_invocation() {
    assert_debug_snapshot!(parse_unwrap("func()"));
    assert_debug_snapshot!(parse_unwrap("func(1)"));
    assert_debug_snapshot!(parse_unwrap("func(1, 2)"));
}

#[test]
fn test_cast() {
    assert_debug_snapshot!(parse_unwrap("1 as Num"));
    assert_debug_snapshot!(parse_unwrap("1 as Num as Text"));
}

#[test]
fn test_nameof() {
    assert_debug_snapshot!(parse_unwrap("nameof variable"));
    assert_debug_snapshot!(parse_unwrap("nameof nameof variable"));
}

#[test]
fn test_expr_precedence() {
    assert_debug_snapshot!(parse_unwrap("1 + 2 * 3"));
    assert_debug_snapshot!(parse_unwrap("1 + 2 / 4 / 6"));
    assert_debug_snapshot!(parse_unwrap("2 - 3 - 4"));
    assert_debug_snapshot!(parse_unwrap("2 - (3 - 4)"));
    assert_debug_snapshot!(parse_unwrap("-(2 + 3) * 5"));
    assert_debug_snapshot!(parse_unwrap("(8+2)*(7-3)/2"));
    assert_debug_snapshot!(parse_unwrap("2 / 3 + 1"));
    assert_debug_snapshot!(parse_unwrap("25 / 5 * 3 + 7 - 2 * 4"));
    assert_debug_snapshot!(parse_unwrap(
        "
            2 + 5 > 3 + 4
                then 15 + 10
                else 5 - 4 <= 1/2
                    then 3 * 4
                    else 2"
    ));
    assert_debug_snapshot!(parse_unwrap("true or false and true and true or false"));
    assert_debug_snapshot!(parse_unwrap("true as Bool as Text as Num * 2 / foo()"));
}

#[test]
fn test_comment() {
    assert_debug_snapshot!(parse_unwrap(
        "
        // This is a comment
        1 + 2
    "
    ));
    assert_debug_snapshot!(parse_unwrap("1 + 2 // This is a comment without a newline"));
}

#[test]
fn test_import() {
    assert_debug_snapshot!(parse_unwrap("import * \"path/to/module\""));
    assert_debug_snapshot!(parse_unwrap("import {} \"path/to/module\""));
    assert_debug_snapshot!(parse_unwrap("import { var1 } \"path/to/module\""));
    assert_debug_snapshot!(parse_unwrap("import { var1, var2 } \"path/to/module\""));
}

#[test]
fn test_function_def() {
    assert_debug_snapshot!(parse_unwrap("fun func() {}"));
    assert_debug_snapshot!(parse_unwrap("fun func(a) {}"));
    assert_debug_snapshot!(parse_unwrap("fun func(a : Num) {}"));
    assert_debug_snapshot!(parse_unwrap("fun func(a: Num, b, c: Bool): Num {}"));
    assert_debug_snapshot!(parse_unwrap(
        "
        fun func(a: Num, b: Text, c: Bool): Num {
            echo 10

            return 10
        }
    "
    ));
}

#[test]
fn test_main_block() {
    assert_debug_snapshot!(parse_unwrap(
        "
        main {
            echo 10
        }

        main {
            echo 3;
        }
    "
    ));
}

#[test]
fn test_var_init() {
    assert_debug_snapshot!(parse_unwrap("let a = 10"));
    assert_debug_snapshot!(parse_unwrap("let a = 10 + 2"));
    assert_debug_snapshot!(parse_unwrap("let a = 10 + 2 * 3"));
}

#[test]
fn test_var_set() {
    assert_debug_snapshot!(parse_unwrap("a = 10"));
    assert_debug_snapshot!(parse_unwrap("a = 10 + 2"));
    assert_debug_snapshot!(parse_unwrap("a = 10 + 2 * 3"));
}

#[test]
fn test_if_condition() {
    assert_debug_snapshot!(parse_unwrap("if true {}"));
    assert_debug_snapshot!(parse_unwrap("if true { echo 10 }"));
    assert_debug_snapshot!(parse_unwrap("if true { echo 10 } else {}"));
    assert_debug_snapshot!(parse_unwrap("if true { echo 10 } else { echo 20 }"));
    assert_debug_snapshot!(parse_unwrap(
        "
        if true: echo 10
        else: echo 20
    "
    ));
}

#[test]
fn test_if_chain() {
    assert_debug_snapshot!(parse_unwrap(
        "
        if {
            2 == 3 {
                echo 10
            }
            true: echo 10
            else {
                echo 20
            }
        }
    "
    ));
}

#[test]
fn test_semicolon() {
    assert_debug_snapshot!(parse_unwrap("1;"));
    assert_debug_snapshot!(parse_unwrap("1; 2;"));
    assert_debug_snapshot!(parse_unwrap(
        "
        main {
            echo 10;
            echo 20

            echo 30;

            10 20 30
        }
    "
    ));
}

#[test]
fn test_shorthands() {
    assert_debug_snapshot!(parse_unwrap("a += 10"));
    assert_debug_snapshot!(parse_unwrap("a -= 10"));
    assert_debug_snapshot!(parse_unwrap("a *= 10"));
    assert_debug_snapshot!(parse_unwrap("a /= 10"));
    assert_debug_snapshot!(parse_unwrap("a %= 10"));
}

#[test]
fn test_loops() {
    assert_debug_snapshot!(parse_unwrap("loop {}"));
    assert_debug_snapshot!(parse_unwrap("loop var1 in 1..2 {}"));
    assert_debug_snapshot!(parse_unwrap("loop var1, var2 in 1..2 {}"));
}

#[test]
fn test_keywords() {
    assert_debug_snapshot!(parse_unwrap("break"));
    assert_debug_snapshot!(parse_unwrap("continue"));
    assert_debug_snapshot!(parse_unwrap("fail"));
    assert_debug_snapshot!(parse_unwrap("fail 1"));
    assert_debug_snapshot!(parse_unwrap("echo 1"));
    assert_debug_snapshot!(parse_unwrap("return"));
    assert_debug_snapshot!(parse_unwrap("return 1"));
}

#[test]
fn test_modifiers() {
    assert_debug_snapshot!(parse_unwrap("unsafe"));
    assert_debug_snapshot!(parse_unwrap("silent"));
    assert_debug_snapshot!(parse_unwrap("silent unsafe {}"));
    assert_debug_snapshot!(parse_unwrap("unsafe silent {}"));
    assert_debug_snapshot!(parse_unwrap("unsafe silent $command$"));
}

#[test]
fn test_failure_handlers() {
    assert_debug_snapshot!(parse_unwrap("$$?"));
    assert_debug_snapshot!(parse_unwrap("$$ failed {}"));
    assert_debug_snapshot!(parse_unwrap("foo()?"));
    assert_debug_snapshot!(parse_unwrap("foo() failed {}"));
}

#[test]
fn test_blocks() {
    assert_debug_snapshot!(parse_unwrap(
        "
        {
            echo 10
        }
        {
            {
                echo 20
            }
        }
    "
    ));
}

#[test]
fn test_recovery() {
    // TODO: Add more tests
    assert_debug_snapshot!(parse("fun foo(abc!) {}"));
    assert_debug_snapshot!(parse(
        "
    5 + 5 +;
    echo 10"
    ));
}

#[test]
fn test_lexer() {
    let mut compiler = AmberCompiler::new();

    assert_debug_snapshot!(compiler.tokenize(
        r#"
        let x = "my \"interpolated\" string {name} end";

        $this --should be - tokenized \$$
        "unclosed string

        abcd {let x = 10
    "#
    ));
}
