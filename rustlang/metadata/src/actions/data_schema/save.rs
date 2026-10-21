use crate::actions::descriptor::save::save_profiled_model;
use crate::models::data_schema::DataSchema;
use crate::models::exception::Result;
use crate::models::profile::ProfileType;

pub fn save_data_schema(data_schema: &DataSchema, path: &str, overwrite: bool) -> Result<()> {
    save_profiled_model(data_schema, ProfileType::DataSchema, path, overwrite)
}
