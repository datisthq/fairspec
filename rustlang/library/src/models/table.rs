use fairspec_table::{LoadTableOptions, SqlTableCheck, TableCheck};
use serde::{Deserialize, Deserializer};

/// Options of `validate_table`; JSON carries checks as polars SQL.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTableOptions {
    #[serde(flatten)]
    pub load_table_options: LoadTableOptions,
    #[serde(default)]
    pub no_infer: Option<bool>,
    #[serde(default)]
    pub max_errors: Option<usize>,
    #[serde(default, deserialize_with = "deserialize_checks")]
    pub checks: Option<Vec<TableCheck>>,
}

fn deserialize_checks<'de, D>(deserializer: D) -> Result<Option<Vec<TableCheck>>, D::Error>
where
    D: Deserializer<'de>,
{
    let checks: Option<Vec<SqlTableCheck>> = Option::deserialize(deserializer)?;
    Ok(checks.map(|checks| checks.into_iter().map(TableCheck::from).collect()))
}
