use std::collections::HashSet;

use fairspec_metadata::{
    CellCheckError, CellError, CheckError, Result, RowCheckError, RowError, TableCheckError,
    TableError,
};
use polars::prelude::*;
use polars::sql::sql_expr;

use crate::actions::column::_shared::python::stringify_cell;
use crate::actions::table::normalize::normalize_columns;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::table::LazyFrameExt;
use crate::models::check::{TableCheck, TableCheckCondition};
use crate::models::schema::SchemaMapping;
use crate::models::table::Table;
use crate::settings::NUMBER_COLUMN_NAME;

/// Checks every row of the table against each condition, or the whole table when a
/// condition reduces to one value; conditions see the values typed by the schema.
pub fn inspect_table_checks(
    table: Table,
    checks: &[TableCheck],
    mapping: Option<&SchemaMapping>,
    max_errors: usize,
) -> Result<Vec<TableError>> {
    let mut errors = Vec::new();
    if checks.is_empty() {
        return Ok(errors);
    }

    let schema = create_typed_table(table.clone(), mapping, &[])?
        .collect_schema()
        .into_result()?;
    let items: Vec<CheckItem> = checks
        .iter()
        .enumerate()
        .map(|(index, check)| infer_check_item(index, check, &schema))
        .collect();

    let row_items: Vec<&CheckItem> = items.iter().filter(|i| i.kind == Kind::Row).collect();
    let rows = collect_failing_rows(table.clone(), &row_items, mapping, max_errors)?;

    let table_items: Vec<&CheckItem> = items.iter().filter(|i| i.kind == Kind::Table).collect();
    let values = collect_table_values(create_typed_table(table, mapping, &[])?, &table_items)?;

    for item in &items {
        let check = item.check;

        match item.kind {
            Kind::Invalid => {
                errors.push(TableError::Check(CheckError {
                    r#type: Default::default(),
                    resource_name: None,
                    check_name: check.name.clone(),
                    message: item.message.clone(),
                }));
            }
            Kind::Table => {
                let value = values.iter().find(|(index, _)| *index == item.index);
                if value.is_some_and(|(_, value)| *value == Some(false)) {
                    errors.push(TableError::TableCheck(TableCheckError {
                        r#type: Default::default(),
                        resource_name: None,
                        check_name: check.name.clone(),
                        message: check.message.clone(),
                    }));
                }
            }
            Kind::Row => {
                let position = row_items
                    .iter()
                    .position(|row_item| row_item.index == item.index)
                    .unwrap_or_default();
                for row in &rows {
                    if row.results[position] != Some(false) {
                        continue;
                    }

                    let Some(column_name) = &check.column_name else {
                        errors.push(TableError::Row(RowError::Check(RowCheckError {
                            resource_name: None,
                            row_number: row.number,
                            check_name: check.name.clone(),
                            message: check.message.clone(),
                        })));
                        continue;
                    };

                    let cell = row
                        .cells
                        .iter()
                        .find(|(index, _)| *index == item.index)
                        .map(|(_, cell)| cell.clone())
                        .unwrap_or_default();
                    errors.push(TableError::Cell(CellError::Check(CellCheckError {
                        resource_name: None,
                        column_name: column_name.clone(),
                        row_number: row.number,
                        cell,
                        check_name: check.name.clone(),
                        message: check.message.clone(),
                    })));
                }
            }
        }
    }

    Ok(errors)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Row,
    Table,
    Invalid,
}

struct CheckItem<'a> {
    index: usize,
    check: &'a TableCheck,
    kind: Kind,
    message: String,
}

struct FailingRow {
    number: usize,
    cells: Vec<(usize, String)>,
    results: Vec<Option<bool>>,
}

