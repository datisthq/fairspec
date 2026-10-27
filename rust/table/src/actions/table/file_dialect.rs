use fairspec_metadata::{
    Error, FairspecError, FileDialect, FileError, HeaderError, Result, create_report,
};
use polars::prelude::*;

use crate::helpers::error::PolarsResultExt;
use crate::helpers::file_dialect::{
    assert_unique_column_names, get_comment_rows, get_header_rows, join_header_labels,
};
use crate::helpers::table::LazyFrameExt;
use crate::helpers::value::get_json_value;
use crate::models::data::DataRow;
use crate::models::file_dialect::FileDialectWithHeaderAndCommentRows;
use crate::models::table::Table;
use crate::settings::NUMBER_COLUMN_NAME;

/// Concatenates the tables of a multipart resource's parts; under a dialect with header
/// rows every part's header gives the first part's column names, else a `file/header`.
/// With `columnNames` the tables carry those names, so `part_headers` gives each part's own.
pub fn concat_part_tables(
    tables: Vec<Table>,
    dialect: &FileDialectWithHeaderAndCommentRows,
    part_headers: Option<Vec<Vec<String>>>,
) -> Result<Table> {
    if !get_header_rows(Some(dialect)).is_empty() {
        let part_headers = match part_headers {
            Some(part_headers) => part_headers,
            None => tables
                .iter()
                .map(|table| {
                    Ok(table
                        .clone()
                        .collect_schema()
                        .into_result()?
                        .iter_names()
                        .map(|name| name.to_string())
                        .collect())
                })
                .collect::<Result<_>>()?,
        };
        assert_part_headers(&part_headers)?;
    }
    concat(tables, UnionArgs::default()).into_result()
}

/// Fails with a `file/header` error for the first part whose header differs from part 1's.
pub fn assert_part_headers(part_headers: &[Vec<String>]) -> Result<()> {
    let Some(first_names) = part_headers.first() else {
        return Ok(());
    };
    for (index, names) in part_headers.iter().enumerate() {
        if names != first_names {
            return Err(create_part_header_error(index + 1, names.clone()));
        }
    }
    Ok(())
}

/// Concatenates tables by column name, as Polars' diagonal concat: the columns are the
/// union of the tables' in the order first seen, null where a table has none, each
/// cast to the supertype of its parts as Polars' relaxed concat does.
pub fn concat_diagonal_tables(tables: Vec<Table>) -> Result<Table> {
    let mut schemas = Vec::new();
    let mut fields: Vec<(PlSmallStr, DataType)> = Vec::new();
    for table in &tables {
        let schema = table.clone().collect_schema().into_result()?;
        for (name, dtype) in schema.iter() {
            if !fields.iter().any(|(field, _)| field == name) {
                fields.push((name.clone(), dtype.clone()));
            }
        }
        schemas.push(schema);
    }
    let tables: Vec<Table> = tables
        .into_iter()
        .zip(&schemas)
        .map(|(table, schema)| {
            let exprs: Vec<Expr> = fields
                .iter()
                .map(|(name, dtype)| match schema.contains(name) {
                    true => col(name.clone()),
                    false => lit(NULL).cast(dtype.clone()).alias(name.clone()),
                })
                .collect();
            table.select(exprs)
        })
        .collect();
    let args = UnionArgs {
        to_supertypes: true,
        ..Default::default()
    };
    concat(tables, args).into_result()
}

/// The error of a part whose header differs from the first part's, or that has none.
pub fn create_part_header_error(part_number: usize, column_names: Vec<String>) -> Error {
    let error = HeaderError {
        resource_name: None,
        part_number,
        column_names,
    };
    Error::Invalid {
        message: format!("Part {part_number} has another header than part 1"),
        report: Some(create_report(
            vec![FairspecError::File(FileError::Header(error))],
            None,
        )),
    }
}

/// Tells the error of a part whose header differs, which a validator reports.
pub fn get_is_part_header_error(error: &Error) -> bool {
    error.report().is_some_and(|report| {
        !report.errors.is_empty()
            && report
                .errors
                .iter()
                .all(|item| matches!(item, FairspecError::File(FileError::Header(_))))
    })
}

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

    // Every row up to the last header row is consumed by the header, listed or not
    let skipped_rows = header_rows[0].saturating_sub(1);
    let last_header_row = header_rows[header_rows.len() - 1];
    let has_column_names = get_column_names(dialect).is_some_and(|names| !names.is_empty());
    let height = last_header_row - skipped_rows - usize::from(has_column_names);
    let header_frame = table.clone().limit(height as u32).collect_table()?;
    if header_frame.height() < height {
        return Err(Error::Load(format!(
            "Header row {last_header_row} is past the end of the file"
        )));
    }
    if has_column_names {
        return Ok(drop_first_rows(table, height));
    }
    let header: Vec<DataRow> = header_rows
        .iter()
        .map(|number| {
            header_frame
                .get_row(number - skipped_rows - 1)
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
            (
                name.to_string(),
                labels
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| name.to_string()),
            )
        })
        .unzip();
    assert_unique_column_names(&new)?;

    Ok(drop_first_rows(table, height).rename(existing, new, true))
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

fn drop_first_rows(table: Table, count: usize) -> Table {
    table
        .with_row_index(NUMBER_COLUMN_NAME, Some(1))
        .filter(
            col(NUMBER_COLUMN_NAME)
                .cast(DataType::Int64)
                .gt(lit(count as i64)),
        )
        .drop(by_name([NUMBER_COLUMN_NAME], true, false))
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
