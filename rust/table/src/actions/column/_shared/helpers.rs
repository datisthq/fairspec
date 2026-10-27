use fairspec_metadata::{
    CellError, CellJsonError, CellMissingError, CellTypeError, Column, ColumnType, Result,
    inspect_json,
};
use polars::prelude::*;
use serde_json::Value;

use super::python::stringify_cell;
use crate::actions::column::substitute::substitute_column;
use crate::helpers::error::PolarsResultExt;
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
    let rows = collect_cell_rows(table, mapping, column, None)?;

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
    // The native kind check already holds what `type` would assert on every value
    let mut constraint_json_schema = match serde_json::to_value(column.property()) {
        Ok(Value::Object(json_schema)) => json_schema,
        _ => Default::default(),
    };
    constraint_json_schema.remove("type");
    let needs_validator = type_json_schema.is_some()
        || constraint_json_schema
            .keys()
            .any(|key| !ANNOTATION_KEYWORDS.contains(&key.as_str()));
    let rows = collect_cell_rows(
        table,
        mapping,
        column,
        Some(JsonCells {
            errors_only: !needs_validator,
        }),
    )?;

    for row in rows {
        let Some(source) = row.source.clone() else {
            if !column.nullable().unwrap_or(false) {
                errors.push(create_missing_error(column.name(), &row));
            }
            continue;
        };

        if !row.valid {
            errors.push(create_type_error(column, &row, source));
            continue;
        }

        let target = serde_json::from_str::<Value>(&source).unwrap_or(Value::Null);

        if let Some(type_json_schema) = type_json_schema {
            if !inspect_json(&target, type_json_schema.clone(), None)?.is_empty() {
                errors.push(create_type_error(column, &row, source));
            }
            continue;
        }

        if needs_validator {
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

/// Keywords that never fail a value: a column with only these needs no validator.
const ANNOTATION_KEYWORDS: &[&str] = &[
    "type",
    "title",
    "description",
    "examples",
    "default",
    "$comment",
    "deprecated",
    "readOnly",
    "writeOnly",
    "rdfType",
    "missingValues",
];

struct CellRow {
    number: usize,
    source: Option<String>,
    cell: String,
    valid: bool,
}

/// Rows of a JSON column carry Polars' verdict on each cell; `errors_only` keeps just the failing ones.
struct JsonCells {
    errors_only: bool,
}

fn collect_cell_rows(
    table: Table,
    mapping: Option<&ColumnMapping>,
    column: &Column,
    json_cells: Option<JsonCells>,
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

    let mut cell_frame = table.with_row_index(NUMBER_COLUMN_NAME, Some(1)).select([
        col(NUMBER_COLUMN_NAME).cast(DataType::UInt64),
        encode_nested_column(mapping, substitute_column(mapping, col(source_name))).alias("source"),
        encode_nested_column(mapping, col(source_name)).alias("cell"),
    ]);
    if let Some(json_cells) = &json_cells {
        cell_frame =
            cell_frame.with_column(create_json_kind_expr(mapping, col("source")).alias("valid"));
        if json_cells.errors_only {
            let is_required = !column.nullable().unwrap_or(false);
            cell_frame = cell_frame.filter(
                col("source")
                    .is_null()
                    .and(lit(is_required))
                    .or(col("source").is_not_null().and(col("valid").not())),
            );
        }
    }
    let frame = cell_frame.collect_table()?;

    let numbers = frame.column(NUMBER_COLUMN_NAME).into_result()?;
    let numbers = numbers.u64().into_result()?;
    let sources = frame.column("source").into_result()?;
    let cells = frame.column("cell").into_result()?;

    let valids = match json_cells {
        Some(_) => Some(
            frame
                .column("valid")
                .into_result()?
                .bool()
                .into_result()?
                .clone(),
        ),
        None => None,
    };

    let mut rows = Vec::with_capacity(frame.height());
    for index in 0..frame.height() {
        let source = sources.get(index).into_result()?;
        rows.push(CellRow {
            number: numbers.get(index).unwrap_or_default() as usize,
            source: (!source.is_null()).then(|| stringify_cell(&source)),
            cell: stringify_cell(&cells.get(index).into_result()?),
            valid: valids
                .as_ref()
                .is_none_or(|valids| valids.get(index).unwrap_or(false)),
        });
    }
    Ok(rows)
}

/// Polars' parser is the one judge of a JSON cell, so every library rejects the same text.
fn create_json_kind_expr(mapping: &ColumnMapping, source_expr: Expr) -> Expr {
    if matches!(
        mapping.source.r#type,
        DataType::Struct(_) | DataType::List(_)
    ) {
        return lit(true);
    }
    let opener = match mapping.target.column_type() {
        ColumnType::Array => r"^[ \t\n\r]*\[",
        _ => r"^[ \t\n\r]*\{",
    };
    source_expr
        .clone()
        .str()
        .contains(lit(opener), false)
        .and(source_expr.str().json_path_match(lit("$")).is_not_null())
}

/// Wrapping in a one-field struct lets List encode too, which has no `json_encode`.
fn encode_nested_column(mapping: &ColumnMapping, column_expr: Expr) -> Expr {
    if !matches!(
        mapping.source.r#type,
        DataType::Struct(_) | DataType::List(_)
    ) {
        return column_expr;
    }
    let encoded = as_struct(vec![column_expr.clone().alias("v")])
        .struct_()
        .json_encode();
    when(column_expr.is_not_null())
        .then(
            encoded
                .str()
                .replace(lit(r#"(?s)^\{"v":(.*)\}$"#), lit("${1}"), false),
        )
        .otherwise(lit(NULL))
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
