use std::collections::HashMap;
use std::sync::Arc;

use fairspec_metadata::{
    CellError, Column, ColumnError, ColumnType, ColumnTypeError, Error, Result, TableError,
};
use indexmap::IndexMap;
use polars::prelude::*;
use serde_json::Value;

use super::_shared::python::stringify_cell;
use super::checks::check::ColumnCheck;
use super::checks::r#const::check_cell_const;
use super::checks::r#enum::check_cell_enum;
use super::checks::max_items::check_cell_max_items;
use super::checks::max_length::check_cell_max_length;
use super::checks::maximum::check_cell_maximum;
use super::checks::min_items::check_cell_min_items;
use super::checks::min_length::check_cell_min_length;
use super::checks::minimum::check_cell_minimum;
use super::checks::missing::check_cell_missing;
use super::checks::multiple_of::check_cell_multiple_of;
use super::checks::pattern::check_cell_pattern;
use super::checks::r#type::check_cell_type;
use super::normalize::normalize_column;
use super::types::array::inspect_array_column;
use super::types::duration::inspect_duration_column;
use super::types::geojson::inspect_geojson_column;
use super::types::object::inspect_object_column;
use super::types::topojson::inspect_topojson_column;
use super::types::wkb::inspect_wkb_column;
use super::types::wkt::inspect_wkt_column;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::table::LazyFrameExt;
use crate::models::cell::CellMapping;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;
use crate::settings::NUMBER_COLUMN_NAME;

/// Builds an always-false Boolean from a reader of the typed value of a column the read
/// parses (None for any other).
pub type FlagFactory = Arc<dyn Fn(&dyn Fn(&str) -> Option<Expr>) -> Expr + Send + Sync>;

/// Checks every column against its schema, keyed by column name in mapping order.
/// Each of `flags` is evaluated in the same read.
pub fn inspect_columns(
    mappings: &[ColumnMapping],
    table: Table,
    max_errors: usize,
    flags: &[FlagFactory],
) -> Result<IndexMap<String, Vec<TableError>>> {
    let mut errors: IndexMap<String, Vec<TableError>> = IndexMap::new();
    let mut polars_mappings = Vec::new();

    for mapping in mappings {
        let type_errors = inspect_type(mapping);
        let is_checked = type_errors.is_empty();
        errors.insert(mapping.target.name().to_string(), type_errors);
        if is_checked && !get_is_row_inspected(&mapping.target) {
            polars_mappings.push(mapping.clone());
        }
    }

    if !polars_mappings.is_empty() || !flags.is_empty() {
        let cell_errors =
            inspect_cells_in_polars(&polars_mappings, table.clone(), max_errors, flags)?;
        errors.extend(cell_errors);
    }

    let mut error_count = 0;
    for mapping in mappings {
        let Some(column_errors) = errors.get_mut(mapping.target.name()) else {
            continue;
        };
        let is_checked = column_errors.is_empty() && error_count < max_errors;
        if is_checked && get_is_row_inspected(&mapping.target) {
            let cell_errors = inspect_cells_by_row(mapping, table.clone())?;
            column_errors.extend(cell_errors.into_iter().map(TableError::Cell));
        }
        error_count += column_errors.len();
    }

    Ok(errors)
}

fn inspect_type(mapping: &ColumnMapping) -> Vec<TableError> {
    let compat_types = get_compat_types(mapping);
    let target_type = mapping.target.column_type();

    let is_compat = compat_types
        .iter()
        .any(|compat_type| *compat_type == target_type || *compat_type == ColumnType::Unknown);
    if is_compat {
        return Vec::new();
    }

    vec![TableError::Column(ColumnError::Type(ColumnTypeError {
        resource_name: None,
        column_name: mapping.target.name().to_string(),
        expected_column_type: target_type,
        actual_column_type: compat_types.first().copied().unwrap_or(ColumnType::Unknown),
    }))]
}

