use crate::models::descriptor::Descriptor;
use serde_json::Value;

use crate::actions::frictionless::mapping::FrictionlessRecorder;
use crate::actions::frictionless::value::{copy_value, get_is_truthy};

/// Convert a Frictionless Table Dialect to a Fairspec File Dialect descriptor
/// Returns None for a format Fairspec does not read as a table; without a format, a
/// standalone dialect takes the one its keys belong to
pub fn convert_file_dialect_from_frictionless(
    dialect: Option<&Descriptor>,
    format: Option<&str>,
    record: Option<&mut FrictionlessRecorder>,
) -> Option<Descriptor> {
    let format = match (format, dialect) {
        (None, Some(dialect)) => infer_format(dialect),
        _ => get_format(&format.unwrap_or_default().to_lowercase())?,
    };

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
            record_keys(&file_dialect, record);
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

    record_keys(&file_dialect, record);
    Some(file_dialect)
}

const DIALECT_KEYS: [(&str, &str); 13] = [
    ("delimiter", "delimiter"),
    ("lineTerminator", "lineTerminator"),
    ("quoteChar", "quoteChar"),
    ("nullSequence", "nullSequence"),
    ("headerJoin", "headerJoin"),
    ("commentRows", "commentRows"),
    ("commentPrefix", "commentChar"),
    ("jsonPointer", "property"),
    ("rowType", "itemType"),
    ("columnNames", "itemKeys"),
    ("sheetName", "sheetName"),
    ("sheetNumber", "sheetNumber"),
    ("tableName", "table"),
];

fn record_keys(file_dialect: &Descriptor, record: Option<&mut FrictionlessRecorder>) {
    let Some(record) = record else {
        return;
    };
    for key in file_dialect.keys() {
        if key == "headerRows" {
            let source = match file_dialect[key] == Value::Bool(false) {
                true => "/header",
                false => "/headerRows",
            };
            record.record("/headerRows", source);
        } else if let Some((_, target)) = DIALECT_KEYS.iter().find(|(from, _)| from == key) {
            record.record(&format!("/{key}"), &format!("/{target}"));
        }
    }
}

fn infer_format(dialect: &Descriptor) -> &'static str {
    let has = |key: &str| dialect.contains_key(key);
    if has("property")
        || get_is_truthy(dialect.get("itemType"))
        || get_is_truthy(dialect.get("itemKeys"))
    {
        return "json";
    }
    if has("sheetName") || has("sheetNumber") {
        return "xlsx";
    }
    if has("table") {
        return "sqlite";
    }
    "csv"
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
