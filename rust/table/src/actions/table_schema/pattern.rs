use fairspec_metadata::{FairspecError, MetadataError, TableSchema};

/// Report each column pattern the table engine cannot compile, such as one using look-around
pub fn inspect_table_schema_patterns(table_schema: &TableSchema) -> Vec<FairspecError> {
    let mut errors = Vec::new();

    for (name, property) in table_schema.properties.iter().flatten() {
        let Ok(property) = serde_json::to_value(property) else {
            continue;
        };
        let Some(pattern) = property.get("pattern").and_then(|value| value.as_str()) else {
            continue;
        };

        // polars compiles str.contains patterns with this same regex crate
        if let Err(error) = regex::Regex::new(pattern) {
            let reason = error.to_string();
            let reason = reason.trim().lines().last().unwrap_or_default();
            let pointer = name.replace('~', "~0").replace('/', "~1");
            errors.push(FairspecError::Metadata(MetadataError {
                message: format!("pattern \"{pattern}\" cannot be compiled: {reason}"),
                json_pointer: format!("/properties/{pointer}/pattern"),
                ..Default::default()
            }));
        }
    }

    errors
}
