#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteColumn {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub comment: Option<String>,
    pub is_auto_incrementing: bool,
    pub has_default_value: bool,
}

impl SqliteColumn {
    pub fn new(name: impl Into<String>, data_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data_type: data_type.into(),
            is_nullable: true,
            comment: None,
            is_auto_incrementing: false,
            has_default_value: false,
        }
    }
}
