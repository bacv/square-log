use std::{path::PathBuf, result::Result};

use serde::{Deserialize, Serialize};

use crate::record::DataRecord;

pub mod mock;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DbConfig {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct Range {
    pub from: Option<usize>,
    pub to: Option<usize>,
}

pub trait Database {
    fn insert(&self, source: &str, key: String, record: DataRecord) -> Result<(), String>;
    fn get(&self, source: &str, key: String) -> Result<Option<DataRecord>, String>;
    fn get_latest(&self, source: &str) -> Result<Option<DataRecord>, String>;
    fn get_range(&self, source: &str, range: Range) -> Result<Vec<DataRecord>, String>;
    fn get_sources(&self) -> Result<Vec<String>, String>;
}
