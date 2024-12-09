use dashmap::DashMap;
use paths::FileId;
use rangemap::RangeMap; // TODO: RangeInclusiveMap

pub mod backend;
pub mod grammar;
pub mod paths;
pub mod symbol_table;
pub mod fs;
