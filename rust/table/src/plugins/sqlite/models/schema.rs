use super::column::SqliteColumn;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SqliteSchema {
    pub name: String,
    pub columns: Vec<SqliteColumn>,
    pub is_view: bool,
    pub primary_key: Option<Vec<String>>,
}
