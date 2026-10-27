use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

use jsonschema::{Draft, Retrieve, Uri, Validator};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::actions::json_schema::load::load_json_schema;
use crate::models::exception::{Error, Result};
use crate::models::json_schema::JsonSchema;
use crate::models::path::PathOr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonIssue {
    pub message: String,
    pub json_pointer: String,
}

pub fn inspect_json(
    value: &Value,
    json_schema: impl Into<PathOr<JsonSchema>>,
    root_json_pointer: Option<&str>,
) -> Result<Vec<JsonIssue>> {
    let json_schema = match json_schema.into() {
        PathOr::Path(path) => load_json_schema(&path, false)?,
        PathOr::Value(json_schema) => json_schema,
    };

    let validator = get_validator(json_schema)?;

    Ok(validator
        .iter_errors(value)
        .map(|error| JsonIssue {
            message: error.to_string(),
            json_pointer: combine_json_pointers(
                root_json_pointer.unwrap_or_default(),
                error.instance_path().as_str(),
            ),
        })
        .collect())
}

/// Callers check a column cell by cell with one schema, so building it once per text pays.
static VALIDATORS: LazyLock<Mutex<HashMap<String, Arc<Validator>>>> =
    LazyLock::new(Default::default);

fn get_validator(json_schema: JsonSchema) -> Result<Arc<Validator>> {
    let json_schema = Value::Object(json_schema);
    let key = json_schema.to_string();
    if let Some(validator) = VALIDATORS
        .lock()
        .ok()
        .and_then(|cache| cache.get(&key).cloned())
    {
        return Ok(validator);
    }

    let validator = Arc::new(
        jsonschema::options()
            .with_draft(Draft::Draft202012)
            .should_validate_formats(false)
            .with_retriever(RemoteRetriever)
            .build(&json_schema)
            .map_err(|error| Error::Invalid {
                message: format!("JsonSchema is not valid: {error}"),
                report: None,
            })?,
    );
    if let Ok(mut cache) = VALIDATORS.lock() {
        if cache.len() >= 128 {
            cache.clear();
        }
        cache.insert(key, validator.clone());
    }
    Ok(validator)
}

pub(crate) fn combine_json_pointers(root: &str, instance: &str) -> String {
    if root == "/" {
        return instance.to_string();
    }
    format!("{root}{instance}")
}

struct RemoteRetriever;

impl Retrieve for RemoteRetriever {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> std::result::Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let json_schema = load_json_schema(uri.as_str(), true)?;
        Ok(Value::Object(json_schema))
    }
}
