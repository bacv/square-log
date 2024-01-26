use std::{path::PathBuf, result::Result};

use serde::{Deserialize, Serialize};

use crate::{plugin::source::SourceSummary, record::DataRecord};

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
    fn insert(&self, source: &str, record: DataRecord) -> Result<(), String>;
    fn get_range(&self, source: &str, range: Range) -> Result<Vec<DataRecord>, String>;
    fn get_source(&self, source: &str) -> Result<Option<SourceSummary>, String>;
    fn get_sources(&self) -> Result<Vec<SourceSummary>, String>;
}
