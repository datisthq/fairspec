use chrono::NaiveDateTime;
use fairspec_metadata::{Error, FileDialect, Result};
use polars::prelude::{
    Column as PolarsColumn, DataFrame, DataType, IntoLazy, LazyFrame, NamedFrom, Series, TimeUnit,
};
use serde_json::Value;

use super::sheet::SheetCell;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::file_dialect::{
    assert_unique_column_names, get_header_rows, join_header_labels,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SheetType {
    Number,
    Boolean,
    Date,
    String,
}

/// Builds a table from sheet rows. Data starts after the last header row; a column keeps
/// its cells' type when they share one and becomes text when they mix.
pub fn create_table_from_sheet_rows(
    rows: &[Vec<Option<SheetCell>>],
    file_dialect: &FileDialect,
) -> Result<LazyFrame> {
    let header_rows = get_header_rows(Some(file_dialect));
    let data_start = header_rows.iter().copied().max().unwrap_or(0);
    let content = rows.get(data_start..).unwrap_or_default();
    let options = get_sheet_options(file_dialect);

    let labels = match options.column_names {
        Some(column_names) => column_names.clone(),
        None => {
            let labels = get_sheet_labels(rows, content, &header_rows, options.header_join)?;
            assert_unique_column_names(&labels)?;
            labels
        }
    };

    let first_row = header_rows.first().map_or(0, |row| row.saturating_sub(1));
    let width = rows
        .get(first_row..)
        .unwrap_or_default()
        .iter()
        .map(|row| {
            row.iter()
                .rposition(Option::is_some)
                .map_or(0, |index| index + 1)
        })
        .max()
        .unwrap_or(0);
    if options.column_names.is_some() && width > labels.len() {
        return Err(Error::Load(format!(
            "The sheet has {width} columns but columnNames has {}",
            labels.len()
        )));
    }

    let columns: Vec<Vec<Option<&SheetCell>>> = (0..labels.len())
        .map(|index| {
            content
                .iter()
                .map(|row| row.get(index).and_then(Option::as_ref))
                .collect()
        })
        .collect();
    let types: Vec<SheetType> = columns
        .iter()
        .map(|column| get_column_type(column))
        .collect();

    let comment_rows = options.comment_rows.cloned().unwrap_or_default();
    let kept: Vec<usize> = content
        .iter()
        .enumerate()
        .filter(|(index, row)| {
            if comment_rows.contains(&(data_start + index + 1)) {
                return false;
            }
            let first = row.first().and_then(Option::as_ref);
            !matches!(
                (options.comment_prefix, first),
                (Some(prefix), Some(SheetCell::String(text)))
                    if !prefix.is_empty() && text.starts_with(prefix.as_str())
            )
        })
        .map(|(index, _)| index)
        .collect();

    let series = labels
        .iter()
        .zip(&columns)
        .zip(&types)
        .map(|((label, column), kind)| {
            let values: Vec<Option<&SheetCell>> = kept.iter().map(|&row| column[row]).collect();
            create_sheet_series(label, &values, *kind).map(PolarsColumn::from)
        })
        .collect::<Result<Vec<_>>>()?;

    let height = if series.is_empty() { 0 } else { kept.len() };
    Ok(DataFrame::new(height, series).into_result()?.lazy())
}

/// Formats a cell as the text a mixed-type column holds.
pub fn format_sheet_cell(cell: Option<&SheetCell>) -> Option<String> {
    match cell? {
        SheetCell::Boolean(value) => Some(value.to_string()),
        SheetCell::Number(value) => Some(format_number(*value)),
        SheetCell::DateTime(value) => Some(format_date(value)),
        SheetCell::String(value) => Some(value.clone()),
    }
}

struct SheetOptions<'a> {
    column_names: Option<&'a Vec<String>>,
    header_join: Option<&'a String>,
    comment_rows: Option<&'a Vec<usize>>,
    comment_prefix: Option<&'a String>,
}

fn get_sheet_options(file_dialect: &FileDialect) -> SheetOptions<'_> {
    match file_dialect {
        FileDialect::Xlsx(dialect) => SheetOptions {
            column_names: dialect.column_names.as_ref(),
            header_join: dialect.header_join.as_ref(),
            comment_rows: dialect.comment_rows.as_ref(),
            comment_prefix: dialect.comment_prefix.as_ref(),
        },
        FileDialect::Ods(dialect) => SheetOptions {
            column_names: dialect.column_names.as_ref(),
            header_join: dialect.header_join.as_ref(),
            comment_rows: dialect.comment_rows.as_ref(),
            comment_prefix: dialect.comment_prefix.as_ref(),
        },
        _ => SheetOptions {
            column_names: None,
            header_join: None,
            comment_rows: None,
            comment_prefix: None,
        },
    }
}

