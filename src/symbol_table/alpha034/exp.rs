use crate::{
    backend::Backend,
    grammar::{
        alpha034::{Expression, InterpolatedCommand, InterpolatedText},
        Spanned,
    },
    paths::FileId,
    symbol_table::{insert_symbol_reference, SymbolLocation, SymbolTable},
};

use super::stmnts::analyze_failure_handler;

pub fn analyze_exp(
    file_id: &FileId,
    (exp, _): &Spanned<Expression>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
) {
    match exp {
        Expression::FunctionInvocation((name, name_span), args, failure) => {
            insert_symbol_reference(
                &name,
                symbol_table,
                backend,
                &SymbolLocation {
                    file: *file_id,
                    start: name_span.start,
                    end: name_span.end,
                    is_public: false,
                },
            );

            args.iter().for_each(|arg| {
                analyze_exp(file_id, arg, symbol_table, backend);
            });

            if let Some(failure) = failure {
                analyze_failure_handler(file_id, failure, symbol_table, backend);
            }
        }
        Expression::Var((name, name_span)) => {
            insert_symbol_reference(
                &name,
                symbol_table,
                backend,
                &SymbolLocation {
                    file: *file_id,
                    start: name_span.start,
                    end: name_span.end,
                    is_public: false,
                },
            );
        }
        Expression::Add(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::And(exp1, _, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Array(elements) => {
            elements.iter().for_each(|exp| {
                analyze_exp(file_id, exp, symbol_table, backend);
            });
        }
        Expression::Cast(exp, _, _) => {
            analyze_exp(file_id, &exp, symbol_table, backend);
        }
        Expression::Command(inter_cmd, failure) => {
            inter_cmd.iter().for_each(|(inter_cmd, _)| match inter_cmd {
                InterpolatedCommand::Expression(exp) => {
                    analyze_exp(file_id, &exp, symbol_table, backend);
                }
                _ => {}
            });

            if let Some(failure) = failure {
                analyze_failure_handler(file_id, failure, symbol_table, backend);
            }
        }
        Expression::Divide(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Eq(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Ge(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Gt(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Is(_, exp, _) => {
            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Expression::Le(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Lt(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Modulo(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Multiply(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Nameof(_, exp) => {
            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Expression::Neg(_, exp) => {
            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Expression::Neq(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Not(_, exp) => {
            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Expression::Or(exp1, _, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Parentheses(exp) => {
            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Expression::Range(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Subtract(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Ternary(exp1, _, exp2, _, exp3) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
            analyze_exp(file_id, exp3, symbol_table, backend);
        }
        Expression::Text(int_text) => {
            int_text.iter().for_each(|(text, _)| match text {
                InterpolatedText::Expression(exp) => {
                    analyze_exp(file_id, exp, symbol_table, backend);
                }
                _ => {}
            });
        }
        _ => {}
    }
}
