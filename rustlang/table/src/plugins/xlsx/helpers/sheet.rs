use calamine::{Data, DataType, Reader, open_workbook_auto};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use fairspec_metadata::{Error, FileDialect, Result};

use super::guard::assert_sheet_size;

#[derive(Debug, Clone, PartialEq)]
pub enum SheetCell {
    String(String),
    Number(f64),
    Boolean(bool),
    DateTime(NaiveDateTime),
}

/// Reads the dialect's sheet as rows of cells, without trailing empty cells; `None` when
/// the workbook has no such sheet.
pub fn read_sheet_rows(
    path: &str,
    file_dialect: &FileDialect,
    max_rows: Option<usize>,
) -> Result<Option<Vec<Vec<Option<SheetCell>>>>> {
    let (sheet_name, sheet_number) = match file_dialect {
        FileDialect::Xlsx(dialect) => (dialect.sheet_name.clone(), dialect.sheet_number),
        FileDialect::Ods(dialect) => (dialect.sheet_name.clone(), dialect.sheet_number),
        _ => (None, None),
    };
    let index = sheet_number
        .filter(|number| *number > 0)
        .map_or(0, |number| number - 1);

    assert_sheet_size(path)?;
    let mut workbook = open_workbook_auto(path).map_err(|error| Error::Load(error.to_string()))?;

    let names = workbook.sheet_names();
    let Some(name) = sheet_name.or_else(|| names.get(index).cloned()) else {
        return Ok(None);
    };
    if !names.contains(&name) {
        return Ok(None);
    }
    let range = workbook
        .worksheet_range(&name)
        .map_err(|error| Error::Load(error.to_string()))?;

    // python-calamine's default `nrows` is the end row's absolute index plus one
    let row_count = max_rows.unwrap_or_else(|| range.end().map_or(0, |end| end.0 as usize + 1));
    let rows = range.rows().take(row_count).map(normalize_row).collect();
    Ok(Some(rows))
}

fn normalize_row(row: &[Data]) -> Vec<Option<SheetCell>> {
    let mut cells: Vec<Option<SheetCell>> = row.iter().map(convert_cell).collect();
    while cells.last().is_some_and(Option::is_none) {
        cells.pop();
    }
    cells
}

/// python-calamine's cell value, then the normalization Python's twin applies to it.
/// TypeScript's twin maps calamine-node cells to the same values: a time-only value lands
/// on 1899-12-31 and a duration is a number of days.
fn convert_cell(value: &Data) -> Option<SheetCell> {
    match convert_calamine_cell(value) {
        CellValue::Empty => None,
        CellValue::String(text) if text.is_empty() => None,
        CellValue::String(text) => Some(SheetCell::String(text)),
        CellValue::Bool(value) => Some(SheetCell::Boolean(value)),
        CellValue::Int(value) => Some(SheetCell::Number(value as f64)),
        CellValue::Float(value) => Some(SheetCell::Number(value)),
        CellValue::DateTime(value) => Some(SheetCell::DateTime(round_milliseconds(value))),
        CellValue::Date(value) => Some(SheetCell::DateTime(value.and_time(NaiveTime::MIN))),
        CellValue::Time(value) => {
            let epoch = NaiveDate::from_ymd_opt(1899, 12, 31).unwrap_or_default();
            Some(SheetCell::DateTime(round_milliseconds(
                epoch.and_time(value),
            )))
        }
        CellValue::Timedelta(value) => {
            let microseconds = value.num_microseconds().unwrap_or(i64::MAX);
            Some(SheetCell::Number(microseconds as f64 / 1e6 / 86400.0))
        }
    }
}

enum CellValue {
    Int(i64),
    Float(f64),
    String(String),
    Time(NaiveTime),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Timedelta(Duration),
    Bool(bool),
    Empty,
}

/// python-calamine's `CellValue::from`, with Python's datetime year range.
fn convert_calamine_cell(value: &Data) -> CellValue {
    let check_year = |year: i32| (1..=9999).contains(&year);
    if let Some(value) = value.get_int().filter(|_| value.is_int()) {
        CellValue::Int(value)
    } else if let Some(value) = value.get_float().filter(|_| value.is_float()) {
        CellValue::Float(value)
    } else if value.is_string() {
        value
            .get_string()
            .map_or(CellValue::Empty, |text| CellValue::String(text.to_string()))
    } else if value.is_datetime() {
        let Some(datetime) = value.get_datetime() else {
            return CellValue::Empty;
        };
        let number = datetime.as_f64();
        let converted = if datetime.is_duration() {
            value.as_duration().map(CellValue::Timedelta)
        } else if number < 1.0 {
            value.as_time().map(CellValue::Time)
        } else if number == (number as u64) as f64 {
            value
                .as_date()
                .filter(|date| check_year(chrono::Datelike::year(date)))
                .map(CellValue::Date)
        } else {
            value
                .as_datetime()
                .filter(|datetime| check_year(chrono::Datelike::year(datetime)))
                .map(CellValue::DateTime)
        };
        converted.unwrap_or(CellValue::Float(number))
    } else if value.is_datetime_iso() {
        let text = value.get_datetime_iso().unwrap_or_default().to_string();
        let converted = if text.contains('T') {
            value
                .as_datetime()
                .filter(|datetime| check_year(chrono::Datelike::year(datetime)))
                .map(CellValue::DateTime)
        } else if text.contains(':') {
            value.as_time().map(CellValue::Time)
        } else {
            value
                .as_date()
                .filter(|date| check_year(chrono::Datelike::year(date)))
                .map(CellValue::Date)
        };
        converted.unwrap_or(CellValue::String(text))
    } else if value.is_duration_iso() {
        match value.as_time() {
            Some(time) => CellValue::Time(time),
            None => value
                .get_duration_iso()
                .map_or(CellValue::Empty, |text| CellValue::String(text.to_string())),
        }
    } else if let Some(value) = value.get_bool().filter(|_| value.is_bool()) {
        CellValue::Bool(value)
    } else {
        CellValue::Empty
    }
}

/// Python's datetime holds microseconds (pyo3 truncates chrono's nanoseconds), and
/// Python's twin rounds them half to even into milliseconds.
fn round_milliseconds(value: NaiveDateTime) -> NaiveDateTime {
    let microseconds = value.nanosecond() / 1000;
    let milliseconds = (microseconds as f64 / 1000.0).round_ties_even() as i64;
    let truncated = value.with_nanosecond(0).unwrap_or(value);
    truncated + Duration::milliseconds(milliseconds)
}
