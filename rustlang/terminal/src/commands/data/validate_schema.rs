use clap::Args;
use fairspec_library::fairspec_metadata::validate_data_schema;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

use crate::helpers::input::require_value;
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDataSchemaInput {
    /// local or remote path to the file
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
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

pub fn validate_data_schema_command(
    input: &ValidateDataSchemaInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let session = Session::new(console, input.silent, input.debug, json);
    let result = session.task("Validating data schema", || {
        validate_data_schema(path.to_string(), None)
    })?;
    let report = fairspec_library::fairspec_metadata::Report {
        valid: result.valid,
        errors: result.errors,
    };
    Ok(session.render_report_result(&report))
}
