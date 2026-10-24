use fairspec_metadata::{
    CellError, CellJsonError, CellMissingError, CellTypeError, Column, ColumnType, Result,
    inspect_json,
};
use polars::prelude::*;
use serde_json::Value;

use super::python::stringify_cell;
use crate::actions::column::substitute::substitute_column;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::general::get_is_object;
use crate::helpers::table::LazyFrameExt;
use crate::models::column::{ColumnMapping, PolarsColumn};
use crate::models::table::Table;
use crate::settings::NUMBER_COLUMN_NAME;

/// Checks each cell of a text column with `parse`, which returns false for an invalid cell.
pub fn inspect_text_column(
    column: &Column,
    table: Table,
    parse: &dyn Fn(&str) -> bool,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    let mut errors = Vec::new();
    let rows = collect_cell_rows(table, mapping, column)?;

    for row in rows {
        let Some(source) = row.source.clone() else {
            if !column.nullable().unwrap_or(false) {
                errors.push(create_missing_error(column.name(), &row));
            }
            continue;
        };

        if !parse(&source) {
            errors.push(create_type_error(column, &row, source));
        }
    }

    Ok(errors)
}

/// Checks each cell of a JSON column: it must parse, be an array/object, match the
/// type schema if given, and satisfy the column's own JSON Schema constraints.
pub fn inspect_json_column(
    column: &Column,
    table: Table,
    type_json_schema: Option<&Value>,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    let mut errors = Vec::new();

    let type_json_schema = type_json_schema
        .and_then(Value::as_object)
        .filter(|json_schema| !json_schema.is_empty());
    let constraint_json_schema = match serde_json::to_value(column.property()) {
        Ok(Value::Object(json_schema)) => json_schema,
        _ => Default::default(),
    };
    let is_array = column.column_type() == ColumnType::Array;
    let rows = collect_cell_rows(table, mapping, column)?;

    for row in rows {
        let Some(source) = row.source.clone() else {
            if !column.nullable().unwrap_or(false) {
                errors.push(create_missing_error(column.name(), &row));
            }
            continue;
        };

        let target = serde_json::from_str::<Value>(&source).unwrap_or(Value::Null);
        let is_compat = if is_array {
            target.is_array()
        } else {
            get_is_object(&target)
        };
        if !is_compat {
            errors.push(create_type_error(column, &row, source));
            continue;
        }

        if let Some(type_json_schema) = type_json_schema {
            if !inspect_json(&target, type_json_schema.clone(), None)?.is_empty() {
                errors.push(create_type_error(column, &row, source));
            }
            continue;
        }

        if !constraint_json_schema.is_empty() {
            for issue in inspect_json(&target, constraint_json_schema.clone(), None)? {
                errors.push(CellError::Json(CellJsonError {
                    resource_name: None,
                    column_name: column.name().to_string(),
                    row_number: row.number,
                    cell: source.clone(),
                    message: issue.message,
                    json_pointer: issue.json_pointer,
                }));
            }
        }
    }

    Ok(errors)
}

struct CellRow {
    number: usize,
    source: Option<String>,
    cell: String,
}

fn collect_cell_rows(
    table: Table,
    mapping: Option<&ColumnMapping>,
    column: &Column,
) -> Result<Vec<CellRow>> {
    let text_mapping;
    let mapping = match mapping {
        Some(mapping) => mapping,
        None => {
            text_mapping = create_text_mapping(column);
            &text_mapping
        }
    };
    let source_name = mapping.source.name.as_str();

    let frame = table
        .with_row_index(NUMBER_COLUMN_NAME, Some(1))
        .select([
            col(NUMBER_COLUMN_NAME).cast(DataType::UInt64),
            substitute_column(mapping, col(source_name)).alias("source"),
            col(source_name).alias("cell"),
        ])
        .collect_table()?;

    let numbers = frame.column(NUMBER_COLUMN_NAME).into_result()?;
    let numbers = numbers.u64().into_result()?;
    let sources = frame.column("source").into_result()?;
    let cells = frame.column("cell").into_result()?;

    let mut rows = Vec::with_capacity(frame.height());
    for index in 0..frame.height() {
        let source = sources.get(index).into_result()?;
        rows.push(CellRow {
            number: numbers.get(index).unwrap_or_default() as usize,
            source: (!source.is_null()).then(|| stringify_cell(&source)),
            cell: stringify_cell(&cells.get(index).into_result()?),
        });
    }
    Ok(rows)
}

fn create_text_mapping(column: &Column) -> ColumnMapping {
    ColumnMapping {
        source: PolarsColumn {
            name: column.name().to_string(),
            r#type: DataType::String,
            time_zone: None,
        },
        target: column.clone(),
    }
}

fn create_type_error(column: &Column, row: &CellRow, source: String) -> CellError {
    CellError::Type(CellTypeError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: row.number,
        cell: source,
        column_type: column.column_type(),
    })
}

fn create_missing_error(column_name: &str, row: &CellRow) -> CellError {
    CellError::Missing(CellMissingError {
        resource_name: None,
        column_name: column_name.to_string(),
        row_number: row.number,
        cell: row.cell.clone(),
    })
}
