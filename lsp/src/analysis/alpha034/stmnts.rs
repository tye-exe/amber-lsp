use crate::{
    analysis::{
        get_symbol_definition_info, insert_symbol_definition, insert_symbol_reference,
        types::{make_union_type, matches_type, GenericsMap},
        BlockContext, Context, DataType, SymbolInfo, SymbolLocation, SymbolType, VariableSymbol,
    },
    files::{FileVersion, Files},
    grammar::{alpha034::*, Spanned},
    paths::FileId,
};

use super::exp::analyze_exp;

/// Analyze a statement.
///
/// Returns the data type of the return statement.
#[tracing::instrument(skip_all)]
pub fn analyze_stmnt(
    file_id: FileId,
    file_version: FileVersion,
    (stmnt, span): &Spanned<Statement>,
    files: &Files,
    scope_end: usize,
    scoped_generic_types: &GenericsMap,
    contexts: &Vec<Context>,
) -> Option<DataType> {
    let file = (file_id, file_version);

    match stmnt {
        Statement::Block(block) => {
            return analyze_block(
                file_id,
                file_version,
                block,
                files,
                scoped_generic_types,
                contexts,
            );
        }
        Statement::IfChain(_, if_chain) => {
            for (if_chain_content, _) in if_chain.iter() {
                match if_chain_content {
                    IfChainContent::IfCondition((condition, _)) => match condition {
                        IfCondition::IfCondition(exp, block) => {
                            analyze_exp(
                                file_id,
                                file_version,
                                exp,
                                DataType::Boolean,
                                files,
                                scoped_generic_types,
                                contexts,
                            );
                            return analyze_block(
                                file_id,
                                file_version,
                                block,
                                files,
                                scoped_generic_types,
                                contexts,
                            );
                        }
                        IfCondition::InlineIfCondition(exp, boxed_stmnt) => {
                            analyze_exp(
                                file_id,
                                file_version,
                                exp,
                                DataType::Boolean,
                                files,
                                scoped_generic_types,
                                contexts,
                            );
                            return analyze_stmnt(
                                file_id,
                                file_version,
                                boxed_stmnt,
                                files,
                                boxed_stmnt.1.end,
                                scoped_generic_types,
                                contexts,
                            );
                        }
                        _ => {}
                    },
                    IfChainContent::Else((else_cond, _)) => match else_cond {
                        ElseCondition::Else(_, block) => {
                            return analyze_block(
                                file_id,
                                file_version,
                                block,
                                files,
                                scoped_generic_types,
                                contexts,
                            );
                        }
                        ElseCondition::InlineElse(_, stmnt) => {
                            return analyze_stmnt(
                                file_id,
                                file_version,
                                stmnt,
                                files,
                                stmnt.1.end,
                                scoped_generic_types,
                                contexts,
                            );
                        }
                    },
                    IfChainContent::Comment(_) => {}
                }
            }
        }
        Statement::IfCondition(_, if_cond, _, else_cond) => {
            match &if_cond.0 {
                IfCondition::IfCondition(exp, block) => {
                    analyze_exp(
                        file_id,
                        file_version,
                        exp,
                        DataType::Boolean,
                        files,
                        scoped_generic_types,
                        contexts,
                    );
                    return analyze_block(
                        file_id,
                        file_version,
                        block,
                        files,
                        scoped_generic_types,
                        contexts,
                    );
                }
                IfCondition::InlineIfCondition(exp, boxed_stmnt) => {
                    analyze_exp(
                        file_id,
                        file_version,
                        exp,
                        DataType::Boolean,
                        files,
                        scoped_generic_types,
                        contexts,
                    );
                    return analyze_stmnt(
                        file_id,
                        file_version,
                        boxed_stmnt,
                        files,
                        boxed_stmnt.1.end,
                        scoped_generic_types,
                        contexts,
                    );
                }
                _ => {}
            }

            if let Some(else_cond) = else_cond {
                match &else_cond.0 {
                    ElseCondition::Else(_, block) => {
                        return analyze_block(
                            file_id,
                            file_version,
                            block,
                            files,
                            scoped_generic_types,
                            contexts,
                        );
                    }
                    ElseCondition::InlineElse(_, stmnt) => {
                        return analyze_stmnt(
                            file_id,
                            file_version,
                            stmnt,
                            files,
                            stmnt.1.end,
                            scoped_generic_types,
                            contexts,
                        );
                    }
                }
            }
        }
        Statement::InfiniteLoop(_, block) => {
            let mut new_contexts = contexts.clone();
            new_contexts.push(Context::Loop);
            return analyze_block(
                file_id,
                file_version,
                block,
                files,
                scoped_generic_types,
                &new_contexts,
            );
        }
        Statement::IterLoop(_, (vars, _), _, exp, block) => {
            let block_span = block.1;

            let iter_type = match analyze_exp(
                file_id,
                file_version,
                exp,
                DataType::Array(Box::new(DataType::Any)),
                files,
                scoped_generic_types,
                contexts,
            ) {
                DataType::Array(ty) => *ty,
                _ => DataType::Any,
            };

            match &vars {
                IterLoopVars::WithIndex((var1, var1_span), (var2, var2_span)) => {
                    let mut symbol_table = files.symbol_table.entry(file).or_default();
                    insert_symbol_definition(
                        &mut symbol_table,
                        &SymbolInfo {
                            name: var1.clone(),
                            data_type: DataType::Number,
                            symbol_type: SymbolType::Variable(VariableSymbol { is_const: false }),
                            is_definition: true,
                            undefined: false,
                            span: *var1_span,
                            contexts: contexts.clone(),
                        },
                        file,
                        block_span.start..=block_span.end,
                        false,
                    );

                    insert_symbol_definition(
                        &mut symbol_table,
                        &SymbolInfo {
                            name: var2.to_string(),
                            symbol_type: SymbolType::Variable(VariableSymbol { is_const: false }),
                            data_type: iter_type,
                            is_definition: true,
                            undefined: false,
                            span: *var2_span,
                            contexts: contexts.clone(),
                        },
                        file,
                        block_span.start..=block_span.end,
                        false,
                    );
                }
                IterLoopVars::Single((var, var_span)) => {
                    let mut symbol_table = files.symbol_table.entry(file).or_default();
                    insert_symbol_definition(
                        &mut symbol_table,
                        &SymbolInfo {
                            name: var.to_string(),
                            symbol_type: SymbolType::Variable(VariableSymbol { is_const: false }),
                            data_type: iter_type,
                            is_definition: true,
                            undefined: false,
                            span: *var_span,
                            contexts: contexts.clone(),
                        },
                        file,
                        block_span.start..=block_span.end,
                        false,
                    );
                }
                _ => {}
            }

            let mut new_contexts = contexts.clone();
            new_contexts.push(Context::Loop);
            return analyze_block(
                file_id,
                file_version,
                block,
                files,
                scoped_generic_types,
                &new_contexts,
            );
        }
        Statement::VariableInit(_, (var_name, var_span), (value, _)) => {
            let var_type = match value {
                VariableInitType::Expression(exp) => analyze_exp(
                    file_id,
                    file_version,
                    exp,
                    DataType::Any,
                    files,
                    scoped_generic_types,
                    contexts,
                ),
                VariableInitType::DataType((ty, _)) => ty.clone(),
                _ => DataType::Error,
            };

            let mut symbol_table = files.symbol_table.entry(file).or_default();
            insert_symbol_definition(
                &mut symbol_table,
                &SymbolInfo {
                    name: var_name.to_string(),
                    symbol_type: SymbolType::Variable(VariableSymbol { is_const: false }),
                    data_type: scoped_generic_types.deref_type(&var_type),
                    is_definition: true,
                    undefined: false,
                    span: *var_span,
                    contexts: contexts.clone(),
                },
                file,
                span.end..=scope_end,
                false,
            );
        }
        Statement::ConstInit(_, (var_name, var_span), exp) => {
            let var_type = analyze_exp(
                file_id,
                file_version,
                exp,
                DataType::Any,
                files,
                scoped_generic_types,
                contexts,
            );

            let mut symbol_table = files.symbol_table.entry(file).or_default();
            insert_symbol_definition(
                &mut symbol_table,
                &SymbolInfo {
                    name: var_name.to_string(),
                    symbol_type: SymbolType::Variable(VariableSymbol { is_const: true }),
                    data_type: scoped_generic_types.deref_type(&var_type),
                    is_definition: true,
                    undefined: false,
                    span: *var_span,
                    contexts: contexts.clone(),
                },
                file,
                span.end..=scope_end,
                false,
            );
        }
        Statement::Echo(_, exp) => {
            analyze_exp(
                file_id,
                file_version,
                exp,
                DataType::Any,
                files,
                scoped_generic_types,
                contexts,
            );
        }
        Statement::Expression(exp) => {
            analyze_exp(
                file_id,
                file_version,
                exp,
                DataType::Any,
                files,
                scoped_generic_types,
                contexts,
            );
        }
        Statement::Fail(_, exp) => {
            if !contexts
                .iter()
                .any(|c| matches!(c, Context::Function(_) | Context::Main))
            {
                files.report_error(
                    &file,
                    "Fail statements can only be used inside of functions or the main block",
                    *span,
                );
            }

            if let Some(exp) = exp {
                analyze_exp(
                    file_id,
                    file_version,
                    exp,
                    DataType::Number,
                    files,
                    scoped_generic_types,
                    contexts,
                );
            }
        }
        Statement::Return(_, exp) => {
            if !contexts.iter().any(|c| matches!(c, Context::Function(_))) {
                files.report_error(&file, "Return statement outside of function", *span);
            }

            if let Some(exp) = exp {
                let ty = analyze_exp(
                    file_id,
                    file_version,
                    exp,
                    DataType::Any,
                    files,
                    scoped_generic_types,
                    contexts,
                );

                return Some(ty);
            }

            return Some(DataType::Null);
        }
        Statement::ShorthandAdd((var, var_span), exp) => {
            let var_ty = match get_symbol_definition_info(files, var, &file, var_span.start) {
                Some(info) => info.data_type,
                None => DataType::Any,
            };

            let default_ty = DataType::Union(vec![
                DataType::Text,
                DataType::Number,
                DataType::Array(Box::new(DataType::Union(vec![
                    DataType::Text,
                    DataType::Number,
                ]))),
            ]);

            let exp_ty = analyze_exp(
                file_id,
                file_version,
                exp,
                var_ty.clone(),
                files,
                scoped_generic_types,
                contexts,
            );

            if !matches_type(&default_ty, &var_ty, scoped_generic_types)
                || !matches_type(&exp_ty, &var_ty, scoped_generic_types)
            {
                files.report_error(
                    &file,
                    &format!(
                        "Cannot add to variable of type {}",
                        var_ty.to_string(scoped_generic_types)
                    ),
                    *var_span,
                );
            }

            insert_symbol_reference(
                var,
                files,
                &SymbolLocation {
                    file,
                    start: var_span.start,
                    end: var_span.end,
                },
                scoped_generic_types,
                contexts,
            );
        }
        Statement::ShorthandDiv((var, var_span), exp) => {
            let var_ty = match get_symbol_definition_info(files, var, &file, var_span.start) {
                Some(info) => info.data_type,
                None => DataType::Any,
            };

            if !matches_type(&DataType::Number, &var_ty, scoped_generic_types) {
                files.report_error(
                    &file,
                    &format!(
                        "Cannot divide variable of type {}",
                        var_ty.to_string(scoped_generic_types)
                    ),
                    *var_span,
                );
            }

            analyze_exp(
                file_id,
                file_version,
                exp,
                DataType::Number,
                files,
                scoped_generic_types,
                contexts,
            );

            insert_symbol_reference(
                var,
                files,
                &SymbolLocation {
                    file,
                    start: var_span.start,
                    end: var_span.end,
                },
                scoped_generic_types,
                contexts,
            );
        }
        Statement::ShorthandModulo((var, var_span), exp) => {
            let var_ty = match get_symbol_definition_info(files, var, &file, var_span.start) {
                Some(info) => info.data_type,
                None => DataType::Any,
            };

            if !matches_type(&DataType::Number, &var_ty, scoped_generic_types) {
                files.report_error(
                    &file,
                    &format!(
                        "Cannot use modulo with variable of type {}",
                        var_ty.to_string(scoped_generic_types)
                    ),
                    *var_span,
                );
            }

            analyze_exp(
                file_id,
                file_version,
                exp,
                DataType::Number,
                files,
                scoped_generic_types,
                contexts,
            );

            insert_symbol_reference(
                var,
                files,
                &SymbolLocation {
                    file,
                    start: var_span.start,
                    end: var_span.end,
                },
                scoped_generic_types,
                contexts,
            );
        }
        Statement::ShorthandMul((var, var_span), exp) => {
            let var_ty = match get_symbol_definition_info(files, var, &file, var_span.start) {
                Some(info) => info.data_type,
                None => DataType::Any,
            };

            if !matches_type(&DataType::Number, &var_ty, scoped_generic_types) {
                files.report_error(
                    &file,
                    &format!(
                        "Cannot use multiply with variable of type {}",
                        var_ty.to_string(scoped_generic_types)
                    ),
                    *var_span,
                );
            }

            analyze_exp(
                file_id,
                file_version,
                exp,
                DataType::Number,
                files,
                scoped_generic_types,
                contexts,
            );

            insert_symbol_reference(
                var,
                files,
                &SymbolLocation {
                    file,
                    start: var_span.start,
                    end: var_span.end,
                },
                scoped_generic_types,
                contexts,
            );
        }
        Statement::ShorthandSub((var, var_span), exp) => {
            let var_ty = match get_symbol_definition_info(files, var, &file, var_span.start) {
                Some(info) => info.data_type,
                None => DataType::Any,
            };

            if !matches_type(&DataType::Number, &var_ty, scoped_generic_types) {
                files.report_error(
                    &file,
                    &format!(
                        "Cannot use subtract with variable of type {}",
                        var_ty.to_string(scoped_generic_types)
                    ),
                    *var_span,
                );
            }

            analyze_exp(
                file_id,
                file_version,
                exp,
                DataType::Number,
                files,
                scoped_generic_types,
                contexts,
            );

            insert_symbol_reference(
                var,
                files,
                &SymbolLocation {
                    file,
                    start: var_span.start,
                    end: var_span.end,
                },
                scoped_generic_types,
                contexts,
            );
        }
        Statement::VariableSet((var, var_span), exp) => {
            let var_ty = match get_symbol_definition_info(files, var, &file, var_span.start) {
                Some(info) => info.data_type,
                None => DataType::Any,
            };

            analyze_exp(
                file_id,
                file_version,
                exp,
                var_ty,
                files,
                scoped_generic_types,
                contexts,
            );

            insert_symbol_reference(
                var,
                files,
                &SymbolLocation {
                    file,
                    start: var_span.start,
                    end: var_span.end,
                },
                scoped_generic_types,
                contexts,
            );
        }
        Statement::Break => {
            if !contexts.iter().any(|c| matches!(c, Context::Loop)) {
                files.report_error(&file, "Break statement outside of loop", *span);
            }
        }
        Statement::Continue => {
            if !contexts.iter().any(|c| matches!(c, Context::Loop)) {
                files.report_error(&file, "Continue statement outside of loop", *span);
            }
        }
        Statement::Comment(_) | Statement::Shebang(_) | Statement::Error => {}
    };

    None
}

