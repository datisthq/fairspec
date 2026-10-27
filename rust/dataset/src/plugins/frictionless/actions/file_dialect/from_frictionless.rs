use fairspec_metadata::Descriptor;
use serde_json::Value;

use crate::plugins::frictionless::helpers::value::{copy_value, get_is_truthy};

/// Convert a Frictionless Table Dialect to a Fairspec File Dialect descriptor
/// Returns None for a format Fairspec does not read as a table
pub fn convert_file_dialect_from_frictionless(
    dialect: Option<&Descriptor>,
    format: Option<&str>,
) -> Option<Descriptor> {
    let format = get_format(&format.unwrap_or_default().to_lowercase())?;

    let mut file_dialect = Descriptor::new();
    file_dialect.insert("format".to_string(), format.into());
    let Some(dialect) = dialect else {
        return Some(file_dialect);
    };

    match format {
        "csv" => {
            for key in ["delimiter", "lineTerminator", "quoteChar", "nullSequence"] {
                copy_value(&mut file_dialect, key, dialect.get(key));
            }
        }
        "tsv" => {
            for key in ["lineTerminator", "nullSequence"] {
                copy_value(&mut file_dialect, key, dialect.get(key));
            }
        }
        "json" => {
            if let Some(property) = dialect.get("property").and_then(Value::as_str) {
                let property = property.replace('~', "~0").replace('/', "~1");
                file_dialect.insert("jsonPointer".to_string(), format!("/{property}").into());
            }
            copy_value(
                &mut file_dialect,
                "rowType",
                get_truthy(dialect, "itemType"),
            );
        }
        "jsonl" => {
            copy_value(
                &mut file_dialect,
                "rowType",
                get_truthy(dialect, "itemType"),
            );
        }
        "xlsx" | "ods" => {
            for key in ["sheetName", "sheetNumber"] {
                copy_value(&mut file_dialect, key, dialect.get(key));
            }
        }
        "sqlite" => {
            copy_value(&mut file_dialect, "tableName", dialect.get("table"));
            return Some(file_dialect);
        }
        _ => return Some(file_dialect),
    }

    if dialect.get("header") == Some(&Value::Bool(false)) {
        file_dialect.insert("headerRows".to_string(), Value::Bool(false));
    } else {
        copy_value(
            &mut file_dialect,
            "headerRows",
            get_truthy(dialect, "headerRows"),
        );
    }
    copy_value(&mut file_dialect, "headerJoin", dialect.get("headerJoin"));
    copy_value(
        &mut file_dialect,
        "commentRows",
        get_truthy(dialect, "commentRows"),
    );
    copy_value(
        &mut file_dialect,
        "commentPrefix",
        dialect.get("commentChar"),
    );
    if format.starts_with("json") {
        copy_value(
            &mut file_dialect,
            "columnNames",
            get_truthy(dialect, "itemKeys"),
        );
    }

    Some(file_dialect)
}

fn get_format(format: &str) -> Option<&'static str> {
    match format {
        "csv" => Some("csv"),
        "tsv" => Some("tsv"),
        "json" => Some("json"),
        "jsonl" | "ndjson" => Some("jsonl"),
        "xlsx" => Some("xlsx"),
        "ods" => Some("ods"),
        "parquet" => Some("parquet"),
        "arrow" | "feather" => Some("arrow"),
        "sqlite" => Some("sqlite"),
        _ => None,
    }
}

fn get_truthy<'a>(dialect: &'a Descriptor, key: &str) -> Option<&'a Value> {
    dialect.get(key).filter(|value| get_is_truthy(Some(value)))
}
