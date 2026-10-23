use clap::Args;
use fairspec_library::fairspec_metadata::{Dataset, infer_resource_name};
use fairspec_library::load_dataset;
use serde::Deserialize;
use serde_json::Value;

use crate::helpers::console::Console;
use crate::helpers::input::require_value;
use crate::helpers::resource::create_value_error;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDatasetInput {
    /// local or remote path to the descriptor
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// Enable debug mode to print exception details to stderr
    #[arg(long)]
    #[serde(default)]
    pub debug: bool,
    #[command(flatten)]
    #[serde(skip)]
    pub output: OutputParams,
}

pub fn list_dataset_command(
    input: &ListDatasetInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let session = Session::new(console, false, input.debug, json);

    let dataset = session.task("Loading dataset", || load_dataset_model(path))?;
    let names: Vec<Value> = dataset
        .resources
        .iter()
        .flatten()
        .enumerate()
        .map(|(index, resource)| {
            Value::String(
                resource
                    .name
                    .clone()
                    .unwrap_or_else(|| infer_resource_name(resource, Some(index + 1))),
            )
        })
        .collect();

    Ok(session.render_data_result(Value::Array(names)))
}

pub fn load_dataset_model(
    path: &str,
) -> Result<Dataset, fairspec_library::fairspec_metadata::Error> {
    let descriptor =
        load_dataset(path)?.ok_or_else(|| create_value_error("Could not load dataset"))?;
    serde_json::from_value(Value::Object(descriptor))
        .map_err(|error| create_value_error(&error.to_string()))
}
