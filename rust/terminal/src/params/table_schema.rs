use indexmap::IndexMap;
use serde_json::Value;

use super::common::parse_list;
use crate::models::output::Descriptor;

#[fireargs::input]
#[derive(Debug, Clone, Default)]
pub struct TableSchemaParams {
    /// a list of comma-separated column name:type pairs to use for the schema
    #[fireargs(help_group = "Table Schema", parse = parse_column_types)]
    pub column_types: Option<IndexMap<String, String>>,
    /// a JSON object of column name to column property to use for the schema
    #[fireargs(help_group = "Table Schema", parse = parse_column_properties)]
    pub column_properties: Option<IndexMap<String, Descriptor>>,
    /// comma-separated values to treat as missing
    #[fireargs(help_group = "Table Schema", parse = parse_list)]
    pub missing_values: Option<Vec<String>>,
    /// character to use as decimal separator
    #[fireargs(help_group = "Table Schema")]
    pub decimal_char: Option<String>,
    /// character to use for digit grouping
    #[fireargs(help_group = "Table Schema")]
    pub group_char: Option<String>,
    /// values to treat as true
    #[fireargs(help_group = "Table Schema", parse = parse_list)]
    pub true_values: Option<Vec<String>>,
    /// values to treat as false
    #[fireargs(help_group = "Table Schema", parse = parse_list)]
    pub false_values: Option<Vec<String>>,
    /// datetime format pattern
    #[fireargs(help_group = "Table Schema")]
    pub datetime_format: Option<String>,
    /// date format pattern
    #[fireargs(help_group = "Table Schema")]
    pub date_format: Option<String>,
    /// time format pattern
    #[fireargs(help_group = "Table Schema")]
    pub time_format: Option<String>,
    /// array type (array or list)
    #[fireargs(help_group = "Table Schema")]
    pub array_type: Option<ArrayType>,
    /// delimiter for list values
    #[fireargs(help_group = "Table Schema")]
    pub list_delimiter: Option<String>,
    /// type of items in lists
    #[fireargs(help_group = "Table Schema")]
    pub list_item_type: Option<ListItemType>,
    /// number of rows to sample for schema inference
    #[fireargs(help_group = "Table Schema")]
    pub sample_rows: Option<usize>,
    /// confidence threshold for schema inference
    #[fireargs(help_group = "Table Schema")]
    pub confidence: Option<f64>,
    /// use comma as decimal separator in schema inference
    #[fireargs(help_group = "Table Schema", default)]
    pub comma_decimal: bool,
    /// interpret dates as month-first in schema inference
    #[fireargs(help_group = "Table Schema", default)]
    pub month_first: bool,
    /// keep fields as strings instead of inferring types
    #[fireargs(help_group = "Table Schema", default)]
    pub keep_strings: bool,
}

#[fireargs::option]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayType {
    Array,
    List,
}

#[fireargs::option]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListItemType {
    String,
    Number,
    Boolean,
    Date,
    DateTime,
    LocalDateTime,
    Integer,
    Time,
    LocalTime,
}

pub fn parse_column_types(value: &str) -> Result<IndexMap<String, String>, String> {
    let mut result = IndexMap::new();
    for pair in value.split(',') {
        if let Some((name, column_type)) = pair.split_once(':')
            && !name.is_empty()
            && !column_type.is_empty()
        {
            result.insert(name.to_string(), column_type.to_string());
        }
    }
    Ok(result)
}

pub fn parse_column_properties(value: &str) -> Result<IndexMap<String, Descriptor>, String> {
    match serde_json::from_str::<Value>(value).map_err(|error| error.to_string())? {
        Value::Object(object) => {
            let mut result = IndexMap::new();
            for (name, property) in object {
                let Value::Object(property) = property else {
                    return Err("--column-properties must be a JSON object".to_string());
                };
                result.insert(name, property);
            }
            Ok(result)
        }
        _ => Err("--column-properties must be a JSON object".to_string()),
    }
}
