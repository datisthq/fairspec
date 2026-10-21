use super::assert::assert_data_schema;
use crate::actions::descriptor::load::load_descriptor;
use crate::models::data_schema::DataSchema;
use crate::models::exception::Result;

pub fn load_data_schema(path: &str) -> Result<DataSchema> {
    let descriptor = load_descriptor(path, false)?;
    assert_data_schema(&descriptor)
}
