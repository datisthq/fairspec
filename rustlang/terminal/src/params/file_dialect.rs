use clap::Args;
use fairspec_library::fairspec_metadata::{HeaderRows, RowType};
use serde::Deserialize;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
#[command(next_help_heading = "Dialect")]
pub struct DialectParams {
    /// path to a Dialect descriptor
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub dialect: Option<String>,
    /// format type (csv, json, xlsx, etc)
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub format: Option<String>,
    /// character used to separate fields in the data
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub delimiter: Option<String>,
    /// character sequence used to terminate rows
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub line_terminator: Option<String>,
    /// character used to quote fields
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub quote_char: Option<String>,
    /// character sequence representing null or missing values
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub null_sequence: Option<String>,
    /// comma-separated row numbers (1-indexed) for headers, or 'false' to disable
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_header_rows)]
    #[serde(default)]
    pub header_rows: Option<HeaderRows>,
    /// character used to join multi-line headers
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub header_join: Option<String>,
    /// comma-separated row numbers (1-indexed) to exclude from data
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_row_numbers)]
    #[serde(default)]
    pub comment_rows: Option<RowNumbers>,
    /// character sequence denoting the start of a comment line
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub comment_prefix: Option<String>,
    /// comma-separated list of column names
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_list)]
    #[serde(default)]
    pub column_names: Option<StringList>,
    /// JSON pointer to the data array within the JSON document
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub json_pointer: Option<String>,
    /// the type of each row in the data
    #[arg(long, value_parser = ["array", "object"])]
    #[serde(default, deserialize_with = "deserialize_row_type")]
    pub row_type: Option<String>,
    /// for spreadsheet data, the sheet number to read (0-indexed)
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub sheet_number: Option<usize>,
    /// for spreadsheet data, the sheet name to read
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub sheet_name: Option<String>,
    /// for database sources, the table name to read
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub table_name: Option<String>,
}

/// A comma-separated CLI list; a newtype so clap parses it as one value.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct StringList(pub Vec<String>);

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct RowNumbers(pub Vec<usize>);

pub fn parse_header_rows(value: &str) -> Result<HeaderRows, String> {
    if value == "false" {
        return Ok(HeaderRows::Disabled);
    }
    parse_row_numbers(value).map(|rows| HeaderRows::Rows(rows.0))
}

pub fn parse_row_numbers(value: &str) -> Result<RowNumbers, String> {
    value
        .split(',')
        .map(|item| {
            item.trim()
                .parse::<usize>()
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()
        .map(RowNumbers)
}

pub fn parse_list(value: &str) -> Result<StringList, String> {
    Ok(StringList(value.split(',').map(str::to_string).collect()))
}

fn deserialize_row_type<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let row_type = Option::<RowType>::deserialize(deserializer)?;
    Ok(row_type.map(|row_type| match row_type {
        RowType::Array => "array".to_string(),
        RowType::Object => "object".to_string(),
    }))
}
