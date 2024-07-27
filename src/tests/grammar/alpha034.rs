use crate::grammar::alpha034::grammar::*;

#[test]
fn test_text() {
    assert_eq!(
        parse("\"Hello, world!\"").unwrap(),
        Statement::Expression(Box::new(Expression::Text(
            (),
            vec![InterpolatedText::Text("Hello, world!".to_string())],
            ()
        )))
    );
    assert_eq!(
        parse("\"Hello, {name}!\"").unwrap(),
        Statement::Expression(Box::new(Expression::Text(
            (),
            vec![
                InterpolatedText::Text("Hello, ".to_string()),
                InterpolatedText::Expression(
                    (),
                    Box::new(Expression::VariableGet("name".to_string())),
                    ()
                ),
                InterpolatedText::Text("!".to_string())
            ],
            ()
        )))
    );
    assert_eq!(
        parse("\"Hello, {name}! How are you?\"").unwrap(),
        Statement::Expression(Box::new(Expression::Text(
            (),
            vec![
                InterpolatedText::Text("Hello, ".to_string()),
                InterpolatedText::Expression(
                    (),
                    Box::new(Expression::VariableGet("name".to_string())),
                    ()
                ),
                InterpolatedText::Text("! How are you?".to_string())
            ],
            ()
        )))
    );
    assert_eq!(
        parse(r#""\"text in quotes\" \\""#).unwrap(),
        Statement::Expression(Box::new(Expression::Text(
            (),
            vec![
                InterpolatedText::Escape((), "\"".to_string()),
                InterpolatedText::Text("text in quotes".to_string()),
                InterpolatedText::Escape((), '\"'.to_string()),
                InterpolatedText::Text(" ".to_string()),
                InterpolatedText::Escape((), '\\'.to_string()),
            ],
            ()
        )))
    );
}

#[test]
fn test_variable_get() {
    assert_eq!(
        parse("name").unwrap(),
        Statement::Expression(Box::new(Expression::VariableGet("name".to_string())))
    );
    assert_eq!(
        parse("name1").unwrap(),
        Statement::Expression(Box::new(Expression::VariableGet("name1".to_string())))
    );
    assert_eq!(
        parse("name_1").unwrap(),
        Statement::Expression(Box::new(Expression::VariableGet("name_1".to_string())))
    );
    assert_eq!(
        parse("name_1_").unwrap(),
        Statement::Expression(Box::new(Expression::VariableGet("name_1_".to_string())))
    );
    assert_eq!(
        parse("_var").unwrap(),
        Statement::Expression(Box::new(Expression::VariableGet("_var".to_string())))
    );
}

#[test]
fn test_number() {
    assert_eq!(parse("1").unwrap(), Statement::Expression(Box::new(Expression::Number(1.0))));
    assert_eq!(parse("1.0").unwrap(), Statement::Expression(Box::new(Expression::Number(1.0))));
    assert_eq!(parse("-1.0").unwrap(), Statement::Expression(Box::new(Expression::Number(-1.0))));
    assert_eq!(parse("-1.24").unwrap(), Statement::Expression(Box::new(Expression::Number(-1.24))));
    assert_eq!(parse("-5").unwrap(), Statement::Expression(Box::new(Expression::Number(-5.0))));
}

#[test]
fn test_whitespace() {
    assert_eq!(parse(" 0").unwrap(), Statement::Expression(Box::new(Expression::Number(0.0))));
    assert_eq!(parse("  0").unwrap(), Statement::Expression(Box::new(Expression::Number(0.0))));
    assert_eq!(parse("  0 ").unwrap(), Statement::Expression(Box::new(Expression::Number(0.0))));
}

#[test]
fn test_bool() {
    assert_eq!(parse("true").unwrap(), Statement::Expression(Box::new(Expression::Boolean(Boolean::True))));
    assert_eq!(parse("false").unwrap(), Statement::Expression(Box::new(Expression::Boolean(Boolean::False))));
}

#[test]
fn test_add() {
    assert_eq!(
        parse("1 + 2").unwrap(),
        Statement::Expression(Box::new(Expression::Add(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        )))
    );
    assert_eq!(
        parse("1 + 2 + 3").unwrap(),
        Statement::Expression(Box::new(Expression::Add(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        )))
    );
}

#[test]
fn test_subtract() {
    assert_eq!(
        parse("1 - 2").unwrap(),
        Statement::Expression(Box::new(Expression::Subtract(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 - 2 - 3").unwrap(),
        Statement::Expression(Box::new(Expression::Subtract(
            Box::new(Expression::Subtract(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        ))
    ));
}

#[test]
fn test_add_and_subtract() {
    assert_eq!(
        parse("1 + 2 - 3").unwrap(),
        Statement::Expression(Box::new(Expression::Subtract(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        )))
    );
    assert_eq!(
        parse("1 - 2 + 3").unwrap(),
        Statement::Expression(Box::new(Expression::Add(
            Box::new(Expression::Subtract(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        )))
    );
}

#[test]
fn test_multiply() {
    assert_eq!(
        parse("1 * 2").unwrap(),
        Statement::Expression(Box::new(Expression::Multiply(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 * 2 * 3").unwrap(),
        Statement::Expression(Box::new(Expression::Multiply(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        ))
    ));
}

#[test]
fn test_divide() {
    assert_eq!(
        parse("1 / 2").unwrap(),
        Statement::Expression(Box::new(Expression::Divide(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 / 2 / 3").unwrap(),
        Statement::Expression(Box::new(Expression::Divide(
            Box::new(Expression::Divide(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        ))
    ));
}

#[test]
fn test_multiply_and_divide(){
    assert_eq!(
        parse("1 * 2 / 3").unwrap(),
        Statement::Expression(Box::new(Expression::Divide(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        )))
    );
    assert_eq!(
        parse("1 / 2 * 3").unwrap(),
        Statement::Expression(Box::new(Expression::Multiply(
            Box::new(Expression::Divide(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        )))
    );
}

#[test]
fn test_mults_and_adds() {
    assert_eq!(
        parse("1 + 2 * 3").unwrap(),
        Statement::Expression(Box::new(Expression::Add(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                (),
                Box::new(Expression::Number(3.0))
            ))
        )))
    );
    assert_eq!(
        parse("1 * 2 + 3").unwrap(),
        Statement::Expression(Box::new(Expression::Add(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        )))
    );
    assert_eq!(
        parse("1 / 2 + 3").unwrap(),
        Statement::Expression(Box::new(Expression::Add(
            Box::new(Expression::Divide(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        )))
    );
    assert_eq!(
        parse("1 - 2 / 3").unwrap(),
        Statement::Expression(Box::new(Expression::Subtract(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Divide(
                Box::new(Expression::Number(2.0)),
                (),
                Box::new(Expression::Number(3.0))
            ))
        )))
    );
}

#[test]
fn test_modulo() {
    assert_eq!(
        parse("1 % 2").unwrap(),
        Statement::Expression(Box::new(Expression::Modulo(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 % 2 % 3").unwrap(),
        Statement::Expression(Box::new(Expression::Modulo(
            Box::new(Expression::Modulo(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(Expression::Number(3.0))
        ))
    ));
}

#[test]
fn test_neg() {
    assert_eq!(
        parse("-(1)").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Neg((),
                    Box::new(
                        Expression::Parentheses(
                            (), 
                            Box::new(Expression::Number(1.0)),
                            ()
                        )
                    )
                )
            )
        )
    );
    assert_eq!(
        parse("-(1 - 2)").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Neg((),
                    Box::new(
                        Expression::Parentheses(
                            (),
                            Box::new(
                                Expression::Subtract(
                                    Box::new(Expression::Number(1.0)),
                                    (),
                                    Box::new(Expression::Number(2.0))
                                )
                            ),
                            ()
                        )
                    )
                )
            )
        )
    );
}

#[test]
fn test_and() {
    assert_eq!(
        parse("true and false").unwrap(),
        Statement::Expression(Box::new(Expression::And(
            Box::new(Expression::Boolean(Boolean::True)),
            (),
            Box::new(Expression::Boolean(Boolean::False))
        ))
    ));
    assert_eq!(
        parse("true and false and true").unwrap(),
        Statement::Expression(Box::new(Expression::And(
            Box::new(Expression::And(
                Box::new(Expression::Boolean(Boolean::True)),
                (),
                Box::new(Expression::Boolean(Boolean::False))
            )),
            (),
            Box::new(Expression::Boolean(Boolean::True))
        ))
    ));
}

#[test]
fn test_or() {
    assert_eq!(
        parse("false or false").unwrap(),
        Statement::Expression(Box::new(Expression::Or(
            Box::new(Expression::Boolean(Boolean::False)),
            (),
            Box::new(Expression::Boolean(Boolean::False))
        ))
    ));
    assert_eq!(
        parse("false or false or true").unwrap(),
        Statement::Expression(Box::new(Expression::Or(
            Box::new(Expression::Or(
                Box::new(Expression::Boolean(Boolean::False)),
                (),
                Box::new(Expression::Boolean(Boolean::False))
            )),
            (),
            Box::new(Expression::Boolean(Boolean::True))
        ))
    ));
}

#[test]
fn test_gt() {
    assert_eq!(
        parse("1 > 2").unwrap(),
        Statement::Expression(Box::new(Expression::Gt(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 + 2 > 2 + 1").unwrap(),
        Statement::Expression(Box::new(Expression::Gt(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(2.0)),
                    (),
                    Box::new(Expression::Number(1.0))
                )
            )
        ))
    ));
    assert_eq!(
        parse("1 + 2 > 2 + 1 > 5").unwrap(),
        Statement::Expression(Box::new(Expression::Gt(
            Box::new(
                Expression::Gt(
                    Box::new(Expression::Add(
                        Box::new(Expression::Number(1.0)),
                        (),
                        Box::new(Expression::Number(2.0))
                    )),
                    (),
                    Box::new(
                        Expression::Add(
                            Box::new(Expression::Number(2.0)),
                            (),
                            Box::new(Expression::Number(1.0))
                        )
                    )
                )
            ),
            (),
            Box::new(Expression::Number(5.0))
        ))
    ));
}

#[test]
fn test_ge() {
    assert_eq!(
        parse("1 >= 2").unwrap(),
        Statement::Expression(Box::new(Expression::Ge(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 + 2 >= 2 + 1").unwrap(),
        Statement::Expression(Box::new(Expression::Ge(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(2.0)),
                    (),
                    Box::new(Expression::Number(1.0))
                )
            )
        ))
    ));
    assert_eq!(
        parse("1 + 2 >= 2 + 1 >= 5").unwrap(),
        Statement::Expression(Box::new(Expression::Ge(
            Box::new(
                Expression::Ge(
                    Box::new(Expression::Add(
                        Box::new(Expression::Number(1.0)),
                        (),
                        Box::new(Expression::Number(2.0))
                    )),
                    (),
                    Box::new(
                        Expression::Add(
                            Box::new(Expression::Number(2.0)),
                            (),
                            Box::new(Expression::Number(1.0))
                        )
                    )
                )
            ),
            (),
            Box::new(Expression::Number(5.0))
        ))
    ));
}

#[test]
fn test_lt() {
    assert_eq!(
        parse("1 < 2").unwrap(),
        Statement::Expression(Box::new(Expression::Lt(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 + 2 < 2 + 1").unwrap(),
        Statement::Expression(Box::new(Expression::Lt(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(2.0)),
                    (),
                    Box::new(Expression::Number(1.0))
                )
            )
        ))
    ));
    assert_eq!(
        parse("1 + 2 < 2 + 1 < 5").unwrap(),
        Statement::Expression(Box::new(Expression::Lt(
            Box::new(
                Expression::Lt(
                    Box::new(Expression::Add(
                        Box::new(Expression::Number(1.0)),
                        (),
                        Box::new(Expression::Number(2.0))
                    )),
                    (),
                    Box::new(
                        Expression::Add(
                            Box::new(Expression::Number(2.0)),
                            (),
                            Box::new(Expression::Number(1.0))
                        )
                    )
                )
            ),
            (),
            Box::new(Expression::Number(5.0))
        ))
    ));
}

#[test]
fn test_le() {
    assert_eq!(
        parse("1 <= 2").unwrap(),
        Statement::Expression(Box::new(Expression::Le(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 + 2 <= 2 + 1").unwrap(),
        Statement::Expression(Box::new(Expression::Le(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(2.0)),
                    (),
                    Box::new(Expression::Number(1.0))
                )
            )
        ))
    ));
    assert_eq!(
        parse("1 + 2 <= 2 + 1 <= 5").unwrap(),
        Statement::Expression(Box::new(Expression::Le(
            Box::new(
                Expression::Le(
                    Box::new(Expression::Add(
                        Box::new(Expression::Number(1.0)),
                        (),
                        Box::new(Expression::Number(2.0))
                    )),
                    (),
                    Box::new(
                        Expression::Add(
                            Box::new(Expression::Number(2.0)),
                            (),
                            Box::new(Expression::Number(1.0))
                        )
                    )
                )
            ),
            (),
            Box::new(Expression::Number(5.0))
        ))
    ));
}

#[test]
fn test_eq() {
    assert_eq!(
        parse("1 == 2").unwrap(),
        Statement::Expression(Box::new(Expression::Eq(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 + 2 == 2 + 1").unwrap(),
        Statement::Expression(Box::new(Expression::Eq(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(2.0)),
                    (),
                    Box::new(Expression::Number(1.0))
                )
            )
        ))
    ));
    assert_eq!(
        parse("1 + 2 == 2 + 1 + 5").unwrap(),
        Statement::Expression(Box::new(Expression::Eq(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Add(
                        Box::new(Expression::Number(2.0)),
                        (),
                        Box::new(Expression::Number(1.0))
                    )),
                    (),
                    Box::new(Expression::Number(5.0))
                )
            )
        ))
    ));
}

#[test]
fn test_neq() {
    assert_eq!(
        parse("1 != 2").unwrap(),
        Statement::Expression(Box::new(Expression::Neq(
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1 + 2 != 2 + 1").unwrap(),
        Statement::Expression(Box::new(Expression::Neq(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(2.0)),
                    (),
                    Box::new(Expression::Number(1.0))
                )
            )
        ))
    ));
    assert_eq!(
        parse("1 + 2 != 2 + 1 + 5").unwrap(),
        Statement::Expression(Box::new(Expression::Neq(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Add(
                        Box::new(Expression::Number(2.0)),
                        (),
                        Box::new(Expression::Number(1.0))
                    )),
                    (),
                    Box::new(Expression::Number(5.0))
                )
            )
        ))
    ));
}

#[test]
fn test_not() {
    assert_eq!(
        parse("not true").unwrap(),
        Statement::Expression(Box::new(Expression::Not(
            (),
            Box::new(Expression::Boolean(Boolean::True))
        )))
    );
    assert_eq!(
        parse("not not true").unwrap(),
        Statement::Expression(Box::new(Expression::Not(
            (),
            Box::new(Expression::Not(
                (),
                Box::new(Expression::Boolean(Boolean::True))
            ))
        )))
    );
}

#[test]
fn test_ternary() {
    assert_eq!(
        parse("true ? 1 : 2").unwrap(),
        Statement::Expression(Box::new(Expression::Ternary(
            Box::new(Expression::Boolean(Boolean::True)),
            (),
            Box::new(Expression::Number(1.0)),
            (),
            Box::new(Expression::Number(2.0))
        )))
    );
    assert_eq!(
        parse("true ? 1 + 2 : 2 + 1").unwrap(),
        Statement::Expression(Box::new(Expression::Ternary(
            Box::new(Expression::Boolean(Boolean::True)),
            (),
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(2.0)),
                    (),
                    Box::new(Expression::Number(1.0))
                )
            )
        )))
    );
    assert_eq!(
        parse("true ? 1 + 2 : false ? 5 : 6").unwrap(),
        Statement::Expression(Box::new(Expression::Ternary(
            Box::new(Expression::Boolean(Boolean::True)),
            (),
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                (),
                Box::new(Expression::Number(2.0))
            )),
            (),
            Box::new(
                Expression::Ternary(
                    Box::new(Expression::Boolean(Boolean::False)),
                    (),
                    Box::new(Expression::Number(5.0)),
                    (),
                    Box::new(Expression::Number(6.0))
                )
            )
        )))
    );
}

#[test]
fn test_command() {
    assert_eq!(
        parse(r#"$echo \"Hello, world!\"$"#).unwrap(),
        Statement::Expression(Box::new(Expression::Command(
            (),
            vec![
                InterpolatedCommand::Text("echo ".to_string()),
                InterpolatedCommand::Escape((), "\"".to_string()),
                InterpolatedCommand::Text("Hello, world!".to_string()),
                InterpolatedCommand::Escape((), "\"".to_string())
            ],
            ()
        )))
    );
    assert_eq!(
        parse("$echo \"Hello, {name}!\"$").unwrap(),
        Statement::Expression(Box::new(Expression::Command(
            (),
            vec![
                InterpolatedCommand::Text("echo \"Hello, ".to_string()),
                InterpolatedCommand::Expression(
                    (),
                    Box::new(Expression::VariableGet("name".to_string())),
                    ()
                ),
                InterpolatedCommand::Text("!\"".to_string()),
            ],
            ()
        )))
    );
    assert_eq!(
        parse("$command --arg1 -v$").unwrap(),
        Statement::Expression(Box::new(Expression::Command(
            (),
            vec![
                InterpolatedCommand::Text("command ".to_string()),
                InterpolatedCommand::CommandOption((), "arg1".to_string()),
                InterpolatedCommand::Text(" ".to_string()),
                InterpolatedCommand::CommandOption((), "v".to_string()),
            ],
            ()
        )))
    );
}

#[test]
fn test_array() {
    assert_eq!(
        parse("[1, 2, 3]").unwrap(),
        Statement::Expression(Box::new(Expression::Array(
            (),
            vec![
                Expression::Number(1.0),
                Expression::Number(2.0),
                Expression::Number(3.0),
            ],
            ()
        )))
    );
    assert_eq!(
        parse("[1]").unwrap(),
        Statement::Expression(Box::new(Expression::Array(
            (),
            vec![
                Expression::Number(1.0),
            ],
            ()
        )))
    );
}

#[test]
fn test_null() {
    assert_eq!(
        parse("null").unwrap(),
        Statement::Expression(Box::new(Expression::Null(())))
    );
}

#[test]
fn test_range() {
    assert_eq!(
        parse("1..2").unwrap(),
        Statement::Expression(Box::new(Expression::Range(
            Box::new(Expression::Number(1.0)),
            (),
            None,
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1..=2").unwrap(),
        Statement::Expression(Box::new(Expression::Range(
            Box::new(Expression::Number(1.0)),
            (),
            Some(()),
            Box::new(Expression::Number(2.0))
        ))
    ));
    assert_eq!(
        parse("1..2..3").unwrap(),
        Statement::Expression(Box::new(Expression::Range(
            Box::new(Expression::Range(
                Box::new(Expression::Number(1.0)),
                (),
                None,
                Box::new(Expression::Number(2.0))
            )),
            (),
            None,
            Box::new(Expression::Number(3.0))
        ))
    ));
}

#[test]
fn test_function_invocation() {
    assert_eq!(
        parse("func()").unwrap(),
        Statement::Expression(Box::new(Expression::FunctionInvocation(
            "func".to_string(),
            (),
            vec![],
            ()
        )))
    );
    assert_eq!(
        parse("func(1)").unwrap(),
        Statement::Expression(Box::new(Expression::FunctionInvocation(
            "func".to_string(),
            (),
            vec![Expression::Number(1.0)],
            ()
        )))
    );
    assert_eq!(
        parse("func(1, 2)").unwrap(),
        Statement::Expression(Box::new(Expression::FunctionInvocation(
            "func".to_string(),
            (),
            vec![Expression::Number(1.0), Expression::Number(2.0)],
            ()
        )))
    );
    assert_eq!(
        parse("func(1, 2, 3)").unwrap(),
        Statement::Expression(Box::new(Expression::FunctionInvocation(
            "func".to_string(),
            (),
            vec![Expression::Number(1.0), Expression::Number(2.0), Expression::Number(3.0)],
            ()
        )))
    );
}

#[test]
fn test_cast() {
    assert_eq!(
        parse("1 as Num").unwrap(),
        Statement::Expression(Box::new(Expression::Cast(
            Box::new(Expression::Number(1.0)),
            (),
            "Num".to_string()
        )))
    );
    assert_eq!(
        parse("1 as Num as Text").unwrap(),
        Statement::Expression(Box::new(Expression::Cast(
            Box::new(Expression::Cast(
                Box::new(Expression::Number(1.0)),
                (),
                "Num".to_string()
            )),
            (),
            "Text".to_string()
        )))
    );
}

#[test]
fn test_nameof() {
    assert_eq!(
        parse("nameof variable").unwrap(),
        Statement::Expression(Box::new(Expression::Nameof(
            (),
            Box::new(Expression::VariableGet("variable".to_string())),
        )))
    );
    assert_eq!(
        parse("nameof nameof variable").unwrap(),
        Statement::Expression(Box::new(Expression::Nameof(
            (),
            Box::new(Expression::Nameof(
                (),
                Box::new(Expression::VariableGet("variable".to_string())),
            )),
        )))
    );
}

#[test]
fn test_expr_precedence() {
    assert_eq!(
        parse("1 + 2 * 3").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(1.0)),
                    (),
                    Box::new(
                        Expression::Multiply(
                            Box::new(Expression::Number(2.0)),
                            (),
                            Box::new(Expression::Number(3.0))
                        )
                    )
                )
            )
        )
    );

    assert_eq!(
        parse("1 + 2 / 4 / 6").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Add(
                    Box::new(Expression::Number(1.0)),
                    (),
                    Box::new(
                        Expression::Divide(
                            Box::new(
                                Expression::Divide(
                                    Box::new(Expression::Number(2.0)),
                                    (),
                                    Box::new(Expression::Number(4.0))
                                )
                            ),
                            (),
                            Box::new(Expression::Number(6.0))
                        )
                    )
                )
            )
        )
    );

    assert_eq!(
        parse("2 - 3 - 4").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Subtract(
                    Box::new(
                        Expression::Subtract(
                            Box::new(Expression::Number(2.0)),
                            (),
                            Box::new(Expression::Number(3.0))
                        )
                    ),
                    (),
                    Box::new(Expression::Number(4.0))
                )
            )
        )
    );

    assert_eq!(
        parse("2 - (3 - 4)").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Subtract(
                    Box::new(Expression::Number(2.0)),
                    (),
                    Box::new(
                        Expression::Parentheses(
                            (),
                            Box::new(
                                Expression::Subtract(
                                    Box::new(Expression::Number(3.0)),
                                    (),
                                    Box::new(Expression::Number(4.0))
                                )
                            ),
                            ()
                        )
                    )
                )
            )
        )
    );

    assert_eq!(
        parse("-(2 + 3) * 5").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Multiply(
                    Box::new(
                        Expression::Neg(
                            (),
                            Box::new(
                                Expression::Parentheses(
                                    (),
                                    Box::new(
                                        Expression::Add(
                                            Box::new(Expression::Number(2.0)),
                                            (),
                                            Box::new(Expression::Number(3.0))
                                        )
                                    ),
                                    ()
                                )
                            )
                        )
                    ),
                    (),
                    Box::new(Expression::Number(5.0))
                )
            )
        )
    );

    assert_eq!(
        parse("-(2 + 3) * 5").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Multiply(
                    Box::new(
                        Expression::Neg(
                            (),
                            Box::new(
                                Expression::Parentheses(
                                    (),
                                    Box::new(
                                        Expression::Add(
                                            Box::new(Expression::Number(2.0)),
                                            (),
                                            Box::new(Expression::Number(3.0))
                                        )
                                    ),
                                    ()
                                )
                            )
                        )
                    ),
                    (),
                    Box::new(Expression::Number(5.0))
                )
            )
        )
    );

    assert_eq!(
        parse("(8+2)*(7-3)/2").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Divide(
                    Box::new(
                        Expression::Multiply(
                            Box::new(
                                Expression::Parentheses(
                                    (),
                                    Box::new(
                                        Expression::Add(
                                            Box::new(Expression::Number(8.0)),
                                            (),
                                            Box::new(Expression::Number(2.0))
                                        )
                                    ),
                                    ()
                                )
                            ),
                            (),
                            Box::new(
                                Expression::Parentheses(
                                    (),
                                    Box::new(
                                        Expression::Subtract(
                                            Box::new(Expression::Number(7.0)),
                                            (),
                                            Box::new(Expression::Number(3.0))
                                        )
                                    ),
                                    ()
                                )
                            )
                        )
                    ),
                    (),
                    Box::new(Expression::Number(2.0))
                )
            )
        )
    );

    assert_eq!(
        parse("2 / 3 + 1").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Add(
                    Box::new(
                        Expression::Divide(
                            Box::new(Expression::Number(2.0)),
                            (),
                            Box::new(Expression::Number(3.0))
                        )
                    ),
                    (),
                    Box::new(Expression::Number(1.0))
                )
            )
        )
    );

    assert_eq!(
        parse("25 / 5 * 3 + 7 - 2 * 4").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Subtract(
                    Box::new(
                        Expression::Add(
                            Box::new(
                                Expression::Multiply(
                                    Box::new(
                                        Expression::Divide(
                                            Box::new(Expression::Number(25.0)),
                                            (),
                                            Box::new(Expression::Number(5.0))
                                        )
                                    ),
                                    (),
                                    Box::new(Expression::Number(3.0))
                                )
                            ),
                            (),
                            Box::new(Expression::Number(7.0))
                        )
                    ),
                    (),
                    Box::new(
                        Expression::Multiply(
                            Box::new(Expression::Number(2.0)),
                            (),
                            Box::new(Expression::Number(4.0))
                        )
                    )
                )
            )
        )
    );

    assert_eq!(
        parse("
            2 + 5 > 3 + 4
                ? 15 + 10
                : 5 - 4 <= 1/2
                    ? 3 * 4
                    : 2"
        ).unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Ternary(
                    Box::new(
                        Expression::Gt(
                            Box::new(
                                Expression::Add(
                                    Box::new(Expression::Number(2.0)),
                                    (),
                                    Box::new(Expression::Number(5.0))
                                )
                            ),
                            (),
                            Box::new(
                                Expression::Add(
                                    Box::new(Expression::Number(3.0)),
                                    (),
                                    Box::new(Expression::Number(4.0))
                                )
                            )
                        )
                    ),
                    (),
                    Box::new(Expression::Add(
                        Box::new(Expression::Number(15.0)),
                        (),
                        Box::new(Expression::Number(10.0))
                    )),
                    (),
                    Box::new(
                        Expression::Ternary(
                            Box::new(
                                Expression::Le(
                                    Box::new(
                                        Expression::Subtract(
                                            Box::new(Expression::Number(5.0)),
                                            (),
                                            Box::new(Expression::Number(4.0))
                                        )
                                    ),
                                    (),
                                    Box::new(
                                        Expression::Divide(
                                            Box::new(Expression::Number(1.0)),
                                            (),
                                            Box::new(Expression::Number(2.0))
                                        )
                                    )
                                )
                            ),
                            (),
                            Box::new(
                                Expression::Multiply(
                                    Box::new(Expression::Number(3.0)),
                                    (),
                                    Box::new(Expression::Number(4.0))
                                )
                            ),
                            (),
                            Box::new(Expression::Number(2.0))
                        )
                    )
                )
            )
        )
    );

    assert_eq!(
        parse("true or false and true and true or false").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Or(
                    Box::new(
                        Expression::Or(
                            Box::new(
                                Expression::Boolean(Boolean::True)
                            ),
                            (),
                            Box::new(
                                Expression::And(
                                    Box::new(
                                        Expression::And(
                                            Box::new(
                                                Expression::Boolean(Boolean::False)
                                            ),
                                            (),
                                            Box::new(
                                                Expression::Boolean(Boolean::True)
                                            )
                                        )
                                    ),
                                    (),
                                    Box::new(
                                        Expression::Boolean(Boolean::True)
                                    )
                                )
                            )
                        )
                    ),
                    (),
                    Box::new(
                        Expression::Boolean(Boolean::False)
                    )
                )
            )
        )
    );

    assert_eq!(
        parse("true as Bool as Text as Num * 2 / foo()").unwrap(),
        Statement::Expression(
            Box::new(
                Expression::Divide(
                    Box::new(
                        Expression::Multiply(
                            Box::new(
                                Expression::Cast(
                                    Box::new(
                                        Expression::Cast(
                                            Box::new(
                                                Expression::Cast(
                                                    Box::new(
                                                        Expression::Boolean(Boolean::True)
                                                    ),
                                                    (),
                                                    "Bool".to_string()
                                                )
                                            ),
                                            (),
                                            "Text".to_string()
                                        )
                                    ),
                                    (),
                                    "Num".to_string()
                                )
                            ),
                            (),
                            Box::new(Expression::Number(2.0))
                        )
                    ),
                    (),
                    Box::new(
                        Expression::FunctionInvocation(
                            "foo".to_string(),
                            (),
                            vec![],
                            ()
                        )
                    )
                )
            )
        )
    );
}
