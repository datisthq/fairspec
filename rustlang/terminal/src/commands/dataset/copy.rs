use clap::Args;
use fairspec_library::fairspec_dataset::SaveDatasetOptions;
use fairspec_library::fairspec_metadata::Dataset;
use fairspec_library::{load_dataset, save_dataset};
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::helpers::input::require_value;
use crate::helpers::resource::create_value_error;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyDatasetInput {
    /// local or remote path to the descriptor
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// a local output path
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub to_path: Option<String>,
    /// suppress all output except errors
    #[arg(long)]
    #[serde(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[arg(long)]
    #[serde(default)]
    pub debug: bool,
    #[command(flatten)]
    #[serde(skip)]
    pub output: OutputParams,
}

pub fn copy_dataset_command(
    input: &CopyDatasetInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let to_path = require_value(&input.to_path);
    let session = Session::new(console, input.silent, input.debug, json);

    session.task("Copy dataset", || {
        let descriptor =
            load_dataset(path)?.ok_or_else(|| create_value_error("Could not load dataset"))?;
        let dataset: Dataset = serde_json::from_value(serde_json::Value::Object(descriptor))
            .map_err(|error| create_value_error(&error.to_string()))?;
        let options = SaveDatasetOptions {
            target: to_path.to_string(),
            with_remote: false,
        };
        save_dataset(&dataset, &options)?.ok_or_else(|| {
            create_value_error(&format!("Could not copy dataset to \"{to_path}\""))
        })?;
        Ok(())
    })?;

    Ok(session.render_text_result(
        &format!("Copied dataset from [bold]{path}[/bold] to [bold]{to_path}[/bold]"),
        Some("success"),
    ))
}