fn get_sheet_labels(
    rows: &[Vec<Option<SheetCell>>],
    content: &[Vec<Option<SheetCell>>],
    header_rows: &[usize],
    header_join: Option<&String>,
) -> Result<Vec<String>> {
    if header_rows.is_empty() {
        let width = content.iter().map(Vec::len).max().unwrap_or(0);
        return Ok((1..=width).map(|index| format!("column{index}")).collect());
    }

    let header: Vec<Vec<Value>> = header_rows
        .iter()
        .map(|number| {
            let row = number.checked_sub(1).and_then(|index| rows.get(index));
            let row = row.ok_or_else(|| {
                Error::Load(format!("Header row {number} is past the end of the sheet"))
            })?;
            Ok(row
                .iter()
                .map(|cell| Value::String(format_sheet_cell(cell.as_ref()).unwrap_or_default()))
                .collect())
        })
        .collect::<Result<_>>()?;

    Ok(match header.as_slice() {
        [] => vec![],
        [labels] => labels
            .iter()
            .map(|label| label.as_str().unwrap_or_default().to_string())
            .collect(),
        _ => join_header_labels(&header, header_join.map_or(" ", String::as_str)),
    })
}

fn get_column_type(cells: &[Option<&SheetCell>]) -> SheetType {
    let mut kind = None;
    for cell in cells.iter().flatten() {
        let cell_kind = match cell {
            SheetCell::Boolean(_) => SheetType::Boolean,
            SheetCell::Number(_) => SheetType::Number,
            SheetCell::DateTime(_) => SheetType::Date,
            SheetCell::String(_) => SheetType::String,
        };
        if kind.is_some_and(|kind| kind != cell_kind) {
            return SheetType::String;
        }
        kind = Some(cell_kind);
    }
    kind.unwrap_or(SheetType::String)
}

fn create_sheet_series(
    name: &str,
    values: &[Option<&SheetCell>],
    kind: SheetType,
) -> Result<Series> {
    let name = name.into();
    let series = match kind {
        SheetType::Number => {
            let values: Vec<Option<f64>> = values
                .iter()
                .map(|cell| match cell {
                    Some(SheetCell::Number(value)) => Some(*value),
                    _ => None,
                })
                .collect();
            Series::new(name, values)
        }
        SheetType::Boolean => {
            let values: Vec<Option<bool>> = values
                .iter()
                .map(|cell| match cell {
                    Some(SheetCell::Boolean(value)) => Some(*value),
                    _ => None,
                })
                .collect();
            Series::new(name, values)
        }
        SheetType::Date => {
            let values: Vec<Option<i64>> = values
                .iter()
                .map(|cell| match cell {
                    Some(SheetCell::DateTime(value)) => Some(value.and_utc().timestamp_millis()),
                    _ => None,
                })
                .collect();
            Series::new(name, values)
                .cast(&DataType::Datetime(TimeUnit::Milliseconds, None))
                .into_result()?
        }
        SheetType::String => {
            let values: Vec<Option<String>> =
                values.iter().map(|cell| format_sheet_cell(*cell)).collect();
            Series::new(name, values)
        }
    };
    Ok(series)
}

fn format_date(value: &NaiveDateTime) -> String {
    let text = value.format("%Y-%m-%dT%H:%M:%S").to_string();
    let milliseconds = value.and_utc().timestamp_subsec_micros() / 1000;
    if milliseconds == 0 {
        return text;
    }
    format!("{text}.{milliseconds:03}")
}

/// JavaScript's `String(number)`, which TypeScript's twin uses.
fn format_number(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    if value == 0.0 {
        return "0".to_string();
    }

    let sign = if value < 0.0 { "-" } else { "" };
    let scientific = format!("{:e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let k = digits.len() as i64;
    let n = exponent.parse::<i64>().unwrap_or_default() + 1;

    if k <= n && n <= 21 {
        return format!("{sign}{digits}{}", "0".repeat((n - k) as usize));
    }
    if 0 < n && n <= 21 {
        let (integer, fraction) = digits.split_at(n as usize);
        return format!("{sign}{integer}.{fraction}");
    }
    if -6 < n && n <= 0 {
        return format!("{sign}0.{}{digits}", "0".repeat(-n as usize));
    }

    let exponent = n - 1;
    let mantissa = match digits.len() {
        1 => digits.clone(),
        _ => format!("{}.{}", &digits[..1], &digits[1..]),
    };
    let exponent_sign = if exponent >= 0 { "+" } else { "-" };
    format!("{sign}{mantissa}e{exponent_sign}{}", exponent.abs())
}
