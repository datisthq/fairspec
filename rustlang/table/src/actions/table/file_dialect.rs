use fairspec_metadata::{FileDialect, Result};
use polars::prelude::*;

use crate::helpers::error::PolarsResultExt;
use crate::helpers::file_dialect::{get_comment_rows, get_header_rows, join_header_labels};
use crate::helpers::value::get_json_value;
use crate::models::data::DataRow;
use crate::models::file_dialect::FileDialectWithHeaderAndCommentRows;
use crate::models::table::Table;
use crate::settings::NUMBER_COLUMN_NAME;

/// Joins several header rows into column names. The table is read without a header,
/// so its header rows are still data rows; with `columnNames` the first header row
/// was read as the header and is gone.
pub fn join_header_rows(
    table: Table,
    dialect: &FileDialectWithHeaderAndCommentRows,
) -> Result<Table> {
    let header_rows = get_header_rows(Some(dialect));
    if header_rows.len() < 2 {
        return Ok(table);
    }

    let skipped_rows = header_rows[0] - 1;
    if get_column_names(dialect).is_some_and(|names| !names.is_empty()) {
        return Ok(drop_rows(table, &header_rows, skipped_rows + 1));
    }

    let height = header_rows[header_rows.len() - 1] - skipped_rows;
    let header_frame = table.clone().limit(height as u32).collect().into_result()?;
    let header: Vec<DataRow> = header_rows
        .iter()
        .map(|number| {
            let index = number - skipped_rows - 1;
            if index >= header_frame.height() {
                return Vec::new();
            }
            header_frame
                .get_row(index)
                .map(|row| row.0.iter().map(get_json_value).collect())
                .unwrap_or_default()
        })
        .collect();
    let header_join = get_header_join(dialect).unwrap_or(" ");
    let labels = join_header_labels(&header, header_join);

    let names = table.clone().collect_schema().into_result()?;
    let (existing, new): (Vec<String>, Vec<String>) = names
        .iter_names()
        .enumerate()
        .map(|(index, name)| {
            let label = labels.get(index).filter(|label| !label.is_empty());
            (
                name.to_string(),
                label.cloned().unwrap_or_else(|| name.to_string()),
            )
        })
        .unzip();

    Ok(drop_rows(table, &header_rows, skipped_rows).rename(existing, new, true))
}

pub fn skip_comment_rows(table: Table, dialect: &FileDialectWithHeaderAndCommentRows) -> Table {
    let header_rows = get_header_rows(Some(dialect));
    let comment_offset = header_rows.last().copied().unwrap_or(0);
    let comment_rows = get_comment_rows(dialect);
    if comment_rows.is_empty() {
        return table;
    }
    drop_rows(table, &comment_rows, comment_offset)
}

fn drop_rows(table: Table, numbers: &[usize], offset: usize) -> Table {
    let numbers: Vec<i64> = numbers.iter().map(|number| *number as i64).collect();
    table
        .with_row_index(NUMBER_COLUMN_NAME, Some(1))
        .filter(
            (col(NUMBER_COLUMN_NAME).cast(DataType::Int64) + lit(offset as i64))
                .is_in(lit(Series::new("".into(), numbers)).implode(true), false)
                .not(),
        )
        .drop(by_name([NUMBER_COLUMN_NAME], true, false))
}

fn get_column_names(dialect: &FileDialect) -> Option<&Vec<String>> {
    match dialect {
        FileDialect::Csv(dialect) => dialect.column_names.as_ref(),
        FileDialect::Tsv(dialect) => dialect.column_names.as_ref(),
        FileDialect::Json(dialect) => dialect.column_names.as_ref(),
        FileDialect::Jsonl(dialect) => dialect.column_names.as_ref(),
        FileDialect::Xlsx(dialect) => dialect.column_names.as_ref(),
        FileDialect::Ods(dialect) => dialect.column_names.as_ref(),
        _ => None,
    }
}

fn get_header_join(dialect: &FileDialect) -> Option<&str> {
    match dialect {
        FileDialect::Csv(dialect) => dialect.header_join.as_deref(),
        FileDialect::Tsv(dialect) => dialect.header_join.as_deref(),
        FileDialect::Json(dialect) => dialect.header_join.as_deref(),
        FileDialect::Jsonl(dialect) => dialect.header_join.as_deref(),
        FileDialect::Xlsx(dialect) => dialect.header_join.as_deref(),
        FileDialect::Ods(dialect) => dialect.header_join.as_deref(),
        _ => None,
    }
}
