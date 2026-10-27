use super::common::parse_list;

#[fireargs::input]
#[derive(Debug, Clone, Default)]
pub struct DialectParams {
    /// path to a Dialect descriptor, Fairspec or Frictionless
    #[fireargs(help_group = "Dialect")]
    pub dialect: Option<String>,
    /// format type (csv, json, xlsx, etc)
    #[fireargs(help_group = "Dialect")]
    pub format: Option<String>,
    /// character used to separate fields in the data
    #[fireargs(help_group = "Dialect")]
    pub delimiter: Option<String>,
    /// character sequence used to terminate rows
    #[fireargs(help_group = "Dialect")]
    pub line_terminator: Option<String>,
    /// character used to quote fields
    #[fireargs(help_group = "Dialect")]
    pub quote_char: Option<String>,
    /// character sequence representing null or missing values
    #[fireargs(help_group = "Dialect")]
    pub null_sequence: Option<String>,
    /// comma-separated row numbers (1-indexed) for headers, or 'false' to disable
    #[fireargs(help_group = "Dialect", parse = parse_header_rows)]
    pub header_rows: Option<HeaderRows>,
    /// character used to join multi-line headers
    #[fireargs(help_group = "Dialect")]
    pub header_join: Option<String>,
    /// comma-separated row numbers (1-indexed) to exclude from data
    #[fireargs(help_group = "Dialect", parse = parse_row_numbers)]
    pub comment_rows: Option<Vec<usize>>,
    /// character sequence denoting the start of a comment line
    #[fireargs(help_group = "Dialect")]
    pub comment_prefix: Option<String>,
    /// comma-separated list of column names
    #[fireargs(help_group = "Dialect", parse = parse_list)]
    pub column_names: Option<Vec<String>>,
    /// JSON pointer to the data array within the JSON document
    #[fireargs(help_group = "Dialect")]
    pub json_pointer: Option<String>,
    /// the type of each row in the data
    #[fireargs(help_group = "Dialect")]
    pub row_type: Option<RowType>,
    /// for spreadsheet data, the sheet number to read (0-indexed)
    #[fireargs(help_group = "Dialect")]
    pub sheet_number: Option<usize>,
    /// for spreadsheet data, the sheet name to read
    #[fireargs(help_group = "Dialect")]
    pub sheet_name: Option<String>,
    /// for database sources, the table name to read
    #[fireargs(help_group = "Dialect")]
    pub table_name: Option<String>,
}

// Python's `Literal[False] | list[int]`: `false` disables the header
#[fireargs::option]
#[serde(untagged)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderRows {
    Flag(bool),
    Rows(Vec<usize>),
}

#[fireargs::option]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowType {
    Array,
    Object,
}

pub fn parse_header_rows(value: &str) -> Result<HeaderRows, String> {
    if value == "false" {
        return Ok(HeaderRows::Flag(false));
    }
    parse_row_numbers(value).map(HeaderRows::Rows)
}

pub fn parse_row_numbers(value: &str) -> Result<Vec<usize>, String> {
    value
        .split(',')
        .map(|item| {
            item.trim()
                .parse::<usize>()
                .map_err(|error| error.to_string())
        })
        .collect()
}
