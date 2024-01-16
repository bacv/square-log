use mlua::{Lua, UserData};

#[derive(Debug, Clone)]
struct DataRecord {
    data: String,
    time: String,
}

impl UserData for DataRecord {}

pub fn process_data_records(_: &Lua, records: Vec<DataRecord>) -> mlua::Result<()> {
    for record in records {
        println!("Data: {}, Time: {}", record.data, record.time);
    }
    Ok(())
}
