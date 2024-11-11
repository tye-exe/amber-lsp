use std::{fs, ops::Range, path::Path};

use chumsky::primitive::Container;
use rangemap::RangeMap;
use std::io::Write;
use tower_lsp::lsp_types::Url;
use std::path::PathBuf;

use crate::{
    backend::Backend, paths::FileId, DataType, SymbolInfo, SymbolLocation, SymbolTable, SymbolType,
};

use super::{
    Block, ElseCondition, Expression, FailureHandler, FunctionArgument, GlobalStatement,
    IfChainContent, IfCondition, ImportContent, InterpolatedCommand, InterpolatedText,
    IterLoopVars, Spanned, Statement,
};

pub fn map_type(data_type: &str) -> DataType {
    match data_type {
        "Text" => DataType::Text,
        "Num" => DataType::Number,
        "Bool" => DataType::Boolean,
        "Null" => DataType::Null,
        _ => {
            if data_type.starts_with("[") && data_type.ends_with("]") {
                let inner_ty = data_type[1..data_type.len() - 1].to_string();

                return match inner_ty.as_str() {
                    "Text" => DataType::Array(Box::new(DataType::Text)),
                    "Num" => DataType::Array(Box::new(DataType::Number)),
                    "Bool" => DataType::Array(Box::new(DataType::Boolean)),
                    "Null" => DataType::Array(Box::new(DataType::Null)),
                    _ => DataType::Array(Box::new(DataType::Any)),
                };
            }

            DataType::Any
        }
    }
}

#[cfg(target_os = "linux")]
fn get_install_dir() -> PathBuf {
    PathBuf::from("/etc/amber_lsp")
}

#[cfg(target_os = "windows")]
fn get_install_dir() -> PathBuf {
    PathBuf::from("C:\\Program Files\\amber_lsp")
}

#[cfg(target_os = "macos")]
fn get_install_dir() -> PathBuf {
    PathBuf::from("/usr/local/etc/amber_lsp")
}

pub fn map_import_path(uri: &Url, path: &str) -> Url {
    if path == "std" {
        let std_file = get_install_dir().join("resources/alpha034/std/main.ab");

        return Url::from_file_path(std_file).unwrap();
    }

    let path = uri.to_file_path().unwrap().parent().unwrap().join(path);

    Url::from_file_path(path).unwrap()
}

fn insert_symbol_definition(
    symbol_table: &mut SymbolTable,
    symbol: &str,
    definition_scope: Range<usize>,
    definition_location: &SymbolLocation,
) {
    let mut symbol_definitions = match symbol_table.definitions.get_mut(symbol) {
        Some(symbol_definitions) => symbol_definitions,
        None => {
            symbol_table
                .definitions
                .insert(symbol.to_string(), RangeMap::new());

            symbol_table.definitions.get_mut(symbol).unwrap()
        }
    };

    symbol_definitions.insert(definition_scope, definition_location.clone());
}

fn insert_symbol_reference(
    symbol: &str,
    current_file_symbol_table: &mut SymbolTable,
    backend: &Backend,
    reference_location: &SymbolLocation,
) {
    let span = reference_location.start..reference_location.end;

    let symbol_definition = match current_file_symbol_table.definitions.get(symbol) {
        Some(symbol_definitions) => symbol_definitions.get(&span.start).cloned(),
        None => None,
    };

    match symbol_definition {
        Some(definition) => {
            let symbol_info = if definition.file == reference_location.file {
                current_file_symbol_table.symbols.get(&definition.start).cloned().unwrap()
            } else {
                let definition_file_symbol_table = backend.symbol_table.get_mut(&definition.file).unwrap();

                definition_file_symbol_table
                    .symbols
                    .get(&definition.start)
                    .cloned()
                    .unwrap()
            };

            current_file_symbol_table.symbols.insert(
                span.clone(),
                SymbolInfo {
                    name: symbol.to_string(),
                    symbol_type: symbol_info.symbol_type.clone(),
                    data_type: symbol_info.data_type.clone(),
                    arguments: symbol_info.arguments.clone(),
                    is_public: symbol_info.is_public,
                    is_definition: false,
                    undefined: false,
                },
            );
        }
        None => {
            current_file_symbol_table.symbols.insert(
                span.clone(),
                SymbolInfo {
                    name: symbol.to_string(),
                    symbol_type: SymbolType::Variable,
                    data_type: DataType::Any,
                    arguments: None,
                    is_public: false,
                    is_definition: false,
                    undefined: true,
                },
            );
        }
    }

    let mut symbol_references = match current_file_symbol_table.references.get_mut(symbol) {
        Some(symbol_references) => symbol_references,
        None => {
            current_file_symbol_table
                .references
                .insert(symbol.to_string(), vec![]);

            current_file_symbol_table
                .references
                .get_mut(symbol)
                .unwrap()
        }
    };

    symbol_references.push(reference_location.clone());
}

