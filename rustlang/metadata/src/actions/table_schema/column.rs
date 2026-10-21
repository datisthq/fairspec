use serde_json::Value;

use crate::actions::column::create::create_column_from_property;
use crate::models::column::column::Column;
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};

pub fn get_columns(table_schema: &Descriptor) -> Result<Vec<Column>> {
    let Some(properties) = table_schema
        .get("properties")
        .filter(|value| !value.is_null())
    else {
        return Ok(Vec::new());
    };
    let Value::Object(properties) = properties else {
        return Err(Error::Parse(
            "table schema properties must be an object".to_string(),
        ));
    };
    let all_required = table_schema.get("allRequired") == Some(&Value::Bool(true));
    let required = table_schema.get("required").and_then(Value::as_array);

    let mut columns = Vec::new();
    for (name, property) in properties {
        let Value::Object(property) = property else {
            return Err(Error::Parse(format!("column \"{name}\" must be an object")));
        };
        let mut column = create_column_from_property(name, property)?;
        let is_required = all_required
            || required.is_some_and(|required| required.iter().any(|item| item == name));
        column.set_required(is_required.then_some(true));
        columns.push(column);
    }
    Ok(columns)
}
