use clap::Args;
use fairspec_library::{ValidateTableOptions, validate_dataset};
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::helpers::input::require_value;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDatasetInput {
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

pub fn validate_dataset_command(
    input: &ValidateDatasetInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let session = Session::new(console, false, input.debug, json);
    let report = session.task("Validating dataset", || {
        validate_dataset(path.to_string(), &ValidateTableOptions::default())
    })?;
    Ok(session.render_report_result(&report))
}
