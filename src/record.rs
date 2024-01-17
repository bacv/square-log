use mlua::UserData;

#[derive(Debug, Clone)]
pub struct DataRecord {
    pub data: String,
    pub time: String,
}

impl UserData for DataRecord {}
