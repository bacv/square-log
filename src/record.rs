use chrono::NaiveDate;
use mlua::UserData;

#[derive(Debug, Clone)]
pub struct DataRecord {
    pub date: NaiveDate,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: String,
    pub extended: String,
    pub hash: String,
}

impl UserData for DataRecord {}
