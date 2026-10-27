use calamine::{Data, Reader, open_workbook_auto};
use serde_json::{Map, Value};

use crate::plugins::xlsx::helpers::write::{
    SheetData, SheetValue, write_ods_buffer, write_xlsx_buffer,
};

/// Reads the first sheet with calamine itself, not fairspec's loader, to isolate the tests.
/// Whole numbers come back as integers, as Python's `1.0 == 1` lets its twin compare them.
pub fn read_test_data(path: &str) -> Value {
    let mut workbook = open_workbook_auto(path).unwrap();
    let name = workbook.sheet_names()[0].clone();
    let range = workbook.worksheet_range(&name).unwrap();
    let mut rows = range.rows();
    let header: Vec<String> = rows
        .next()
        .unwrap()
        .iter()
        .map(|cell| cell.to_string())
        .collect();
    let records = rows
        .map(|row| {
            header
                .iter()
                .zip(row)
                .map(|(label, cell)| (label.clone(), convert_cell(cell)))
                .collect::<Map<_, _>>()
        })
        .map(Value::Object)
        .collect();
    Value::Array(records)
}

fn convert_cell(cell: &Data) -> Value {
    match cell {
        Data::Float(value) if value.fract() == 0.0 => Value::from(*value as i64),
        Data::Float(value) => Value::from(*value),
        Data::Int(value) => Value::from(*value),
        Data::String(value) => Value::from(value.as_str()),
        Data::Bool(value) => Value::from(*value),
        _ => Value::Null,
    }
}

pub fn write_test_data(
    path: &str,
    rows: &[Vec<Option<SheetValue>>],
    sheet_number: usize,
    sheet_name: Option<&str>,
    format: &str,
) {
    let sheets: Vec<SheetData> = (0..sheet_number)
        .map(|index| SheetData {
            name: sheet_name.map_or_else(|| format!("Sheet{}", index + 1), String::from),
            rows: rows.to_vec(),
        })
        .collect();
    let buffer = match format {
        "ods" => write_ods_buffer(&sheets).unwrap(),
        _ => write_xlsx_buffer(&sheets).unwrap(),
    };
    std::fs::write(path, buffer).unwrap();
}

/// Builds sheet rows from JSON values.
pub fn create_rows(value: Value) -> Vec<Vec<Option<SheetValue>>> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            row.as_array()
                .unwrap()
                .iter()
                .map(|cell| match cell {
                    Value::Null => None,
                    Value::Bool(value) => Some(SheetValue::Boolean(*value)),
                    Value::Number(value) => Some(SheetValue::Number(value.as_f64().unwrap())),
                    Value::String(value) => Some(SheetValue::String(value.clone())),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}