pub fn analyze_global_stmnt(
    file_id: &FileId,
    ast: &Vec<Spanned<GlobalStatement>>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
) {
    let mut log_file = fs::File::options()
        .append(true)
        .create(true)
        .open("logs")
        .unwrap();

    for (global, span) in ast.iter() {
        writeln!(&mut log_file, "start analisis of statement {:?}", global).unwrap();

        match global {
            GlobalStatement::FunctionDefinition((is_pub, _), _, (name, name_span), args, _, _) => {
                symbol_table.symbols.insert(
                    name_span.clone(),
                    SymbolInfo {
                        name: name.clone(),
                        symbol_type: SymbolType::Function,
                        data_type: DataType::Null,
                        arguments: Some(
                            args.iter()
                                .filter_map(|(arg, _)| match arg {
                                    FunctionArgument::Generic((name, _)) => {
                                        Some((name.clone(), DataType::Any))
                                    }
                                    FunctionArgument::Typed((name, _), (ty, _)) => {
                                        Some((name.clone(), map_type(&ty)))
                                    }
                                    _ => None,
                                })
                                .collect(),
                        ),
                        is_public: *is_pub,
                        is_definition: true,
                        undefined: false,
                    },
                );

                insert_symbol_definition(
                    symbol_table,
                    name,
                    span.end..usize::MAX,
                    &SymbolLocation {
                        file: *file_id,
                        start: name_span.start,
                        end: name_span.end,
                        is_public: *is_pub,
                    },
                );
            }
            GlobalStatement::Import((is_pub, _), _, (import_content, _), _, (path, _)) => {
                let uri = &backend.paths.lookup(file_id);
                let import_path = map_import_path(uri, path);

                writeln!(&mut log_file, "import path: {:?}", import_path.to_file_path()).unwrap();

                let result = backend.open_document(&map_import_path(uri, path));

                writeln!(&mut log_file, "result of open_document: {:?}", result).unwrap();
                if result.is_err() {
                    continue;
                }

                let import_file_id = result.unwrap();

                let imported_file_symbol_table = match backend.symbol_table.get(&import_file_id) {
                    Some(symbol_table) => symbol_table,
                    None => continue,
                };

                // fs::write("logs", &format!("{:?}", imported_file_symbol_table.definitions)).unwrap();
                writeln!(
                    &mut log_file,
                    "{:?}",
                    imported_file_symbol_table.definitions
                )
                .unwrap();

                writeln!(&mut log_file, "{:?}", imported_file_symbol_table.symbols).unwrap();

                match import_content {
                    ImportContent::ImportSpecific(ident_list) => {
                        ident_list.iter().for_each(|(ident, span)| {
                            let symbol_definition =
                                match imported_file_symbol_table.definitions.get(ident) {
                                    Some(symbol_definitions) => symbol_definitions
                                        .clone()
                                        .into_iter()
                                        .find(|(_, location)| location.is_public),
                                    None => None,
                                };

                            match symbol_definition {
                                Some(symbol_definition) => {
                                    insert_symbol_definition(
                                        symbol_table,
                                        ident,
                                        span.start..usize::MAX,
                                        &SymbolLocation {
                                            file: symbol_definition.1.file.clone(),
                                            start: symbol_definition.1.start,
                                            end: symbol_definition.1.end,
                                            is_public: *is_pub,
                                        },
                                    );

                                    let symbol_info = imported_file_symbol_table
                                        .symbols
                                        .get(&symbol_definition.1.start)
                                        .unwrap();

                                    symbol_table.symbols.insert(
                                        span.clone(),
                                        SymbolInfo {
                                            name: ident.clone(),
                                            symbol_type: symbol_info.symbol_type.clone(),
                                            data_type: symbol_info.data_type.clone(),
                                            arguments: symbol_info.arguments.clone(),
                                            is_public: *is_pub,
                                            is_definition: false,
                                            undefined: false,
                                        },
                                    );
                                }
                                None => {
                                    symbol_table.symbols.insert(
                                        span.clone(),
                                        SymbolInfo {
                                            name: ident.clone(),
                                            symbol_type: SymbolType::Function,
                                            data_type: DataType::Null,
                                            arguments: None,
                                            is_public: *is_pub,
                                            is_definition: false,
                                            undefined: true,
                                        },
                                    );
                                }
                            };
                        });
                    }
                    ImportContent::ImportAll => imported_file_symbol_table
                        .definitions
                        .iter()
                        .for_each(|definition| {
                            definition.value().iter().for_each(|(_, location)| {
                                if !location.is_public {
                                    return;
                                }

                                let symbol_info =
                                    match imported_file_symbol_table.symbols.get(&location.start) {
                                        Some(symbol_info) => symbol_info,
                                        None => return,
                                    };

                                symbol_table.symbols.insert(
                                    span.end..usize::MAX,
                                    SymbolInfo {
                                        name: symbol_info.name.clone(),
                                        symbol_type: symbol_info.symbol_type.clone(),
                                        data_type: symbol_info.data_type.clone(),
                                        arguments: symbol_info.arguments.clone(),
                                        is_public: *is_pub,
                                        is_definition: false,
                                        undefined: false,
                                    },
                                );

                                insert_symbol_definition(
                                    symbol_table,
                                    &symbol_info.name,
                                    location.start..usize::MAX,
                                    location,
                                );
                            });
                        }),
                }
            }
            GlobalStatement::Main(body) => {
                body.iter().for_each(|stmnt| {
                    analyze_stmnt(&file_id, stmnt, symbol_table, backend, stmnt.1.end);
                });
            }
            GlobalStatement::Statement(stmnt) => {
                analyze_stmnt(&file_id, stmnt, symbol_table, backend, usize::MAX);
            }
        }
    }

    writeln!(&mut log_file, "stop analisis of {:?}", file_id).unwrap()
}

