use fairspec_metadata::{
    ColumnProperty, Descriptor, Error, Result, TableSchema, TableSchemaMissingValue, ValueOrItem,
    create_column_from_property,
};
use indexmap::IndexMap;
use polars::prelude::*;
use regex::Regex;
use serde_json::{Value, json};

use crate::helpers::error::PolarsResultExt;
use crate::helpers::schema::get_polars_schema;
use crate::models::column::PolarsColumn;
use crate::models::frame::Frame;
use crate::models::schema::InferTableSchemaOptions;
use crate::models::table::Table;

const DEFAULT_MISSING_VALUES: [&str; 5] = ["", "NA", "N/A", "null", "-"];

pub fn infer_table_schema_from_table(
    table: Table,
    options: &InferTableSchemaOptions,
) -> Result<TableSchema> {
    let sample_rows = options.sample_rows.filter(|rows| *rows > 0).unwrap_or(100);
    let sample = table.limit(sample_rows as u32).collect().into_result()?;
    infer_table_schema_from_sample(&sample, options)
}

pub fn infer_table_schema_from_sample(
    sample: &Frame,
    options: &InferTableSchemaOptions,
) -> Result<TableSchema> {
    let schema_options = &options.table_schema_options;
    let confidence = options
        .confidence
        .filter(|value| *value != 0.0)
        .unwrap_or(0.9);
    let column_types = schema_options.column_types.as_ref();
    let column_properties = schema_options.column_properties.clone().unwrap_or_default();
    let keep_strings = options.keep_strings == Some(true);
    let missing_values: Vec<String> = match &schema_options.missing_values {
        Some(values) => values.clone(),
        None => DEFAULT_MISSING_VALUES
            .iter()
            .map(|value| value.to_string())
            .collect(),
    };
    let mut detected_missing_values: Vec<String> = Vec::new();

    let regex_mapping = create_regex_mapping(options);

    let polars_schema = get_polars_schema(sample.schema());
    let column_names: Vec<String> = match &schema_options.column_names {
        Some(names) if !names.is_empty() => names.clone(),
        _ => polars_schema
            .columns
            .iter()
            .map(|column| column.name.clone())
            .collect(),
    };

    for name in column_properties.keys() {
        if !column_names.contains(name) {
            return Err(create_not_found_error(name));
        }
    }

    let height = sample.height();
    let failure_threshold = get_failure_threshold(height, confidence);
    let lazy_sample = sample.clone().lazy();
    let count_rows = |filter: Expr, limit: usize| -> Result<usize> {
        let frame = lazy_sample
            .clone()
            .filter(filter)
            .limit(limit as u32)
            .collect()
            .into_result()?;
        Ok(frame.height())
    };

    let mut properties: IndexMap<String, ColumnProperty> = IndexMap::new();
    for name in &column_names {
        let Some(polars_column) = polars_schema
            .columns
            .iter()
            .find(|column| &column.name == name)
        else {
            return Err(create_not_found_error(name));
        };

        let column_override = column_properties.get(name);
        if let Some(column_override) =
            column_override.filter(|o| o.get("type").is_some_and(|t| !t.is_null()))
        {
            let column = create_column_from_property(name, column_override)?;
            properties.insert(name.clone(), column.property());
            continue;
        }

        let column_type = column_types
            .and_then(|types| types.get(name).cloned())
            .filter(|column_type| !column_type.is_empty())
            .or_else(|| get_polars_column_type(polars_column).map(str::to_string))
            .unwrap_or_else(|| "unknown".to_string());
        let mut is_nullable = false;
        let mut property = create_property_dict(&column_type);
        let is_typed = column_types.is_some_and(|types| types.contains_key(name));

        if !is_typed {
            if column_type == "array" {
                property = if schema_options.array_type.as_deref() == Some("list") {
                    to_descriptor(json!({"type": "string", "format": "list"}))
                } else {
                    to_descriptor(json!({"type": "array"}))
                };
            }

            if column_type == "string" {
                let has_polars_nulls = count_rows(col(name.as_str()).is_null(), 1)? > 0;
                let mut missing_filter = col(name.as_str()).is_null();
                let mut has_missing_values = false;
                for value in &missing_values {
                    let is_missing = col(name.as_str()).eq(lit(value.as_str()));
                    if count_rows(is_missing.clone(), 1)? > 0 {
                        if !detected_missing_values.contains(value) {
                            detected_missing_values.push(value.clone());
                        }
                        has_missing_values = true;
                        missing_filter = missing_filter.or(is_missing);
                    }
                }
                is_nullable = has_polars_nulls || has_missing_values;

                if !keep_strings {
                    let effective_sample = lazy_sample.clone().filter(missing_filter.not());
                    let effective_height =
                        effective_sample.clone().collect().into_result()?.height();
                    if effective_height > 0 {
                        let effective_threshold =
                            get_failure_threshold(effective_height, confidence);
                        for (regex, nameless_column) in &regex_mapping {
                            let failures = effective_sample
                                .clone()
                                .filter(
                                    col(name.as_str())
                                        .str()
                                        .contains(lit(regex.as_str()), true)
                                        .not(),
                                )
                                .limit(effective_threshold as u32)
                                .collect()
                                .into_result()?
                                .height();
                            if failures < effective_threshold {
                                property = nameless_column.clone();
                                break;
                            }
                        }
                    }
                }
            }

            if column_type == "number" {
                let is_fractional = col(name.as_str())
                    .eq(col(name.as_str()).round(0, RoundMode::HalfToEven))
                    .not();
                if count_rows(is_fractional, failure_threshold)? < failure_threshold {
                    property = to_descriptor(json!({"type": "integer"}));
                }
            }

            if column_type != "string" && count_rows(col(name.as_str()).is_null(), 1)? > 0 {
                is_nullable = true;
            }
        }

        if is_nullable && let Some(Value::String(base_type)) = property.get("type").cloned() {
            property.insert("type".to_string(), json!([base_type, "null"]));
        }
        let column = create_column_from_property(name, &property)?;
        let mut property =
            to_descriptor(serde_json::to_value(column.property()).unwrap_or_default());
        let column_type = serde_json::to_value(column.column_type()).unwrap_or_default();
        enhance_property(
            &mut property,
            column_type.as_str().unwrap_or_default(),
            options,
        );
        if let Some(column_override) = column_override {
            property.extend(column_override.clone());
        }
        let column = create_column_from_property(name, &property)?;
        properties.insert(name.clone(), column.property());
    }

    let mut table_schema = TableSchema {
        properties: Some(properties),
        ..TableSchema::default()
    };
    if let Some(values) = &schema_options.missing_values {
        table_schema.missing_values = Some(create_missing_values(values));
    } else if !detected_missing_values.is_empty() {
        table_schema.missing_values = Some(create_missing_values(&detected_missing_values));
    }
    Ok(table_schema)
}

