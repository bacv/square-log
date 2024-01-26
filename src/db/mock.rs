use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::db::{Database, DbConfig, Range};
use crate::plugin::source::SourceSummary;
use crate::record::DataRecord;

pub struct MockDatabase {
    records: Arc<RwLock<HashMap<String, Vec<DataRecord>>>>,
    indices: Arc<RwLock<HashMap<String, usize>>>,
}

impl MockDatabase {
    pub fn new(_config: DbConfig) -> Self {
        MockDatabase {
            records: Arc::new(RwLock::new(HashMap::new())),
            indices: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Database for MockDatabase {
    fn insert(&self, source: &str, record: DataRecord) -> Result<(), String> {
        let mut records = self.records.write().unwrap();
        let mut indices = self.indices.write().unwrap();
        let mut record = record;

        let idx = indices.entry(source.to_string()).or_insert(0);
        record.id = Some(*idx);
        records.entry(source.to_string()).or_default().push(record);

        *idx += 1;
        Ok(())
    }

    fn get_range(&self, source: &str, range: Range) -> Result<Vec<DataRecord>, String> {
        let records = self.records.read().unwrap();
        let start = range.from.unwrap_or(0);
        let end = range.to.unwrap_or(start + 10);

        Ok(records
            .get(source)
            .map_or(Vec::new(), |v| v[start..end.min(v.len())].to_vec()))
    }

    fn get_sources(&self) -> Result<Vec<SourceSummary>, String> {
        let records = self.records.read().unwrap();
        Ok(records
            .keys()
            .cloned()
            .map(|id| {
                let latest = records.get(&id).and_then(|v| v.last().cloned());
                SourceSummary { id, latest }
            })
            .collect())
    }

    fn get_source(&self, source: &str) -> Result<Option<SourceSummary>, String> {
        let records = self.records.read().unwrap();
        let latest = records.get(source).and_then(|v| v.last().cloned());
        Ok(Some(SourceSummary {
            id: source.to_string(),
            latest,
        }))
    }
}
