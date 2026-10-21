// A one-variant enum stands in for a pydantic `Literal`: it serializes to the
// string and rejects any other value, which is what selects a union variant
macro_rules! define_literal {
    ($name:ident, $variant:ident, $value:literal) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
        )]
        pub enum $name {
            #[default]
            #[serde(rename = $value)]
            $variant,
        }
    };
}

pub mod catalog;
pub mod column;
pub mod data;
pub mod data_schema;
pub mod datacite;
pub mod dataset;
pub mod descriptor;
pub mod error;
pub mod exception;
pub mod file_dialect;
pub mod foreign_key;
pub mod integrity;
pub mod json_schema;
pub mod path;
pub mod profile;
pub mod report;
pub mod resource;
pub mod table_schema;
pub mod unique_key;

#[cfg(test)]
mod _test;