// A condition is run on two typed rows of nulls: a row check returns both and an
// aggregate returns one, which would otherwise be broadcast to fail every row
fn infer_check_item<'a>(index: usize, check: &'a TableCheck, schema: &Schema) -> CheckItem<'a> {
    let item = |kind, message: String| CheckItem {
        index,
        check,
        kind,
        message,
    };

    if let Some(column_name) = &check.column_name
        && schema.get(column_name.as_str()).is_none()
    {
        return item(
            Kind::Invalid,
            format!("Column \"{column_name}\" is not found"),
        );
    }

    let name = get_check_name(index);
    let frame = create_condition(check).and_then(|condition| {
        let nulls: Vec<Expr> = schema
            .iter()
            .filter(|(name, _)| name.as_str() != NUMBER_COLUMN_NAME)
            .map(|(name, dtype)| lit(NULL).cast(dtype.clone()).alias(name.clone()))
            .collect();
        df!(NUMBER_COLUMN_NAME => [1i64, 2])?
            .lazy()
            .with_columns(nulls)
            .drop(by_name([NUMBER_COLUMN_NAME], true, false))
            .select([condition.alias(name.as_str())])
            .collect()
    });
    let frame = match frame {
        Ok(frame) => frame,
        Err(error) => {
            let message = error.to_string();
            let message = message.split('\n').next().unwrap_or_default().to_string();
            return item(Kind::Invalid, message);
        }
    };

    let dtype = frame
        .schema()
        .get(name.as_str())
        .cloned()
        .unwrap_or(DataType::Null);
    if dtype != DataType::Boolean {
        let message = format!(
            "Condition is {} instead of Boolean",
            get_base_type_name(&dtype)
        );
        return item(Kind::Invalid, message);
    }

    let kind = if frame.height() == 1 {
        Kind::Table
    } else {
        Kind::Row
    };
    item(kind, String::new())
}

fn collect_failing_rows(
    table: Table,
    items: &[&CheckItem],
    mapping: Option<&SchemaMapping>,
    max_errors: usize,
) -> Result<Vec<FailingRow>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let cell_items: Vec<&CheckItem> = items
        .iter()
        .copied()
        .filter(|item| item.check.column_name.is_some())
        .collect();
    let check_names: Vec<String> = items
        .iter()
        .map(|item| get_check_name(item.index))
        .collect();
    let conditions = items
        .iter()
        .map(|item| {
            Ok(create_condition(item.check)
                .into_result()?
                .alias(get_check_name(item.index)))
        })
        .collect::<Result<Vec<Expr>>>()?;
    let check_table = create_typed_table(table, mapping, &cell_items)?.with_columns(conditions);

    let is_failing: Vec<Expr> = check_names
        .iter()
        .map(|name| col(name.as_str()).not().fill_null(lit(false)))
        .collect();
    let mut selected = vec![col(NUMBER_COLUMN_NAME)];
    selected.extend(cell_items.iter().map(|item| col(get_cell_name(item.index))));
    selected.extend(check_names.iter().map(|name| col(name.as_str())));

    let frame = check_table
        .filter(any_horizontal(is_failing).into_result()?)
        .select(selected)
        .limit(max_errors as IdxSize)
        .collect_table()?;

    let numbers = frame
        .column(NUMBER_COLUMN_NAME)
        .and_then(|column| column.cast(&DataType::UInt64))
        .into_result()?;
    let numbers = numbers.u64().into_result()?;
    let mut cell_columns = Vec::new();
    for item in &cell_items {
        cell_columns.push((
            item.index,
            frame.column(&get_cell_name(item.index)).into_result()?,
        ));
    }
    let mut result_columns = Vec::new();
    for name in &check_names {
        result_columns.push(
            frame
                .column(name)
                .and_then(|c| c.bool().cloned())
                .into_result()?,
        );
    }

    let mut rows = Vec::with_capacity(frame.height());
    for position in 0..frame.height() {
        let mut cells = Vec::with_capacity(cell_columns.len());
        for (index, column) in &cell_columns {
            let value = column.get(position).into_result()?;
            cells.push((*index, stringify_cell(&value)));
        }
        rows.push(FailingRow {
            number: numbers.get(position).unwrap_or_default() as usize,
            cells,
            results: result_columns
                .iter()
                .map(|column| column.get(position))
                .collect(),
        });
    }
    Ok(rows)
}

