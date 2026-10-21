use crate::actions::descriptor::save::save_descriptor;
use crate::models::exception::Result;
use crate::models::json_schema::JsonSchema;

pub fn save_json_schema(json_schema: &JsonSchema, path: &str, overwrite: bool) -> Result<()> {
    save_descriptor(json_schema, path, overwrite)
}
