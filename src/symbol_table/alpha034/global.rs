use crate::{
    backend::Backend,
    grammar::{
        alpha034::{FunctionArgument, GlobalStatement, ImportContent},
        Spanned,
    },
    paths::FileId,
    symbol_table::{
        insert_symbol_definition, map_type, DataType, SymbolInfo, SymbolLocation, SymbolTable,
        SymbolType,
    },
};

use super::{map_import_path, stmnts::analyze_stmnt};

pub fn analyze_global_stmnt(
    file_id: &FileId,
    ast: &Vec<Spanned<GlobalStatement>>,
    symbol_table: &mut SymbolTable,
    backend: &Backend,
) {
    for (global, span) in ast.iter() {
        match global {
            GlobalStatement::FunctionDefinition(
                (is_pub, _),
                _,
                (name, name_span),
                args,
                _,
                body,
            ) => {
                symbol_table.symbols.insert(
                    name_span.start..=name_span.end,
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
                    span.end..=usize::MAX,
                    &SymbolLocation {
                        file: *file_id,
                        start: name_span.start,
                        end: name_span.end,
                        is_public: *is_pub,
                    },
                );

                let args_end = args.last().map(|(_, span)| span.end).unwrap_or(name_span.end);

                args.iter().for_each(|(arg, _)| {
                    let (name, ty, name_span) = match arg {
                        FunctionArgument::Generic((name, span)) => (name, DataType::Any, span),
                        FunctionArgument::Typed((name, span), (ty, _)) => {
                            (name, map_type(&ty), span)
                        }
                        _ => return,
                    };

                    symbol_table.symbols.insert(
                        name_span.start..=name_span.end,
                        SymbolInfo {
                            name: name.clone(),
                            symbol_type: SymbolType::Variable,
                            data_type: ty,
                            arguments: None,
                            is_public: false,
                            is_definition: true,
                            undefined: false,
                        },
                    );
                    insert_symbol_definition(
                        symbol_table,
                        name,
                        args_end..=span.end,
                        &SymbolLocation {
                            file: *file_id,
                            start: name_span.start,
                            end: name_span.end,
                            is_public: false,
                        },
                    );
                });

                body.iter().for_each(|stmnt| {
                    analyze_stmnt(&file_id, stmnt, symbol_table, backend, span.end);
                });
            }
            GlobalStatement::Import((is_pub, _), _, (import_content, _), _, (path, _)) => {
                let uri = &backend.paths.lookup(file_id);

                let result = backend.open_document(&map_import_path(uri, path));

                if result.is_err() {
                    continue;
                }

                let import_file_id = result.unwrap();

                let imported_file_symbol_table = match backend.symbol_table.get(&import_file_id) {
                    Some(symbol_table) => symbol_table,
                    None => continue,
                };

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
                                        span.start..=usize::MAX,
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
                                        span.start..=span.end,
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
                                        span.start..=span.end,
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
                                    span.end..=usize::MAX,
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
                                    location.start..=usize::MAX,
                                    location,
                                );
                            });
                        }),
                }
            }
            GlobalStatement::Main(_, body) => {
                body.iter().for_each(|stmnt| {
                    analyze_stmnt(&file_id, stmnt, symbol_table, backend, stmnt.1.end);
                });
            }
            GlobalStatement::Statement(stmnt) => {
                analyze_stmnt(&file_id, stmnt, symbol_table, backend, usize::MAX);
            }
        }
    }
}
