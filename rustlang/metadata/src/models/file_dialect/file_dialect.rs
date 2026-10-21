use serde::de::{DeserializeOwned, Error as _};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::arrow::ArrowFileDialect;
use super::csv::CsvFileDialect;
use super::json::JsonFileDialect;
use super::jsonl::JsonlFileDialect;
use super::ods::OdsFileDialect;
use super::parquet::ParquetFileDialect;
use super::sqlite::SqliteFileDialect;
use super::tsv::TsvFileDialect;
use super::unknown::UnknownFileDialect;
use super::xlsx::XlsxFileDialect;

/// Selected by `format`; a missing or unrecognised format is `Unknown`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum FileDialect {
    Csv(CsvFileDialect),
    Tsv(TsvFileDialect),
    Json(JsonFileDialect),
    Jsonl(JsonlFileDialect),
    Xlsx(XlsxFileDialect),
    Ods(OdsFileDialect),
    Sqlite(SqliteFileDialect),
    Parquet(ParquetFileDialect),
    Arrow(ArrowFileDialect),
    Unknown(UnknownFileDialect),
}

impl FileDialect {
    pub fn format(&self) -> Option<&str> {
        match self {
            FileDialect::Csv(_) => Some("csv"),
            FileDialect::Tsv(_) => Some("tsv"),
            FileDialect::Json(_) => Some("json"),
            FileDialect::Jsonl(_) => Some("jsonl"),
            FileDialect::Xlsx(_) => Some("xlsx"),
            FileDialect::Ods(_) => Some("ods"),
            FileDialect::Sqlite(_) => Some("sqlite"),
            FileDialect::Parquet(_) => Some("parquet"),
            FileDialect::Arrow(_) => Some("arrow"),
            FileDialect::Unknown(dialect) => dialect.format.as_deref(),
        }
    }
}

impl<'de> Deserialize<'de> for FileDialect {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let Value::Object(object) = &value else {
            return Err(D::Error::custom("expected a file dialect object"));
        };
        match object.get("format").and_then(Value::as_str) {
            Some("csv") => parse_variant(value, FileDialect::Csv),
            Some("tsv") => parse_variant(value, FileDialect::Tsv),
            Some("json") => parse_variant(value, FileDialect::Json),
            Some("jsonl") => parse_variant(value, FileDialect::Jsonl),
            Some("xlsx") => parse_variant(value, FileDialect::Xlsx),
            Some("ods") => parse_variant(value, FileDialect::Ods),
            Some("sqlite") => parse_variant(value, FileDialect::Sqlite),
            Some("parquet") => parse_variant(value, FileDialect::Parquet),
            Some("arrow") => parse_variant(value, FileDialect::Arrow),
            _ => parse_variant(value, FileDialect::Unknown),
        }
    }
}

fn parse_variant<T: DeserializeOwned, E: serde::de::Error>(
    value: Value,
    variant: fn(T) -> FileDialect,
) -> Result<FileDialect, E> {
    serde_json::from_value(value)
        .map(variant)
        .map_err(E::custom)
}