fn create_missing_values(values: &[String]) -> Vec<TableSchemaMissingValue> {
    values
        .iter()
        .map(|value| ValueOrItem::Value(fairspec_metadata::StringOrNumber::String(value.clone())))
        .collect()
}

fn create_not_found_error(name: &str) -> Error {
    Error::Load(format!("Column \"{name}\" not found in the table"))
}

fn get_failure_threshold(height: usize, confidence: f64) -> usize {
    let threshold = height - (height as f64 * confidence).floor() as usize;
    threshold.max(1)
}

fn to_descriptor(value: Value) -> Descriptor {
    match value {
        Value::Object(descriptor) => descriptor,
        _ => Descriptor::new(),
    }
}

fn create_property_dict(column_type: &str) -> Descriptor {
    to_descriptor(match column_type {
        "boolean" | "integer" | "number" | "string" | "array" | "object" => {
            json!({"type": column_type})
        }
        "date" => json!({"type": "string", "format": "date"}),
        "date-time" | "local-date-time" | "time" | "local-time" => {
            json!({"type": "string", "format": column_type})
        }
        _ => json!({}),
    })
}

fn get_polars_column_type(polars_column: &PolarsColumn) -> Option<&'static str> {
    Some(match &polars_column.r#type {
        DataType::Datetime(_, _) => {
            return Some(if polars_column.time_zone.is_some() {
                "date-time"
            } else {
                "local-date-time"
            });
        }
        DataType::Boolean => "boolean",
        DataType::Categorical(_, _) | DataType::String => "string",
        DataType::Date => "date",
        DataType::Decimal(_, _) | DataType::Float32 | DataType::Float64 => "number",
        DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64
        | DataType::UInt8
        | DataType::UInt16
        | DataType::UInt32
        | DataType::UInt64 => "integer",
        DataType::List(_) => "array",
        DataType::Null => "unknown",
        DataType::Struct(_) => "object",
        DataType::Time => "local-time",
        _ => return None,
    })
}