fn get_compat_types(mapping: &ColumnMapping) -> Vec<ColumnType> {
    use ColumnType as T;
    match &mapping.source.r#type {
        DataType::Datetime(_, _) => {
            let is_zoned = mapping
                .source
                .time_zone
                .as_ref()
                .is_some_and(|time_zone| !time_zone.is_empty());
            vec![if is_zoned {
                T::DateTime
            } else {
                T::LocalDateTime
            }]
        }
        DataType::Boolean => vec![T::Boolean],
        DataType::Categorical(_, _) => vec![T::String],
        DataType::Date => vec![T::Date],
        DataType::Float32 | DataType::Float64 => vec![T::Number, T::Integer],
        DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64
        | DataType::UInt8
        | DataType::UInt16
        | DataType::UInt32
        | DataType::UInt64 => vec![T::Integer, T::Number],
        DataType::List(_) => vec![T::List, T::Array],
        DataType::String => vec![T::Unknown],
        DataType::Struct(_) => vec![T::Object, T::Geojson, T::Topojson],
        DataType::Time => vec![T::Time, T::LocalTime],
        _ => Vec::new(),
    }
}

// These types are parsed row by row outside polars, so each keeps its own scan
fn get_is_row_inspected(column: &Column) -> bool {
    matches!(
        column,
        Column::Duration(_)
            | Column::Wkb(_)
            | Column::Wkt(_)
            | Column::Array(_)
            | Column::Object(_)
            | Column::Geojson(_)
            | Column::Topojson(_)
    )
}

fn inspect_cells_by_row(mapping: &ColumnMapping, table: Table) -> Result<Vec<CellError>> {
    let mapping_ref = Some(mapping);
    match &mapping.target {
        Column::Duration(column) => inspect_duration_column(column, table, mapping_ref),
        Column::Wkb(column) => inspect_wkb_column(column, table, mapping_ref),
        Column::Wkt(column) => inspect_wkt_column(column, table, mapping_ref),
        Column::Array(column) => inspect_array_column(column, table, mapping_ref),
        Column::Object(column) => inspect_object_column(column, table, mapping_ref),
        Column::Geojson(column) => inspect_geojson_column(column, table, mapping_ref),
        Column::Topojson(column) => inspect_topojson_column(column, table, mapping_ref),
        _ => Ok(Vec::new()),
    }
}

/// Each check with whether it short-circuits: a cell failing `type` or `missing` has
/// that error alone, while every failing constraint is reported, in keyword order.
fn check_cells(column: &Column, mapping: &CellMapping) -> Vec<(bool, ColumnCheck)> {
    let short_circuits = [
        check_cell_type(column, mapping),
        check_cell_missing(column, mapping),
    ];
    let constraints = [
        check_cell_enum(column, mapping),
        check_cell_const(column, mapping),
        check_cell_minimum(column, mapping, false),
        check_cell_maximum(column, mapping, false),
        check_cell_minimum(column, mapping, true),
        check_cell_maximum(column, mapping, true),
        check_cell_multiple_of(column, mapping),
        check_cell_min_length(column, mapping),
        check_cell_max_length(column, mapping),
        check_cell_pattern(column, mapping),
        check_cell_min_items(column, mapping),
        check_cell_max_items(column, mapping),
    ];
    let short_circuits = short_circuits
        .into_iter()
        .flatten()
        .map(|check| (true, check));
    let constraints = constraints
        .into_iter()
        .flatten()
        .map(|check| (false, check));
    short_circuits.chain(constraints).collect()
}

struct CheckedColumn<'a> {
    index: usize,
    mapping: &'a ColumnMapping,
    templates: Vec<CellError>,
    error_expr: Expr,
}

