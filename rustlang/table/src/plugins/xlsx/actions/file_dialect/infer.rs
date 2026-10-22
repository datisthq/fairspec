use fairspec_metadata::{
    FileDialect, Resource, Result, get_data_first_path, get_supported_file_dialect,
};
use serde_json::{Value, json};

use crate::models::table::LoadTableOptions;
use crate::plugins::xlsx::helpers::sheet::read_sheet_rows;
use crate::plugins::xlsx::helpers::table::format_sheet_cell;
use crate::utils::sniffer::sniffer::Sniffer;

pub fn infer_xlsx_file_dialect(
    resource: &Resource,
    _options: Option<&LoadTableOptions>,
) -> Result<Option<FileDialect>> {
    let Some(data_path) = get_data_first_path(resource) else {
        return Ok(None);
    };
    let file_dialect = get_supported_file_dialect(resource, &["xlsx", "ods"])?;
    let Some(file_dialect) = file_dialect
        .filter(|dialect| matches!(dialect, FileDialect::Xlsx(_) | FileDialect::Ods(_)))
    else {
        return Ok(None);
    };
    let format = file_dialect.format().unwrap_or("xlsx");

    let rows = match read_sheet_rows(&data_path, &file_dialect, Some(100)) {
        Ok(Some(rows)) if !rows.is_empty() => rows,
        _ => return create_dialect(format, None),
    };

    let rows: Vec<Vec<Value>> = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| format_sheet_cell(cell.as_ref()).map_or(Value::Null, Value::String))
                .collect()
        })
        .collect();
    let Ok(detection) = Sniffer::new().sniff_rows(&rows) else {
        return create_dialect(format, None);
    };

    let header_rows = if detection.dialect.header.has_header_row {
        Some(json!([detection.dialect.header.num_preamble_rows + 1]))
    } else if detection.num_fields > 0 {
        Some(json!(false))
    } else {
        None
    };
    create_dialect(format, header_rows)
}

fn create_dialect(format: &str, header_rows: Option<Value>) -> Result<Option<FileDialect>> {
    let mut dialect = json!({"format": if format == "ods" { "ods" } else { "xlsx" }});
    if let Some(header_rows) = header_rows {
        dialect["headerRows"] = header_rows;
    }
    Ok(serde_json::from_value(dialect).ok())
}
