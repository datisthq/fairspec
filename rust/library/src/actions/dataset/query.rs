use std::collections::HashSet;

use fairspec_metadata::{Dataset, Error, Result, assert_dataset, infer_resource_name};
use fairspec_table::{LoadTableOptions, Table, query_tables};

use super::load::load_dataset;
use crate::actions::table::load::load_table;
use crate::models::dataset::DatasetSource;

/// Runs a SQL query across a dataset, each table resource exposed under its name.
/// Only the resources the query names are loaded.
pub fn query_dataset(
    source: impl Into<DatasetSource>,
    query: &str,
    options: &LoadTableOptions,
) -> Result<Table> {
    let dataset = match source.into() {
        DatasetSource::Path(path) => {
            let Some(descriptor) = load_dataset(&path)? else {
                return Err(create_error(format!(
                    "No plugin can load the dataset: {path}"
                )));
            };
            serde_json::from_value::<Dataset>(serde_json::Value::Object(descriptor))
                .map_err(|error| Error::Parse(error.to_string()))?
        }
        DatasetSource::Descriptor(descriptor) => assert_dataset(&descriptor, None)?,
        DatasetSource::Dataset(dataset) => *dataset,
    };

    let words = get_query_words(query);
    let resources: Vec<_> = dataset
        .resources
        .iter()
        .flatten()
        .enumerate()
        .map(|(index, resource)| {
            let name = resource
                .name
                .clone()
                .unwrap_or_else(|| infer_resource_name(resource, Some(index + 1)));
            (resource, name)
        })
        .filter(|(_, name)| words.contains(&name.to_lowercase()))
        .collect();

    let mut tables = Vec::new();
    for (resource, name) in &resources {
        if resources.iter().filter(|(_, other)| other == name).count() > 1 {
            return Err(create_error(format!(
                "More than one resource is named \"{name}\""
            )));
        }
        // A word in the query can be a column or a literal, so a non-table is skipped
        if let Some(table) = load_table(resource, options)? {
            tables.push((name.clone(), table));
        }
    }

    query_tables(tables, query)
}

// Python's `re.findall(r"[a-z0-9_]+", query.lower())`
fn get_query_words(query: &str) -> HashSet<String> {
    query
        .to_lowercase()
        .split(|char: char| !(char.is_ascii_lowercase() || char.is_ascii_digit() || char == '_'))
        .filter(|word| !word.is_empty())
        .map(str::to_string)
        .collect()
}

fn create_error(message: String) -> Error {
    Error::Invalid {
        message,
        report: None,
    }
}
