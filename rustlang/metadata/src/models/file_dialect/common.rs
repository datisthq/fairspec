use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

pub type Delimiter = String;
pub type LineTerminator = String;
pub type QuoteChar = String;
pub type NullSequence = String;
pub type HeaderJoin = String;
pub type CommentRows = Vec<usize>;
pub type CommentPrefix = String;
pub type ColumnNames = Vec<String>;
pub type JsonPointer = String;
pub type SheetNumber = usize;
pub type SheetName = String;
pub type TableName = String;

/// Python's `Literal[False] | list[int]`: `false` means the data has no header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderRows {
    Disabled,
    Rows(Vec<usize>),
}

impl Serialize for HeaderRows {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            HeaderRows::Disabled => serializer.serialize_bool(false),
            HeaderRows::Rows(rows) => rows.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for HeaderRows {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Value::deserialize(deserializer)? {
            Value::Bool(false) => Ok(HeaderRows::Disabled),
            rows @ Value::Array(_) => serde_json::from_value(rows)
                .map(HeaderRows::Rows)
                .map_err(D::Error::custom),
            other => Err(D::Error::custom(format!(
                "expected false or a list of row numbers, got {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RowType {
    Array,
    Object,
}
