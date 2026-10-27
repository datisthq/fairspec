use std::sync::LazyLock;

use fairspec_metadata::{CellError, Column, DurationColumn, Result};
use regex::Regex;

use crate::actions::column::_shared::helpers::inspect_text_column;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;
use crate::settings::DURATION_REGEX;

pub fn inspect_duration_column(
    column: &DurationColumn,
    table: Table,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    inspect_text_column(
        &Column::Duration(column.clone()),
        table,
        &parse_duration,
        mapping,
    )
}

static DURATION_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&DURATION_REGEX).expect("valid regex"));

fn parse_duration(text: &str) -> bool {
    DURATION_PATTERN.is_match(text)
}
