use clap::Args;
use fairspec_library::fairspec_metadata::Descriptor;
use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;

use super::file_dialect::{StringList, parse_list};

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
#[command(next_help_heading = "Table Schema")]
pub struct TableSchemaParams {
    /// a list of comma-separated column name:type pairs to use for the schema
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_column_types)]
    #[serde(default)]
    pub column_types: Option<ColumnTypes>,
    /// a JSON object of column name to column property to use for the schema
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_column_properties)]
    #[serde(default)]
    pub column_properties: Option<ColumnProperties>,
    /// comma-separated values to treat as missing
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_list)]
    #[serde(default)]
    pub missing_values: Option<StringList>,
    /// character to use as decimal separator
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub decimal_char: Option<String>,
    /// character to use for digit grouping
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub group_char: Option<String>,
    /// values to treat as true
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_list)]
    #[serde(default)]
    pub true_values: Option<StringList>,
    /// values to treat as false
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_list)]
    #[serde(default)]
    pub false_values: Option<StringList>,
    /// datetime format pattern
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub datetime_format: Option<String>,
    /// date format pattern
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub date_format: Option<String>,
    /// time format pattern
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub time_format: Option<String>,
    /// array type (array or list)
    #[arg(long, value_parser = ["array", "list"])]
    #[serde(default)]
    pub array_type: Option<String>,
    /// delimiter for list values
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub list_delimiter: Option<String>,
    /// type of items in lists
    #[arg(long, value_parser = ["string", "number", "boolean", "date", "date-time", "local-date-time", "integer", "time", "local-time"])]
    #[serde(default)]
    pub list_item_type: Option<String>,
    /// number of rows to sample for schema inference
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub sample_rows: Option<usize>,
    /// confidence threshold for schema inference
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub confidence: Option<f64>,
    /// use comma as decimal separator in schema inference
    #[arg(long)]
    #[serde(default)]
    pub comma_decimal: bool,
    /// interpret dates as month-first in schema inference
    #[arg(long)]
    #[serde(default)]
    pub month_first: bool,
    /// keep fields as strings instead of inferring types
    #[arg(long)]
    #[serde(default)]
    pub keep_strings: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct ColumnTypes(pub IndexMap<String, String>);

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct ColumnProperties(pub IndexMap<String, Descriptor>);

pub fn parse_column_types(value: &str) -> Result<ColumnTypes, String> {
    let mut result = IndexMap::new();
    for pair in value.split(',') {
        if let Some((name, column_type)) = pair.split_once(':')
            && !name.is_empty()
            && !column_type.is_empty()
        {
            result.insert(name.to_string(), column_type.to_string());
        }
    }
    Ok(ColumnTypes(result))
}

pub fn parse_column_properties(value: &str) -> Result<ColumnProperties, String> {
    match serde_json::from_str::<Value>(value).map_err(|error| error.to_string())? {
        Value::Object(object) => {
            let mut result = IndexMap::new();
            for (name, property) in object {
                let Value::Object(property) = property else {
                    return Err("--column-properties must be a JSON object".to_string());
                };
                result.insert(name, property);
            }
            Ok(ColumnProperties(result))
        }
        _ => Err("--column-properties must be a JSON object".to_string()),
    }
}
