use crate::{
    backend::Backend,
    grammar::{alpha034::*, Spanned},
    paths::FileId,
    symbol_table::{
        insert_symbol_definition, insert_symbol_reference, DataType, SymbolInfo, SymbolLocation,
        SymbolTable, SymbolType,
    },
};

use super::exp::analyze_exp;

pub fn analyze_stmnt(
    file_id: &FileId,
    (stmnt, span): &Spanned<Statement>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
    scope_end: usize,
) {
    match stmnt {
        Statement::Block(block) => analyze_block(file_id, block, symbol_table, backend),
        Statement::IfChain(_, if_chain) => {
            for (if_chain_content, _) in if_chain.iter() {
                match if_chain_content {
                    IfChainContent::IfCondition((condition, _)) => match condition {
                        IfCondition::IfCondition(exp, block) => {
                            analyze_exp(file_id, exp, symbol_table, backend);
                            return analyze_block(file_id, block, symbol_table, backend);
                        }
                        IfCondition::InlineIfCondition(exp, boxed_stmnt) => {
                            analyze_exp(file_id, exp, symbol_table, backend);
                            return analyze_stmnt(
                                file_id,
                                boxed_stmnt,
                                symbol_table,
                                backend,
                                boxed_stmnt.1.end,
                            );
                        }
                        _ => {}
                    },
                    IfChainContent::Else((else_cond, _)) => match else_cond {
                        ElseCondition::Else(_, block) => {
                            return analyze_block(file_id, block, symbol_table, backend)
                        }
                        ElseCondition::InlineElse(_, stmnt) => {
                            return analyze_stmnt(
                                file_id,
                                stmnt,
                                symbol_table,
                                backend,
                                stmnt.1.end,
                            )
                        }
                    },
                }
            }
        }
        Statement::IfCondition(_, if_cond, else_cond) => {
            match &if_cond.0 {
                IfCondition::IfCondition(exp, block) => {
                    analyze_exp(file_id, exp, symbol_table, backend);
                    return analyze_block(file_id, block, symbol_table, backend);
                }
                IfCondition::InlineIfCondition(exp, boxed_stmnt) => {
                    analyze_exp(file_id, exp, symbol_table, backend);
                    return analyze_stmnt(
                        file_id,
                        boxed_stmnt,
                        symbol_table,
                        backend,
                        boxed_stmnt.1.end,
                    );
                }
                _ => {}
            }

            if let Some(else_cond) = else_cond {
                match &else_cond.0 {
                    ElseCondition::Else(_, block) => {
                        return analyze_block(file_id, block, symbol_table, backend)
                    }
                    ElseCondition::InlineElse(_, stmnt) => {
                        return analyze_stmnt(file_id, stmnt, symbol_table, backend, stmnt.1.end)
                    }
                }
            }
        }
        Statement::InfiniteLoop(_, block) => analyze_block(file_id, block, symbol_table, backend),
        Statement::IterLoop(_, (vars, _), _, exp, block) => {
            let block_span = block.1.clone();
            match &vars {
                IterLoopVars::WithIndex((var1, var1_span), (var2, var2_span)) => {
                    symbol_table.symbols.insert(
                        var1_span.start..=var1_span.end,
                        SymbolInfo {
                            name: var1.clone(),
                            symbol_type: SymbolType::Variable,
                            data_type: DataType::Number,
                            arguments: None,
                            is_public: false,
                            is_definition: true,
                            undefined: false,
                        },
                    );

                    insert_symbol_definition(
                        symbol_table,
                        var1,
                        block_span.start..=block_span.end,
                        &SymbolLocation {
                            file: *file_id,
                            start: var1_span.start,
                            end: var1_span.end,
                            is_public: false,
                        },
                    );

                    symbol_table.symbols.insert(
                        var2_span.start..=var2_span.end,
                        SymbolInfo {
                            name: var2.clone(),
                            symbol_type: SymbolType::Variable,
                            data_type: DataType::Number,
                            arguments: None,
                            is_public: false,
                            is_definition: true,
                            undefined: false,
                        },
                    );

                    insert_symbol_definition(
                        symbol_table,
                        var2,
                        block_span.start..=block_span.end,
                        &SymbolLocation {
                            file: *file_id,
                            start: var2_span.start,
                            end: var2_span.end,
                            is_public: false,
                        },
                    );
                }
                IterLoopVars::Single((var, var_span)) => {
                    symbol_table.symbols.insert(
                        var_span.start..=var_span.end,
                        SymbolInfo {
                            name: var.clone(),
                            symbol_type: SymbolType::Variable,
                            data_type: DataType::Number,
                            arguments: None,
                            is_public: false,
                            is_definition: true,
                            undefined: false,
                        },
                    );

                    insert_symbol_definition(
                        symbol_table,
                        var,
                        block_span.start..=block_span.end,
                        &SymbolLocation {
                            file: *file_id,
                            start: var_span.start,
                            end: var_span.end,
                            is_public: false,
                        },
                    );
                }
                _ => {}
            }

            analyze_exp(file_id, exp, symbol_table, backend);
            analyze_block(file_id, block, symbol_table, backend);
        }
        Statement::VariableInit(_, (var_name, var_span), exp) => {
            symbol_table.symbols.insert(
                var_span.start..=var_span.end,
                SymbolInfo {
                    name: var_name.clone(),
                    symbol_type: SymbolType::Variable,
                    data_type: DataType::Any, // TODO: Implement type checker
                    arguments: None,
                    is_public: false,
                    is_definition: true,
                    undefined: false,
                },
            );

            insert_symbol_definition(
                symbol_table,
                var_name,
                span.end..=scope_end,
                &SymbolLocation {
                    file: *file_id,
                    start: var_span.start,
                    end: var_span.end,
                    is_public: false,
                },
            );

            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Statement::Echo(_, exp) => analyze_exp(file_id, exp, symbol_table, backend),
        Statement::Expression(exp) => analyze_exp(file_id, exp, symbol_table, backend),
        Statement::Fail(_, exp) => {
            if let Some(exp) = exp {
                analyze_exp(file_id, exp, symbol_table, backend);
            }
        }
        Statement::Return(_, exp) => {
            if let Some(exp) = exp {
                analyze_exp(file_id, exp, symbol_table, backend);
            }
        }
        Statement::ShorthandAdd((var, var_span), exp) => {
            insert_symbol_reference(
                &var,
                symbol_table,
                backend,
                &SymbolLocation {
                    file: *file_id,
                    start: var_span.start,
                    end: var_span.end,
                    is_public: false,
                },
            );

            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Statement::ShorthandDiv((var, var_span), exp) => {
            insert_symbol_reference(
                &var,
                symbol_table,
                backend,
                &SymbolLocation {
                    file: *file_id,
                    start: var_span.start,
                    end: var_span.end,
                    is_public: false,
                },
            );

            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Statement::ShorthandModulo((var, var_span), exp) => {
            insert_symbol_reference(
                &var,
                symbol_table,
                backend,
                &SymbolLocation {
                    file: *file_id,
                    start: var_span.start,
                    end: var_span.end,
                    is_public: false,
                },
            );

            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Statement::ShorthandMul((var, var_span), exp) => {
            insert_symbol_reference(
                &var,
                symbol_table,
                backend,
                &SymbolLocation {
                    file: *file_id,
                    start: var_span.start,
                    end: var_span.end,
                    is_public: false,
                },
            );

            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Statement::ShorthandSub((var, var_span), exp) => {
            insert_symbol_reference(
                &var,
                symbol_table,
                backend,
                &SymbolLocation {
                    file: *file_id,
                    start: var_span.start,
                    end: var_span.end,
                    is_public: false,
                },
            );

            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Statement::VariableSet((var, var_span), exp) => {
            insert_symbol_reference(
                &var,
                symbol_table,
                backend,
                &SymbolLocation {
                    file: *file_id,
                    start: var_span.start,
                    end: var_span.end,
                    is_public: false,
                },
            );

            analyze_exp(file_id, exp, symbol_table, backend);
        }
        _ => {}
    }
}

pub fn analyze_block(
    file_id: &FileId,
    (block, span): &Spanned<Block>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
) {
    if let Block::Block(stmnt) = block {
        for stmnt in stmnt.iter() {
            analyze_stmnt(file_id, stmnt, symbol_table, backend, span.end);
        }
    }
}

pub fn analyze_failure_handler(
    file_id: &FileId,
    (failure, span): &Spanned<FailureHandler>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
) {
    match failure {
        FailureHandler::Handle(_, stmnts) => {
            stmnts.iter().for_each(|stmnt| {
                analyze_stmnt(file_id, stmnt, symbol_table, backend, span.end);
            });
        }
        _ => {}
    }
}
