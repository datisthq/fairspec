use fairspec_dataset::load_file;
use fairspec_metadata::{
    FileDialect, Resource, Result, get_data_first_path, get_supported_file_dialect,
};
use serde_json::{Value, json};

use crate::models::table::LoadTableOptions;
use crate::plugins::json::actions::buffer::decode::decode_json_buffer;
use crate::plugins::json::helpers::pointer::format_json_pointer;
use crate::utils::sniffer::sniffer::Sniffer;

pub fn infer_json_file_dialect(
    resource: &Resource,
    _options: Option<&LoadTableOptions>,
) -> Result<Option<FileDialect>> {
    let Some(data_path) = get_data_first_path(resource) else {
        return Ok(None);
    };
    let Some(dialect) = get_supported_file_dialect(resource, &["json", "jsonl"])? else {
        return Ok(None);
    };
    let is_json = dialect.format() == Some("json");

    let max_bytes = if is_json { None } else { Some(10_000) };
    let Ok(buffer) = load_file(&data_path, max_bytes) else {
        return build_dialect(is_json, None, None, None);
    };
    let Ok(parsed) = decode_json_buffer(&buffer, !is_json) else {
        return build_dialect(is_json, None, None, None);
    };

    let mut json_pointer = None;
    let mut data = &parsed;
    if let Value::Object(object) = &parsed
        && let Some((key, value)) = object.iter().find(|(_, value)| value.is_array())
    {
        if is_json {
            json_pointer = Some(format_json_pointer(std::slice::from_ref(key)));
        }
        data = value;
    }

    let Some(first) = data.as_array().and_then(|items| items.first()) else {
        return build_dialect(is_json, json_pointer, None, None);
    };
    let row_type = match first {
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        _ => return build_dialect(is_json, json_pointer, None, None),
    };

    let mut header_rows = None;
    if row_type == "array" {
        let rows: Vec<Vec<Value>> = data
            .as_array()
            .into_iter()
            .flatten()
            .take(100)
            .map(|row| row.as_array().cloned().unwrap_or_default())
            .collect();
        let Ok(detection) = Sniffer::new().sniff_rows(&rows) else {
            return build_dialect(is_json, json_pointer, Some(row_type), None);
        };
        if detection.dialect.header.has_header_row {
            header_rows = Some(json!([detection.dialect.header.num_preamble_rows + 1]));
        } else if detection.num_fields > 0 {
            header_rows = Some(json!(false));
        }
    }

    build_dialect(is_json, json_pointer, Some(row_type), header_rows)
}

fn build_dialect(
    is_json: bool,
    json_pointer: Option<String>,
    row_type: Option<&str>,
    header_rows: Option<Value>,
) -> Result<Option<FileDialect>> {
    let mut dialect = json!({"format": if is_json { "json" } else { "jsonl" }});
    if let Some(json_pointer) = json_pointer.filter(|_| is_json) {
        dialect["jsonPointer"] = json!(json_pointer);
    }
    if let Some(row_type) = row_type {
        dialect["rowType"] = json!(row_type);
    }
    if let Some(header_rows) = header_rows {
        dialect["headerRows"] = header_rows;
    }
    Ok(serde_json::from_value(dialect).ok())
}
