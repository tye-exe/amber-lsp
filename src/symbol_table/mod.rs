use dashmap::DashMap;
use rangemap::RangeInclusiveMap;
use std::{ops::{Range, RangeInclusive}, path::PathBuf};

use crate::{backend::Backend, paths::FileId};

pub mod alpha034;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    Any,
    Number,
    Boolean,
    Text,
    Null,
    Array(Box<DataType>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SymbolType {
    Function,
    Variable,
}

/// Information about a symbol in the document.
///
/// The `name` field contains the name of the symbol.
///
/// The `symbol_type` field contains the type of the symbol.
///
/// The `data_type` field contains the data type of the symbol.
///
/// The `arguments` field contains the arguments of the symbol if it is a function.
///
/// The `is_public` field is true if the symbol is public.
///
/// The `is_definition` field is true if the symbol is a definition.
///
/// The `undefined` field is true if the symbol is undefined.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub symbol_type: SymbolType,
    pub data_type: DataType,
    pub arguments: Option<Vec<(String, DataType)>>,
    pub is_public: bool,
    pub is_definition: bool,
    pub undefined: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolLocation {
    pub file: FileId,
    pub start: usize,
    pub end: usize,
    pub is_public: bool,
}

/// A symbol table that contains all the symbols in a document.
/// The symbols are stored in a RangeMap data structure for fast
/// range queries.
///
/// The `definitions` map contains the definition of each symbol. RangeMap is used to store the scope of each symbol definition.
///
/// The `references` map contains the references to each symbol.
///
/// The `symbols` Lapper contains the information about symbols in the document.
pub struct SymbolTable {
    pub symbols: RangeInclusiveMap<usize, SymbolInfo>,
    // TODO: Scoped definitions
    pub definitions: DashMap<String, RangeInclusiveMap<usize, SymbolLocation>>,
    pub references: DashMap<String, Vec<SymbolLocation>>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            symbols: RangeInclusiveMap::new(),
            definitions: DashMap::new(),
            references: DashMap::new(),
        }
    }
}

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

#[inline]
#[cfg(target_os = "linux")]
pub fn get_install_dir() -> PathBuf {
    PathBuf::from("/etc/amber_lsp")
}

#[inline]
#[cfg(target_os = "windows")]
pub fn get_install_dir() -> PathBuf {
    PathBuf::from("C:\\Program Files\\amber_lsp")
}

#[inline]
#[cfg(target_os = "macos")]
pub fn get_install_dir() -> PathBuf {
    PathBuf::from("/usr/local/etc/amber_lsp")
}

pub fn insert_symbol_definition(
    symbol_table: &mut SymbolTable,
    symbol: &str,
    definition_scope: RangeInclusive<usize>,
    definition_location: &SymbolLocation,
) {
    let mut symbol_definitions = match symbol_table.definitions.get_mut(symbol) {
        Some(symbol_definitions) => symbol_definitions,
        None => {
            symbol_table
                .definitions
                .insert(symbol.to_string(), RangeInclusiveMap::new());

            symbol_table.definitions.get_mut(symbol).unwrap()
        }
    };

    symbol_definitions.insert(
        definition_scope,
        definition_location.clone(),
    );
}

pub fn insert_symbol_reference(
    symbol: &str,
    current_file_symbol_table: &mut SymbolTable,
    backend: &Backend,
    reference_location: &SymbolLocation,
) {
    let span = reference_location.start..=reference_location.end;

    let symbol_definition = match current_file_symbol_table.definitions.get(symbol) {
        Some(symbol_definitions) => symbol_definitions.get(&span.start()).cloned(),
        None => None,
    };

    match symbol_definition {
        Some(definition) => {
            let symbol_info = if definition.file == reference_location.file {
                current_file_symbol_table
                    .symbols
                    .get(&definition.start)
                    .cloned()
                    .unwrap()
            } else {
                let definition_file_symbol_table =
                    backend.symbol_table.get_mut(&definition.file).unwrap();

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