fn collect_table_values(table: Table, items: &[&CheckItem]) -> Result<Vec<(usize, Option<bool>)>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let conditions = items
        .iter()
        .map(|item| {
            Ok(create_condition(item.check)
                .into_result()?
                .alias(get_check_name(item.index)))
        })
        .collect::<Result<Vec<Expr>>>()?;
    let frame = table.select(conditions).collect_table()?;

    let mut values = Vec::with_capacity(items.len());
    for item in items {
        let column = frame.column(&get_check_name(item.index)).into_result()?;
        let value = match column.get(0) {
            Ok(AnyValue::Boolean(value)) => Some(value),
            _ => None,
        };
        values.push((item.index, value));
    }
    Ok(values)
}

fn create_typed_table(
    table: Table,
    mapping: Option<&SchemaMapping>,
    cell_items: &[&CheckItem],
) -> Result<Table> {
    let source_names: HashSet<&str> = mapping
        .map(|mapping| {
            mapping
                .source
                .columns
                .iter()
                .map(|column| column.name.as_str())
                .collect()
        })
        .unwrap_or_default();

    let cell_columns: Vec<Expr> = cell_items
        .iter()
        .map(|item| {
            let column_name = item.check.column_name.as_deref().unwrap_or_default();
            let is_sourced = mapping.is_none() || source_names.contains(column_name);
            let cell_column = if is_sourced {
                col(column_name)
            } else {
                lit(NULL)
            };
            cell_column.alias(get_cell_name(item.index))
        })
        .collect();

    let numbered = table.with_row_index(NUMBER_COLUMN_NAME, Some(1));
    let Some(mapping) = mapping else {
        return Ok(numbered.with_columns(cell_columns));
    };

    let mut columns = vec![col(NUMBER_COLUMN_NAME)];
    columns.extend(normalize_columns(mapping)?.into_values());
    columns.extend(cell_columns);
    Ok(numbered.select(columns))
}

fn create_condition(check: &TableCheck) -> PolarsResult<Expr> {
    match &check.condition {
        TableCheckCondition::Sql(condition) => sql_expr(condition),
        TableCheckCondition::Expr(condition) => Ok(condition.clone()),
    }
}

// The name of the polars class of a data type, as Python prints it
fn get_base_type_name(dtype: &DataType) -> String {
    let name = match dtype {
        DataType::Boolean => "Boolean",
        DataType::UInt8 => "UInt8",
        DataType::UInt16 => "UInt16",
        DataType::UInt32 => "UInt32",
        DataType::UInt64 => "UInt64",
        DataType::Int8 => "Int8",
        DataType::Int16 => "Int16",
        DataType::Int32 => "Int32",
        DataType::Int64 => "Int64",
        DataType::Int128 => "Int128",
        DataType::Float32 => "Float32",
        DataType::Float64 => "Float64",
        DataType::String => "String",
        DataType::Binary => "Binary",
        DataType::Date => "Date",
        DataType::Datetime(_, _) => "Datetime",
        DataType::Duration(_) => "Duration",
        DataType::Time => "Time",
        DataType::List(_) => "List",
        DataType::Array(_, _) => "Array",
        DataType::Struct(_) => "Struct",
        DataType::Decimal(_, _) => "Decimal",
        DataType::Categorical(_, _) => "Categorical",
        DataType::Enum(_, _) => "Enum",
        DataType::Null => "Null",
        other => return format!("{other:?}"),
    };
    name.to_string()
}

fn get_check_name(index: usize) -> String {
    format!("fairspec:check:{index}")
}

fn get_cell_name(index: usize) -> String {
    format!("fairspec:check-cell:{index}")
}
