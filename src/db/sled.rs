use crate::db::{Database, DbConfig, Range};
use crate::plugin::source::SourceSummary;
use crate::record::DataRecord;
use sled::Db;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct SledDatabase {
    db: Arc<Db>,
    indices: RwLock<HashMap<String, usize>>,
}

impl SledDatabase {
    pub fn new(config: DbConfig) -> Result<Self, sled::Error> {
        let db = sled::open(config.path)?;
        let indices = RwLock::new(HashMap::new());

        // Initialize indices for each source
        for name in db.tree_names() {
            let source = String::from_utf8(name.to_vec()).unwrap();
            let tree = db.open_tree(&source)?;
            let last_key = tree.last().map(|res| res.map(|(k, _)| k))?;
            let last_index = last_key
                .map(|k| bincode::deserialize::<usize>(&k).unwrap())
                .unwrap_or(0);
            indices.write().unwrap().insert(source, last_index);
        }

        Ok(SledDatabase {
            db: Arc::new(db),
            indices,
        })
    }

    fn get_next_index(&self, source: &str) -> usize {
        let mut indices = self.indices.write().unwrap();
        let index = indices.entry(source.to_string()).or_insert(0);
        *index += 1;
        *index
    }
}

impl Database for SledDatabase {
    fn insert(&self, source: &str, record: DataRecord) -> Result<(), String> {
        let index = self.get_next_index(source);
        let mut record = record;
        record.id = Some(index);

        let serialized = bincode::serialize(&record).map_err(|e| e.to_string())?;
        let tree = self.db.open_tree(source).map_err(|e| e.to_string())?;
        tree.insert(bincode::serialize(&index).unwrap(), serialized)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn get_range(&self, source: &str, range: Range) -> Result<Vec<DataRecord>, String> {
        let tree = self.db.open_tree(source).map_err(|e| e.to_string())?;
        let oldest_idx = range.from.unwrap_or(0);
        let latest_idx = range.to.unwrap_or_else(|| {
            self.indices
                .read()
                .unwrap()
                .get(source)
                .cloned()
                .unwrap_or(0)
        });

        let mut records = Vec::new();
        for i in oldest_idx..=latest_idx {
            if let Ok(Some(value)) = tree.get(bincode::serialize(&i).unwrap()) {
                let record: DataRecord = bincode::deserialize(&value).map_err(|e| e.to_string())?;
                records.push(record);
            }
        }
        Ok(records)
    }

    fn get_source(&self, source: &str) -> Result<Option<SourceSummary>, String> {
        let tree = self.db.open_tree(source).map_err(|e| e.to_string())?;
        if let Ok(Some((_, value))) = tree.last() {
            let record: DataRecord = bincode::deserialize(&value).map_err(|e| e.to_string())?;
            Ok(Some(SourceSummary {
                id: source.to_string(),
                latest: Some(record),
            }))
        } else {
            Ok(None)
        }
    }

    fn get_sources(&self) -> Result<Vec<SourceSummary>, String> {
        let mut summaries = Vec::new();
        for name in self.db.tree_names() {
            let source = String::from_utf8(name.to_vec()).unwrap();
            if let Some(summary) = self.get_source(&source)? {
                summaries.push(summary);
            }
        }
        Ok(summaries)
    }
}
