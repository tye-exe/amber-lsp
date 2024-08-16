use chumsky::{error::Simple, Parser};
use insta::assert_debug_snapshot;

use amber_lsp::grammar::alpha034::{global::global_statement_parser, AmberCompiler, Spanned};

fn parse(
    input: &str,
) -> Result<Vec<Spanned<amber_lsp::grammar::alpha034::GlobalStatement>>, Vec<Simple<char>>> {
    global_statement_parser().parse(input)
}

fn parse_recover(
    input: &str,
) -> (
    Option<Vec<Spanned<amber_lsp::grammar::alpha034::GlobalStatement>>>,
    Vec<Simple<char>>,
) {
    global_statement_parser().parse_recovery_verbose(input)
}

#[test]
fn test_text() {
    assert_debug_snapshot!(parse("\"Hello, world!\"").unwrap());
    assert_debug_snapshot!(parse("\"Hello, {name}!\"").unwrap());
    assert_debug_snapshot!(parse("\"Hello, {name}! How are you?\"").unwrap());
    assert_debug_snapshot!(parse(r#""\"text in quotes\" \\""#).unwrap());
}

#[test]
fn test_variable_get() {
    assert_debug_snapshot!(parse("name").unwrap());
    assert_debug_snapshot!(parse("name1").unwrap());
    assert_debug_snapshot!(parse("name_1").unwrap());
    assert_debug_snapshot!(parse("name_1_").unwrap());
    assert_debug_snapshot!(parse("_var").unwrap());
}

#[test]
fn test_number() {
    assert_debug_snapshot!(parse("1").unwrap());
    assert_debug_snapshot!(parse("1.0").unwrap());
    assert_debug_snapshot!(parse("-1.0").unwrap());
    assert_debug_snapshot!(parse("-1.24").unwrap());
    assert_debug_snapshot!(parse("-5").unwrap());
    assert_debug_snapshot!(parse("001.00004").unwrap());
}

#[test]
fn test_whitespace() {
    assert_debug_snapshot!(parse(" 0").unwrap());
    assert_debug_snapshot!(parse("  0").unwrap());
    assert_debug_snapshot!(parse("  0 ").unwrap());
}

#[test]
fn test_bool() {
    assert_debug_snapshot!(parse("true").unwrap());
    assert_debug_snapshot!(parse("false").unwrap());
}

#[test]
fn test_add() {
    assert_debug_snapshot!(parse("1 + 2").unwrap());
    assert_debug_snapshot!(parse("1 + 2 + 3").unwrap());
}

#[test]
fn test_subtract() {
    assert_debug_snapshot!(parse("1 - 2").unwrap());
    assert_debug_snapshot!(parse("1 - 2 - 3").unwrap());
}

#[test]
fn test_add_and_subtract() {
    assert_debug_snapshot!(parse("1 + 2 - 3").unwrap());
    assert_debug_snapshot!(parse("1 - 2 + 3").unwrap());
}

#[test]
fn test_multiply() {
    assert_debug_snapshot!(parse("1 * 2").unwrap());
    assert_debug_snapshot!(parse("1 * 2 * 3").unwrap());
}

#[test]
fn test_divide() {
    assert_debug_snapshot!(parse("1 / 2").unwrap());
    assert_debug_snapshot!(parse("1 / 2 / 3").unwrap());
}

#[test]
fn test_multiply_and_divide() {
    assert_debug_snapshot!(parse("1 * 2 / 3").unwrap());
    assert_debug_snapshot!(parse("1 / 2 * 3").unwrap());
}

#[test]
fn test_mults_and_adds() {
    assert_debug_snapshot!(parse("1 + 2 * 3").unwrap());
    assert_debug_snapshot!(parse("1 * 2 + 3").unwrap());
    assert_debug_snapshot!(parse("1 / 2 + 3").unwrap());
    assert_debug_snapshot!(parse("1 - 2 / 3").unwrap());
    assert_debug_snapshot!(parse("3 * 2 - --2 * 2").unwrap());
}

#[test]
fn test_modulo() {
    assert_debug_snapshot!(parse("1 % 2").unwrap());
    assert_debug_snapshot!(parse("1 % 2 % 3").unwrap());
}

#[test]
fn test_neg() {
    assert_debug_snapshot!(parse("-(1)").unwrap());
    assert_debug_snapshot!(parse("-(1 - 2)").unwrap());
}

#[test]
fn test_and() {
    assert_debug_snapshot!(parse("true and false").unwrap());
    assert_debug_snapshot!(parse("true and false and true").unwrap());
}

#[test]
fn test_or() {
    assert_debug_snapshot!(parse("false or false").unwrap());
    assert_debug_snapshot!(parse("false or false or true").unwrap());
}

#[test]
fn test_gt() {
    assert_debug_snapshot!(parse("1 > 2").unwrap());
    assert_debug_snapshot!(parse("1 + 2 > 2 + 1").unwrap());
    assert_debug_snapshot!(parse("1 + 2 > 2 + 1 > 5").unwrap());
}

#[test]
fn test_ge() {
    assert_debug_snapshot!(parse("1 >= 2").unwrap());
    assert_debug_snapshot!(parse("1 + 2 >= 2 + 1").unwrap());
    assert_debug_snapshot!(parse("1 + 2 >= 2 + 1 >= 5").unwrap());
}

#[test]
fn test_lt() {
    assert_debug_snapshot!(parse("1 < 2").unwrap());
    assert_debug_snapshot!(parse("1 + 2 < 2 + 1").unwrap());
    assert_debug_snapshot!(parse("1 + 2 < 2 + 1 < 5").unwrap());
}

#[test]
fn test_le() {
    assert_debug_snapshot!(parse("1 <= 2").unwrap());
    assert_debug_snapshot!(parse("1 + 2 <= 2 + 1").unwrap());
    assert_debug_snapshot!(parse("1 + 2 <= 2 + 1 <= 5").unwrap());
}

#[test]
fn test_eq() {
    assert_debug_snapshot!(parse("1 == 2").unwrap());
    assert_debug_snapshot!(parse("1 + 2 == 2 + 1").unwrap());
    assert_debug_snapshot!(parse("1 + 2 == 2 + 1 + 5").unwrap());
}

#[test]
fn test_neq() {
    assert_debug_snapshot!(parse("1 != 2").unwrap());
    assert_debug_snapshot!(parse("1 + 2 != 2 + 1").unwrap());
    assert_debug_snapshot!(parse("1 + 2 != 2 + 1 + 5").unwrap());
}

#[test]
fn test_not() {
    assert_debug_snapshot!(parse("not true").unwrap());
    assert_debug_snapshot!(parse("not not true").unwrap());
}

#[test]
fn test_ternary() {
    assert_debug_snapshot!(parse("   true then 1 else 2   ").unwrap());
    assert_debug_snapshot!(parse("true then 1 + 2 else 2 + 1").unwrap());
    assert_debug_snapshot!(parse("true then 1 + 2 else false then 5 else 6").unwrap());
}

#[test]
fn test_command() {
    assert_debug_snapshot!(parse(r#"$echo \"Hello, world!\"$"#).unwrap());
    assert_debug_snapshot!(parse("$echo \"Hello, {name}!\"$").unwrap());
    assert_debug_snapshot!(parse("$command --arg1 -v$").unwrap());
}

#[test]
fn test_array() {
    assert_debug_snapshot!(parse("[1, 2, 3]").unwrap());
    assert_debug_snapshot!(parse("[1]").unwrap());
}

#[test]
fn test_null() {
    assert_debug_snapshot!(parse("null").unwrap());
}

#[test]
fn test_range() {
    assert_debug_snapshot!(parse("1..2").unwrap());
    assert_debug_snapshot!(parse("1..=2").unwrap());
    assert_debug_snapshot!(parse("1..2..3").unwrap());
}

#[test]
fn test_function_invocation() {
    assert_debug_snapshot!(parse("func()").unwrap());
    assert_debug_snapshot!(parse("func(1)").unwrap());
    assert_debug_snapshot!(parse("func(1, 2)").unwrap());
}

#[test]
fn test_cast() {
    assert_debug_snapshot!(parse("1 as Num").unwrap());
    assert_debug_snapshot!(parse("1 as Num as Text").unwrap());
}

#[test]
fn test_nameof() {
    assert_debug_snapshot!(parse("nameof variable").unwrap());
    assert_debug_snapshot!(parse("nameof nameof variable").unwrap());
}

#[test]
fn test_expr_precedence() {
    assert_debug_snapshot!(parse("1 + 2 * 3").unwrap());
    assert_debug_snapshot!(parse("1 + 2 / 4 / 6").unwrap());
    assert_debug_snapshot!(parse("2 - 3 - 4").unwrap());
    assert_debug_snapshot!(parse("2 - (3 - 4)").unwrap());
    assert_debug_snapshot!(parse("-(2 + 3) * 5").unwrap());
    assert_debug_snapshot!(parse("(8+2)*(7-3)/2").unwrap());
    assert_debug_snapshot!(parse("2 / 3 + 1").unwrap());
    assert_debug_snapshot!(parse("25 / 5 * 3 + 7 - 2 * 4").unwrap());
    assert_debug_snapshot!(parse(
        "
            2 + 5 > 3 + 4
                then 15 + 10
                else 5 - 4 <= 1/2
                    then 3 * 4
                    else 2"
    )
    .unwrap());
    assert_debug_snapshot!(parse("true or false and true and true or false").unwrap());
    assert_debug_snapshot!(parse("true as Bool as Text as Num * 2 / foo()").unwrap());
}

#[test]
fn test_comment() {
    assert_debug_snapshot!(parse(
        "
        // This is a comment
        1 + 2
    "
    ));
    assert_debug_snapshot!(parse("1 + 2 // This is a comment without a newline"));
}

#[test]
fn test_import() {
    assert_debug_snapshot!(parse("import * \"path/to/module\"").unwrap());
    assert_debug_snapshot!(parse("import {} \"path/to/module\"").unwrap());
    assert_debug_snapshot!(parse("import { var1 } \"path/to/module\"").unwrap());
    assert_debug_snapshot!(parse("import { var1, var2 } \"path/to/module\"").unwrap());
}

#[test]
fn test_function_def() {
    assert_debug_snapshot!(parse("fun func() {}").unwrap());
    assert_debug_snapshot!(parse("fun func(a) {}").unwrap());
    assert_debug_snapshot!(parse("fun func(a : Num) {}").unwrap());
    assert_debug_snapshot!(parse("fun func(a: Num, b, c: Bool): Num {}").unwrap());
    assert_debug_snapshot!(parse(
        "
        fun func(a: Num, b: Text, c: Bool): Num {
            echo 10

            return 10
        }
    "
    )
    .unwrap());
}

#[test]
fn test_main_block() {
    assert_debug_snapshot!(parse(
        "
        main {
            echo 10
        }

        main {
            echo 3;
        }
    "
    )
    .unwrap());
}

#[test]
fn test_var_init() {
    assert_debug_snapshot!(parse("let a = 10").unwrap());
    assert_debug_snapshot!(parse("let a = 10 + 2").unwrap());
    assert_debug_snapshot!(parse("let a = 10 + 2 * 3").unwrap());
}

#[test]
fn test_var_set() {
    assert_debug_snapshot!(parse("a = 10").unwrap());
    assert_debug_snapshot!(parse("a = 10 + 2").unwrap());
    assert_debug_snapshot!(parse("a = 10 + 2 * 3").unwrap());
}

#[test]
fn test_if_condition() {
    assert_debug_snapshot!(parse("if true {}").unwrap());
    assert_debug_snapshot!(parse("if true { echo 10 }").unwrap());
    assert_debug_snapshot!(parse("if true { echo 10 } else {}").unwrap());
    assert_debug_snapshot!(parse("if true { echo 10 } else { echo 20 }").unwrap());
    assert_debug_snapshot!(parse(
        "
        if true: echo 10
        else: echo 20
    "
    )
    .unwrap());
}

#[test]
fn test_if_chain() {
    assert_debug_snapshot!(parse(
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
    )
    .unwrap());
}

#[test]
fn test_semicolon() {
    assert_debug_snapshot!(parse("1;").unwrap());
    assert_debug_snapshot!(parse("1; 2;").unwrap());
    assert_debug_snapshot!(parse(
        "
        main {
            echo 10;
            echo 20

            echo 30;

            10 20 30
        }
    "
    )
    .unwrap());
}

#[test]
fn test_shorthands() {
    assert_debug_snapshot!(parse("a += 10").unwrap());
    assert_debug_snapshot!(parse("a -= 10").unwrap());
    assert_debug_snapshot!(parse("a *= 10").unwrap());
    assert_debug_snapshot!(parse("a /= 10").unwrap());
    assert_debug_snapshot!(parse("a %= 10").unwrap());
}

#[test]
fn test_loops() {
    assert_debug_snapshot!(parse("loop {}").unwrap());
    assert_debug_snapshot!(parse("loop var1 in 1..2 {}").unwrap());
    assert_debug_snapshot!(parse("loop var1, var2 in 1..2 {}").unwrap());
}

#[test]
fn test_keywords() {
    assert_debug_snapshot!(parse("break").unwrap());
    assert_debug_snapshot!(parse("continue").unwrap());
    assert_debug_snapshot!(parse("fail").unwrap());
    assert_debug_snapshot!(parse("fail 1").unwrap());
    assert_debug_snapshot!(parse("echo 1").unwrap());
    assert_debug_snapshot!(parse("return").unwrap());
    assert_debug_snapshot!(parse("return 1").unwrap());
}

#[test]
fn test_modifiers() {
    assert_debug_snapshot!(parse("unsafe").unwrap());
    assert_debug_snapshot!(parse("silent").unwrap());
    assert_debug_snapshot!(parse("silent unsafe {}").unwrap());
    assert_debug_snapshot!(parse("unsafe silent {}").unwrap());
    assert_debug_snapshot!(parse("unsafe silent $command$").unwrap());
}

#[test]
fn test_failure_handlers() {
    assert_debug_snapshot!(parse("$$?").unwrap());
    assert_debug_snapshot!(parse("$$ failed {}").unwrap());
    assert_debug_snapshot!(parse("foo()?").unwrap());
    assert_debug_snapshot!(parse("foo() failed {}").unwrap());
}

#[test]
fn test_blocks() {
    assert_debug_snapshot!(parse(
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
    )
    .unwrap());
}

#[test]
fn test_recovery() {
    // TODO: Add more tests
    assert_debug_snapshot!(parse_recover("fun foo(abc!) {}"));
    assert_debug_snapshot!(parse_recover(
        "
    5 + 5 +;
    echo 10"
    ));
}

#[test]
fn test_lexer() {
    let mut compiler = AmberCompiler::new();

    assert_debug_snapshot!(compiler.tokenize(r#"
        let x = "my \"interpolated\" string {name} end";
        "unclosed string

        abcd {let x = 10
    "#));
}
