use std::{fs::File, path::PathBuf};

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SourcesConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub address: String,
    pub plugin: String,
}

pub struct Sources {}

impl Sources {
    // Function to read sources from a CSV file.
    pub fn load(config: SourcesConfig) -> color_eyre::Result<Vec<Source>> {
        let file = File::open(config.path)?;
        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

        let mut sources = Vec::new();
        for result in rdr.deserialize() {
            let source: Source = result?;
            sources.push(source);
        }

        Ok(sources)
    }
}

