use polars::prelude::Expr;
use serde::{Deserialize, Serialize};

/// A named condition every row must satisfy, beyond what the Table Schema expresses.
/// A condition reducing to one value checks the whole table instead.
#[derive(Debug, Clone)]
pub struct TableCheck {
    pub name: String,
    pub condition: TableCheckCondition,
    pub column_name: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TableCheckCondition {
    Sql(String),
    Expr(Expr),
}

/// A table check whose condition is polars SQL, the form JSON carries.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlTableCheck {
    pub name: String,
    pub condition: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl From<SqlTableCheck> for TableCheck {
    fn from(check: SqlTableCheck) -> Self {
        TableCheck {
            name: check.name,
            condition: TableCheckCondition::Sql(check.condition),
            column_name: check.column_name,
            message: check.message,
        }
    }
}
