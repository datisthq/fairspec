use serde_json::Value;

use crate::models::error::error::FairspecError;
use crate::models::error::metadata::MetadataError;
use crate::models::file_dialect::file_dialect::FileDialect;

/// Checks what the profile cannot: `headerRows` out of ascending order, and a row
/// listed both in `headerRows` and `commentRows`.
pub fn inspect_file_dialect(
    file_dialect: &FileDialect,
    root_json_pointer: Option<&str>,
) -> Vec<FairspecError> {
    let root = match root_json_pointer {
        None | Some("/") => "",
        Some(root) => root,
    };
    let Ok(descriptor) = serde_json::to_value(file_dialect) else {
        return Vec::new();
    };
    let mut errors = Vec::new();

    let header_rows = match descriptor.get("headerRows") {
        Some(Value::Array(rows)) => rows.clone(),
        Some(Value::Bool(false)) => Vec::new(),
        _ => vec![Value::from(1)],
    };
    let is_unordered = header_rows
        .windows(2)
        .any(|pair| pair[1].as_f64() < pair[0].as_f64());
    if is_unordered {
        errors.push(FairspecError::Metadata(MetadataError {
            message: "headerRows must list rows in ascending order".to_string(),
            json_pointer: format!("{root}/headerRows"),
            ..Default::default()
        }));
    }

    let Some(Value::Array(comment_rows)) = descriptor.get("commentRows") else {
        return errors;
    };
    if descriptor.get("rowType").and_then(Value::as_str) == Some("object") {
        return errors;
    }

    let rows: Vec<String> = comment_rows
        .iter()
        .filter(|row| header_rows.contains(row))
        .map(Value::to_string)
        .collect();
    if !rows.is_empty() {
        errors.push(FairspecError::Metadata(MetadataError {
            message: format!(
                "row {} is listed both in headerRows and commentRows",
                rows.join(", ")
            ),
            json_pointer: format!("{root}/commentRows"),
            ..Default::default()
        }));
    }

    errors
}
