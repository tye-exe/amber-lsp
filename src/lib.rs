use dashmap::DashMap;
use paths::FileId;
use rangemap::RangeMap; // TODO: RangeInclusiveMap

pub mod backend;
pub mod grammar;
pub mod paths;

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
    pub symbols: RangeMap<usize, SymbolInfo>,
    // TODO: Scoped definitions
    pub definitions: DashMap<String, RangeMap<usize, SymbolLocation>>,
    pub references: DashMap<String, Vec<SymbolLocation>>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            symbols: RangeMap::new(),
            definitions: DashMap::new(),
            references: DashMap::new(),
        }
    }
}
