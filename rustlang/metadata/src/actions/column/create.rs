use serde_json::Value;

use super::property::get_is_nullable_property_type;
use crate::models::column::column::{Column, ColumnProperty};
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};

pub fn create_column_from_property(name: &str, property: &Descriptor) -> Result<Column> {
    let nullable = get_is_nullable_property_type(property.get("type"));
    let property: ColumnProperty = serde_json::from_value(Value::Object(property.clone()))
        .map_err(|error| Error::Parse(format!("column \"{name}\": {error}")))?;
    let mut column = Column::from_property(name, property);
    column.set_nullable(nullable.then_some(true));
    Ok(column)
}
