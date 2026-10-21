use super::assert::assert_table_schema;
use crate::actions::descriptor::load::load_descriptor;
use crate::models::exception::Result;
use crate::models::table_schema::TableSchema;

pub fn load_table_schema(path: &str) -> Result<TableSchema> {
    let descriptor = load_descriptor(path, false)?;
    assert_table_schema(&descriptor)
}