fn get_month_first(format: Option<&str>) -> Option<bool> {
    let format = format.filter(|format| !format.is_empty())?;
    let month_index = format.find("%m")?;
    let day_index = format.find("%d")?;
    Some(month_index < day_index)
}

fn create_regex_mapping(options: &InferTableSchemaOptions) -> Vec<(String, Descriptor)> {
    let schema_options = &options.table_schema_options;
    let decimal_char = schema_options.decimal_char.as_deref();
    let group_char = schema_options.group_char.as_deref();
    let comma_decimal = options
        .comma_decimal
        .unwrap_or(decimal_char == Some(",") || group_char == Some("."));
    let date_format = schema_options.date_format.as_deref();
    let time_format = schema_options.time_format.as_deref();
    let datetime_format = schema_options.datetime_format.as_deref();
    let month_first = options
        .month_first
        .or_else(|| get_month_first(date_format))
        .or_else(|| get_month_first(datetime_format))
        .unwrap_or(false);

    let true_values = get_values(&schema_options.true_values, &["true", "True", "TRUE"]);
    let false_values = get_values(&schema_options.false_values, &["false", "False", "FALSE"]);
    let bool_regex = format!(
        "^({})$",
        true_values
            .iter()
            .chain(&false_values)
            .map(|value| regex::escape(value))
            .collect::<Vec<_>>()
            .join("|")
    );
    let list_escaped = regex::escape(schema_options.list_delimiter.as_deref().unwrap_or(","));

    let entry =
        |column_type: &str, property: Value| json!({"type": column_type, "property": property});
    let pick =
        |condition: bool, then: Value, otherwise: Value| if condition { then } else { otherwise };
    let date = |format: &str| {
        entry(
            "date",
            json!({"type": "string", "format": "date", "temporalFormat": format}),
        )
    };
    let local_time = |format: &str| {
        entry(
            "local-time",
            json!({"type": "string", "format": "local-time", "temporalFormat": format}),
        )
    };
    let local_date_time = |format: &str| {
        entry(
            "local-date-time",
            json!({"type": "string", "format": "local-date-time", "temporalFormat": format}),
        )
    };

    let entries: Vec<(String, Value)> = vec![
        (r"^-?(?:0|[1-9]\d*)$".into(), entry("integer", json!({"type": "integer"}))),
        (
            r"^[1-9]\d{0,2}(,\d{3})+$".into(),
            pick(
                comma_decimal,
                entry("number", json!({"type": "number"})),
                entry("integer", json!({"type": "integer", "groupChar": ","})),
            ),
        ),
        (
            r"^-?(?:0|[1-9]\d*)(\.\d+)?$".into(),
            pick(
                comma_decimal,
                entry("integer", json!({"type": "integer", "groupChar": "."})),
                entry("number", json!({"type": "number"})),
            ),
        ),
        (r"^[1-9]\d{0,2}(,\d{3})+(\.\d+)?$".into(), entry("number", json!({"type": "number", "groupChar": ","}))),
        (
            r"^[1-9]\d{0,2}(\.\d{3})+(,\d+)?$".into(),
            entry("number", json!({"type": "number", "groupChar": ".", "decimalChar": ","})),
        ),
        (
            r"^[\p{Sc}\s-]*(?:0|[1-9]\d*)[%\p{Sc}\s]*$".into(),
            entry("integer", json!({"type": "integer", "withText": true})),
        ),
        (
            r"^[\p{Sc}\s-]*[1-9]\d{0,2}(,\d{3})+[%\p{Sc}\s]*$".into(),
            pick(
                comma_decimal,
                entry("number", json!({"type": "number", "withText": true})),
                entry("integer", json!({"type": "integer", "groupChar": ",", "withText": true})),
            ),
        ),
        (
            r"^[\p{Sc}\s-]*(?:0|[1-9]\d*)(\.\d+)?[%\p{Sc}\s]*$".into(),
            pick(
                comma_decimal,
                entry("integer", json!({"type": "integer", "groupChar": ".", "withText": true})),
                entry("number", json!({"type": "number", "withText": true})),
            ),
        ),
        (
            r"^[\p{Sc}\s-]*[1-9]\d{0,2}(,\d{3})+(\.\d+)?[%\p{Sc}\s]*$".into(),
            entry("number", json!({"type": "number", "groupChar": ",", "withText": true})),
        ),
        (
            r"^[\p{Sc}\s-]*[1-9]\d{0,2}(\.\d{3})+(,\d+)?[%\p{Sc}\s]*$".into(),
            entry(
                "number",
                json!({"type": "number", "groupChar": ".", "decimalChar": ",", "withText": true}),
            ),
        ),
        (bool_regex, entry("boolean", json!({"type": "boolean"}))),
        (r"^\d{4}-\d{2}-\d{2}$".into(), entry("date", json!({"type": "string", "format": "date"}))),
        (r"^\d{4}/\d{2}/\d{2}$".into(), date("%Y/%m/%d")),
        (r"^\d{2}/\d{2}/\d{4}$".into(), pick(month_first, date("%m/%d/%Y"), date("%d/%m/%Y"))),
        (r"^\d{2}-\d{2}-\d{4}$".into(), pick(month_first, date("%m-%d-%Y"), date("%d-%m-%Y"))),
        (r"^\d{2}\.\d{2}\.\d{4}$".into(), pick(month_first, date("%m.%d.%Y"), date("%d.%m.%Y"))),
        (
            r"^\d{2}:\d{2}:\d{2}(\.\d+)?$".into(),
            entry("local-time", json!({"type": "string", "format": "local-time"})),
        ),
        (r"^\d{2}:\d{2}$".into(), local_time("%H:%M")),
        (r"^\d{1,2}:\d{2}:\d{2}\s*(am|pm|AM|PM)$".into(), local_time("%I:%M:%S %p")),
        (r"^\d{1,2}:\d{2}\s*(am|pm|AM|PM)$".into(), local_time("%I:%M %p")),
        (
            r"^\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$".into(),
            entry("time", json!({"type": "string", "format": "time"})),
        ),
        (
            r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$".into(),
            entry("date-time", json!({"type": "string", "format": "date-time"})),
        ),
        (
            r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?$".into(),
            entry("local-date-time", json!({"type": "string", "format": "local-date-time"})),
        ),
        (r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$".into(), local_date_time("%Y-%m-%d %H:%M:%S")),
        (
            r"^\d{2}/\d{2}/\d{4} \d{2}:\d{2}$".into(),
            pick(month_first, local_date_time("%m/%d/%Y %H:%M"), local_date_time("%d/%m/%Y %H:%M")),
        ),
        (
            r"^\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}$".into(),
            pick(
                month_first,
                local_date_time("%m/%d/%Y %H:%M:%S"),
                local_date_time("%d/%m/%Y %H:%M:%S"),
            ),
        ),
        (r"^\{".into(), entry("object", json!({"type": "object"}))),
        (r"^\[".into(), entry("array", json!({"type": "array"}))),
        (
            format!(r"^(?:0|[1-9]\d*){list_escaped}(?:0|[1-9]\d*)$"),
            entry("list", json!({"type": "string", "format": "list", "itemType": "integer"})),
        ),
        (
            format!(r"^(?:0|[1-9]\d*)(?:\.\d+)?{list_escaped}(?:0|[1-9]\d*)(?:\.\d+)?$"),
            entry("list", json!({"type": "string", "format": "list", "itemType": "number"})),
        ),
        (r"^https?://\S+$".into(), entry("url", json!({"type": "string", "format": "url"}))),
        (
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$".into(),
            entry("email", json!({"type": "string", "format": "email"})),
        ),
        (
            r"^(POINT|LINESTRING|POLYGON|MULTIPOINT|MULTILINESTRING|MULTIPOLYGON|GEOMETRYCOLLECTION)\s*\(".into(),
            entry("wkt", json!({"type": "string", "format": "wkt"})),
        ),
        (
            r"^P(\d+Y)?(\d+M)?(\d+W)?(\d+D)?(T(\d+H)?(\d+M)?(\d+(\.\d+)?S)?)?$".into(),
            entry("duration", json!({"type": "string", "format": "duration"})),
        ),
        (r"^([0-9a-fA-F]{2}){8,}$".into(), entry("hex", json!({"type": "string", "format": "hex"}))),
    ];

    // A dict in Python: a repeated pattern keeps its first place and its last entry
    let mut mapping: IndexMap<String, Value> = IndexMap::new();
    for (regex, entry) in entries {
        mapping.insert(regex, entry);
    }

    let canonical_format = |column_type: &str| match column_type {
        "date" => Some("%Y-%m-%d"),
        "local-time" => Some("%H:%M:%S"),
        "local-date-time" => Some("%Y-%m-%dT%H:%M:%S"),
        _ => None,
    };
    let user_format = |column_type: &str| match column_type {
        "date" => date_format,
        "time" | "local-time" => time_format,
        "date-time" | "local-date-time" => datetime_format,
        _ => None,
    };
    mapping.retain(|_, entry| {
        let column_type = entry["type"].as_str().unwrap_or_default();
        let Some(user_format) = user_format(column_type) else {
            return true;
        };
        let entry_format = match entry["property"].get("temporalFormat") {
            Some(format) => format.as_str(),
            None => canonical_format(column_type),
        };
        entry_format == Some(user_format)
    });

    mapping
        .into_iter()
        .map(|(regex, entry)| (regex, to_descriptor(entry["property"].clone())))
        .collect()
}

fn get_values(values: &Option<Vec<String>>, defaults: &[&str]) -> Vec<String> {
    match values {
        Some(values) if !values.is_empty() => values.clone(),
        _ => defaults.iter().map(|value| value.to_string()).collect(),
    }
}

fn enhance_property(
    property: &mut Descriptor,
    column_type: &str,
    options: &InferTableSchemaOptions,
) {
    if *options == InferTableSchemaOptions::default() {
        return;
    }
    let schema_options = &options.table_schema_options;
    let mut set = |key: &str, value: Option<Value>| {
        if let Some(value) = value {
            property.insert(key.to_string(), value);
        }
    };
    let to_value = |value: &Option<String>| value.clone().map(Value::String);
    let to_list = |value: &Option<Vec<String>>| value.clone().map(|values| json!(values));

    match column_type {
        "boolean" => {
            set("trueValues", to_list(&schema_options.true_values));
            set("falseValues", to_list(&schema_options.false_values));
        }
        "integer" => set("groupChar", to_value(&schema_options.group_char)),
        "number" => {
            set("decimalChar", to_value(&schema_options.decimal_char));
            set("groupChar", to_value(&schema_options.group_char));
        }
        "date-time" | "local-date-time" => {
            // a native typed column reaches here unfiltered, so a format of the wrong kind is skipped
            if let Some(format) = &schema_options.datetime_format
                && get_has_offset_specifier(format) == (column_type == "date-time")
            {
                set("temporalFormat", Some(Value::String(format.clone())));
            }
        }
        "date" => set("temporalFormat", to_value(&schema_options.date_format)),
        "time" | "local-time" => {
            if let Some(format) = &schema_options.time_format
                && get_has_offset_specifier(format) == (column_type == "time")
            {
                set("temporalFormat", Some(Value::String(format.clone())));
            }
        }
        "list" => {
            set("delimiter", to_value(&schema_options.list_delimiter));
            set("itemType", to_value(&schema_options.list_item_type));
        }
        _ => {}
    }
}

fn get_has_offset_specifier(format: &str) -> bool {
    static PATTERN: std::sync::LazyLock<Regex> =
        std::sync::LazyLock::new(|| Regex::new(r"(^|[^%])(%%)*%:?z").unwrap());
    PATTERN.is_match(format)
}
