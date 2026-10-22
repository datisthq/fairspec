use polars::prelude::{DataType, Schema};

use crate::models::column::PolarsColumn;
use crate::models::schema::PolarsSchema;

pub fn get_polars_schema(schema: &Schema) -> PolarsSchema {
    let columns = schema
        .iter()
        .map(|(name, dtype)| PolarsColumn {
            name: name.to_string(),
            r#type: dtype.clone(),
            time_zone: match dtype {
                DataType::Datetime(_, Some(time_zone)) => Some(time_zone.to_string()),
                _ => None,
            },
        })
        .collect();
    PolarsSchema { columns }
}