fn inspect_cells_in_polars(
    mappings: &[ColumnMapping],
    table: Table,
    max_errors: usize,
    flags: &[FlagFactory],
) -> Result<IndexMap<String, Vec<TableError>>> {
    let mut errors: IndexMap<String, Vec<TableError>> = mappings
        .iter()
        .map(|mapping| (mapping.target.name().to_string(), Vec::new()))
        .collect();
    let is_flagged: Vec<Expr> = (0..flags.len())
        .map(|index| col(get_flag_name(index)))
        .collect();

    let candidates: Vec<_> = mappings
        .iter()
        .enumerate()
        .filter(|(_, mapping)| mapping.target.column_type() != ColumnType::Unknown)
        .map(|(index, mapping)| {
            let cell_mapping = CellMapping {
                source: col(get_source_name(index)),
                target: col(get_target_name(index)),
            };
            let checks = check_cells(&mapping.target, &cell_mapping);
            let is_parsed = normalize_column(mapping, false) != normalize_column(mapping, true);
            (index, mapping, checks, is_parsed)
        })
        .collect();

    // A target read as its source is null only where the source is, so it never fails the
    // type check; a column left with no check is still read (carried), as the read
    // reports a long row or invalid UTF-8 only in the columns it reads
    let has_check = candidates
        .iter()
        .any(|(_, _, checks, is_parsed)| *is_parsed || checks.len() > 1);
    let mut checked = Vec::new();
    let mut carried = Vec::new();
    for (index, mapping, checks, is_parsed) in candidates {
        let checks: Vec<(bool, ColumnCheck)> = checks
            .into_iter()
            .filter(|(_, check)| {
                is_parsed || !has_check || !matches!(check.error_template, CellError::Type(_))
            })
            .collect();
        if checks.is_empty() {
            carried.push((index, mapping));
            continue;
        }

        // The error column is a bitmask over the checks, bit n set when check n fails, and
        // 0 when none does: nulling the 0 would read the mask twice, as the streaming
        // engine evaluates an expression once per use. Templates, not errors: a literal is
        // broadcast to every row, so they are resolved once the frame is filtered down
        let zero = || create_mask_literal(0);
        let mut error_expr = checks
            .iter()
            .enumerate()
            .filter(|(_, (is_short_circuit, _))| !is_short_circuit)
            .map(|(error_tag, (_, check))| {
                when(check.is_error_expr.clone())
                    .then(create_mask_literal(1 << error_tag))
                    .otherwise(zero())
            })
            .reduce(|left, right| left + right)
            .unwrap_or_else(zero);
        for (error_tag, (is_short_circuit, check)) in checks.iter().enumerate().rev() {
            if *is_short_circuit {
                error_expr = when(check.is_error_expr.clone())
                    .then(create_mask_literal(1 << error_tag))
                    .otherwise(error_expr);
            }
        }

        checked.push(CheckedColumn {
            index,
            mapping,
            templates: checks
                .into_iter()
                .map(|(_, check)| check.error_template)
                .collect(),
            error_expr,
        });
    }

    if checked.is_empty() {
        if !flags.is_empty() {
            table
                .select(create_flag_exprs(flags, &HashMap::new()))
                .filter(any_horizontal(is_flagged).into_result()?)
                .collect_table()?;
        }
        return Ok(errors);
    }

    // The targets come first so the flags read them: a flag parsing a column again
    // would parse it twice, as the streaming engine shares no repeated expression
    let target_names: HashMap<String, String> = checked
        .iter()
        .map(|column| {
            (
                column.mapping.source.name.clone(),
                get_target_name(column.index),
            )
        })
        .collect();
    let mut selection = vec![col(NUMBER_COLUMN_NAME)];
    selection.extend(create_flag_exprs(flags, &target_names));
    for column in &checked {
        let index = column.index;
        selection.push(col(get_target_name(index)));
        selection.push(normalize_column(column.mapping, true).alias(get_source_name(index)));
        selection.push(col(column.mapping.source.name.as_str()).alias(get_cell_name(index)));
    }
    for (index, mapping) in &carried {
        selection.push(col(mapping.source.name.as_str()).alias(get_cell_name(*index)));
    }
    let check_table = table
        .with_row_index(NUMBER_COLUMN_NAME, Some(1))
        .with_columns(
            checked
                .iter()
                .map(|column| {
                    normalize_column(column.mapping, false).alias(get_target_name(column.index))
                })
                .collect::<Vec<_>>(),
        )
        .select(selection)
        .with_columns(
            checked
                .iter()
                .map(|column| {
                    column
                        .error_expr
                        .clone()
                        .alias(get_error_name(column.index))
                })
                .collect::<Vec<_>>(),
        );

    // A lazy frame caches nothing, so every column shares this one collect: a collect
    // per column would parse the whole source once for each. The flags join the filter
    // only so the optimizer keeps them; they are never true
    let mut is_failed: Vec<Expr> = checked
        .iter()
        .map(|column| col(get_error_name(column.index)).neq(create_mask_literal(0)))
        .collect();
    is_failed.extend(is_flagged);
    let mut output = vec![col(NUMBER_COLUMN_NAME).cast(DataType::UInt64)];
    for column in &checked {
        output.push(col(get_cell_name(column.index)));
        output.push(col(get_error_name(column.index)));
    }
    for (index, _) in &carried {
        output.push(col(get_cell_name(*index)));
    }
    let check_frame = check_table
        .filter(any_horizontal(is_failed).into_result()?)
        .select(output)
        .limit(max_errors as IdxSize)
        .collect_table()?;

    let row_numbers = check_frame.column(NUMBER_COLUMN_NAME).into_result()?;
    let row_numbers = row_numbers.u64().into_result()?;
    for column in &checked {
        let cells = check_frame
            .column(&get_cell_name(column.index))
            .into_result()?;
        let error_masks = check_frame
            .column(&get_error_name(column.index))
            .into_result()?;
        let error_masks = error_masks.u32().into_result()?;
        let column_errors = errors
            .entry(column.mapping.target.name().to_string())
            .or_default();

        for row in 0..check_frame.height() {
            let Some(error_mask) = error_masks.get(row).filter(|mask| *mask != 0) else {
                continue;
            };
            let row_number = row_numbers.get(row).unwrap_or_default() as usize;
            let cell = stringify_cell(&cells.get(row).into_result()?);
            for (error_tag, template) in column.templates.iter().enumerate() {
                if error_mask >> error_tag & 1 == 1 {
                    column_errors.push(TableError::Cell(locate_cell_error(
                        template,
                        row_number,
                        cell.clone(),
                    )?));
                }
            }
        }
    }

    Ok(errors)
}

