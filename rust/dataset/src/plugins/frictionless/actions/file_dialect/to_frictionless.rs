use fairspec_metadata::Descriptor;
use serde_json::Value;

const BUILTIN_FORMATS: [&str; 9] = [
    "csv", "tsv", "json", "jsonl", "xlsx", "ods", "parquet", "arrow", "sqlite",
];

/// Convert a Fairspec File Dialect to a Frictionless resource format and Table Dialect
pub fn convert_file_dialect_to_frictionless(
    file_dialect: &Descriptor,
) -> Option<(String, Descriptor)> {
    let format = file_dialect.get("format").and_then(Value::as_str)?;
    if !BUILTIN_FORMATS.contains(&format) {
        return None;
    }

    let mut dialect = Descriptor::new();
    for (key, value) in file_dialect {
        match key.as_str() {
            "delimiter" | "lineTerminator" | "quoteChar" | "nullSequence" | "headerJoin"
            | "commentRows" | "sheetName" | "sheetNumber" => {
                dialect.insert(key.clone(), value.clone());
            }
            "headerRows" => {
                if *value == Value::Bool(false) {
                    dialect.insert("header".to_string(), Value::Bool(false));
                } else {
                    dialect.insert("headerRows".to_string(), value.clone());
                }
            }
            "commentPrefix" => {
                dialect.insert("commentChar".to_string(), value.clone());
            }
            "jsonPointer" => {
                if let Some(pointer) = value.as_str().filter(|pointer| !pointer.is_empty()) {
                    let property = pointer
                        .chars()
                        .skip(1)
                        .collect::<String>()
                        .replace("~1", "/")
                        .replace("~0", "~");
                    dialect.insert("property".to_string(), property.into());
                }
            }
            "rowType" => {
                dialect.insert("itemType".to_string(), value.clone());
            }
            "columnNames" => {
                dialect.insert("itemKeys".to_string(), value.clone());
            }
            "tableName" => {
                dialect.insert("table".to_string(), value.clone());
            }
            _ => {}
        }
    }

    Some((format.to_string(), dialect))
}
