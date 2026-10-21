use jsonschema::{Draft, Retrieve, Uri};
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

    let validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(false)
        .with_retriever(RemoteRetriever)
        .build(&Value::Object(json_schema))
        .map_err(|error| Error::Invalid {
            message: format!("JsonSchema is not valid: {error}"),
            report: None,
        })?;

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