pub(crate) fn locate_cell_error(
    template: &CellError,
    row_number: usize,
    cell: String,
) -> Result<CellError> {
    let mut error = serde_json::to_value(template).map_err(create_error)?;
    if let Value::Object(object) = &mut error {
        object.insert("rowNumber".to_string(), Value::from(row_number));
        object.insert("cell".to_string(), Value::String(cell));
    }
    serde_json::from_value(error).map_err(create_error)
}

fn create_error(error: serde_json::Error) -> Error {
    Error::Load(error.to_string())
}

// A bare literal is a dynamic integer, which would leave an all-literal mask Int32
fn create_mask_literal(value: u32) -> Expr {
    lit(value).cast(DataType::UInt32)
}

fn create_flag_exprs(flags: &[FlagFactory], target_names: &HashMap<String, String>) -> Vec<Expr> {
    let read_typed = |name: &str| target_names.get(name).map(|target| col(target.as_str()));
    flags
        .iter()
        .enumerate()
        .map(|(index, create_flag)| create_flag(&read_typed).alias(get_flag_name(index)))
        .collect()
}

fn get_target_name(index: usize) -> String {
    format!("fairspec:target:{index}")
}

fn get_source_name(index: usize) -> String {
    format!("fairspec:source:{index}")
}

fn get_cell_name(index: usize) -> String {
    format!("fairspec:cell:{index}")
}

fn get_flag_name(index: usize) -> String {
    format!("fairspec:flag:{index}")
}

fn get_error_name(index: usize) -> String {
    format!("fairspec:error:{index}")
}