pub fn analyze_block(
    file_id: FileId,
    file_version: FileVersion,
    (block, span): &Spanned<Block>,
    files: &Files,
    scoped_generic_types: &GenericsMap,
    contexts: &[Context],
) -> Option<DataType> {
    let mut types: Vec<DataType> = vec![];

    if let Block::Block(modifiers, stmnt) = block {
        let mut new_contexts = contexts.to_owned();
        new_contexts.push(Context::Block(BlockContext {
            modifiers: modifiers.iter().map(|(m, _)| m.clone()).collect(),
        }));

        for stmnt in stmnt.iter() {
            if let Some(ty) = analyze_stmnt(
                file_id,
                file_version,
                stmnt,
                files,
                span.end,
                scoped_generic_types,
                &new_contexts,
            ) {
                types.push(ty);
            }
        }
    }

    if types.is_empty() {
        None
    } else {
        Some(make_union_type(types))
    }
}

pub fn analyze_failure_handler(
    file_id: FileId,
    file_version: FileVersion,
    (failure, span): &Spanned<FailureHandler>,
    files: &Files,
    scoped_generic_types: &GenericsMap,
    contexts: &Vec<Context>,
) {
    if let FailureHandler::Handle(_, stmnts) = failure {
        stmnts.iter().for_each(|stmnt| {
            analyze_stmnt(
                file_id,
                file_version,
                stmnt,
                files,
                span.end,
                scoped_generic_types,
                contexts,
            );
        });
    }
}