pub fn analyze_stmnt(
    file_id: &FileId,
    (stmnt, span): &Spanned<Statement>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
    scope_end: usize,
) {
    let mut log_file = fs::File::options()
        .append(true)
        .create(true)
        .open("logs")
        .unwrap();

    writeln!(&mut log_file, "start analisis of statement {:?}", stmnt).unwrap();

    match stmnt {
        Statement::Block(block) => analyze_block(file_id, block, symbol_table, backend),
        Statement::IfChain(if_chain) => {
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
                        ElseCondition::Else(block) => {
                            return analyze_block(file_id, block, symbol_table, backend)
                        }
                        ElseCondition::InlineElse(stmnt) => {
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
        Statement::IfCondition(if_cond, else_cond) => {
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
                    ElseCondition::Else(block) => {
                        return analyze_block(file_id, block, symbol_table, backend)
                    }
                    ElseCondition::InlineElse(stmnt) => {
                        return analyze_stmnt(file_id, stmnt, symbol_table, backend, stmnt.1.end)
                    }
                }
            }
        }
        Statement::InfiniteLoop(block) => analyze_block(file_id, block, symbol_table, backend),
        Statement::IterLoop((vars, _), exp, block) => {
            match &vars {
                IterLoopVars::WithIndex((var1, var1_span), (var2, var2_span)) => {
                    symbol_table.symbols.insert(
                        var1_span.clone(),
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
                        block.1.clone(),
                        &SymbolLocation {
                            file: *file_id,
                            start: var1_span.start,
                            end: var1_span.end,
                            is_public: false,
                        },
                    );

                    symbol_table.symbols.insert(
                        var2_span.clone(),
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
                        block.1.clone(),
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
                        var_span.clone(),
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
                        block.1.clone(),
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
        Statement::VariableInit((var_name, var_span), exp) => {
            symbol_table.symbols.insert(
                var_span.clone(),
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
                span.end..scope_end,
                &SymbolLocation {
                    file: *file_id,
                    start: var_span.start,
                    end: var_span.end,
                    is_public: false,
                },
            );

            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Statement::Echo(exp) => analyze_exp(file_id, exp, symbol_table, backend),
        Statement::Expression(exp) => analyze_exp(file_id, exp, symbol_table, backend),
        Statement::Fail(exp) => {
            if let Some(exp) = exp {
                analyze_exp(file_id, exp, symbol_table, backend);
            }
        }
        Statement::Return(exp) => {
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

pub fn analyze_exp(
    file_id: &FileId,
    (exp, _): &Spanned<Expression>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
) {
    let mut log_file = fs::File::options()
        .append(true)
        .create(true)
        .open("logs")
        .unwrap();

    writeln!(&mut log_file, "start analisis of expression {:?}", exp).unwrap();

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
        Expression::And(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Array(elements) => {
            elements.iter().for_each(|exp| {
                analyze_exp(file_id, exp, symbol_table, backend);
            });
        }
        Expression::Cast(expressions, _) => {
            expressions.get_iter().for_each(|exp| {
                analyze_exp(file_id, &exp, symbol_table, backend);
            });
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
        Expression::Is(exp, _) => {
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
        Expression::Nameof(exp) => {
            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Expression::Neg(exp) => {
            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Expression::Neq(exp1, exp2) => {
            analyze_exp(file_id, exp1, symbol_table, backend);
            analyze_exp(file_id, exp2, symbol_table, backend);
        }
        Expression::Not(exp) => {
            analyze_exp(file_id, exp, symbol_table, backend);
        }
        Expression::Or(exp1, exp2) => {
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
        Expression::Ternary(exp1, exp2, exp3) => {
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

pub fn analyze_failure_handler(
    file_id: &FileId,
    (failure, span): &Spanned<FailureHandler>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
) {
    match failure {
        FailureHandler::Handle(stmnts) => {
            stmnts.iter().for_each(|stmnt| {
                analyze_stmnt(file_id, stmnt, symbol_table, backend, span.end);
            });
        }
        _ => {}
    }
}
