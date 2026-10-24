use crate::actions::descriptor::save::save_profiled_model;
use crate::models::exception::Result;
use crate::models::profile::ProfileType;
use crate::models::table_schema::TableSchema;

pub fn save_table_schema(table_schema: &TableSchema, path: &str, overwrite: bool) -> Result<()> {
    save_profiled_model(table_schema, ProfileType::TableSchema, path, overwrite)
}
