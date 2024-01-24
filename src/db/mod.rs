use std::result::Result;

use crate::record::DataRecord;

pub mod mock;

pub struct Range {
    pub from: Option<usize>,
    pub to: Option<usize>,
}

pub trait Database {
    fn insert(&self, source: String, key: String, record: DataRecord) -> Result<(), String>;
    fn get(&self, source: String, key: String) -> Result<Option<DataRecord>, String>;
    fn get_latest(&self, source: String) -> Result<Option<DataRecord>, String>;
    fn get_range(&self, source: String, range: Range) -> Result<Vec<DataRecord>, String>;
}
