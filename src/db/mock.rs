use std::collections::HashMap;
use std::result::Result;
use std::sync::{Arc, Mutex};

use crate::record::DataRecord;

use super::{Database, Range};

type Records = Arc<Mutex<HashMap<String, DataRecord>>>;

#[derive(Default)]
pub struct MockDatabase {
    records: Records,
}

impl MockDatabase {
    pub fn new() -> Self {
        Default::default()
    }

    fn make_key(source: &str, key: &str) -> String {
        format!("{}:{}", source, key)
    }
}

impl Database for MockDatabase {
    fn insert(&self, source: String, key: String, record: DataRecord) -> Result<(), String> {
        let composite_key = Self::make_key(&source, &key);
        let mut records = self.records.lock().expect("Should lock records");
        records.insert(composite_key, record);
        Ok(())
    }

    fn get(&self, source: String, key: String) -> Result<Option<DataRecord>, String> {
        let composite_key = Self::make_key(&source, &key);
        Ok(self.records.lock().expect("").get(&composite_key).cloned())
    }

    fn get_latest(&self, source: String) -> Result<Option<DataRecord>, String> {
        let res = self.records.lock().expect("").iter().find_map(|(k, v)| {
            if k.starts_with(&source) {
                Some(v.clone())
            } else {
                None
            }
        });

        Ok(res)
    }

    fn get_range(&self, source: String, range: Range) -> Result<Vec<DataRecord>, String> {
        let records = self.records.lock().expect("");
        let records: Vec<DataRecord> = records
            .iter()
            .filter_map(|(k, v)| {
                if k.starts_with(&source) {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .skip(range.from.unwrap_or(0))
            .take(range.to.unwrap_or_else(|| records.len()) - range.from.unwrap_or(0))
            .collect();

        Ok(records)
    }
}
