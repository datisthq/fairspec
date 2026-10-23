use clap::Args;
use fairspec_library::fairspec_metadata::{PathOr, Resource, load_data_schema};
use fairspec_library::validate_data;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

use crate::helpers::input::require_value;
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDataInput {
    /// local or remote path to the file
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// path to a data schema descriptor (JSON Schema)
    #[arg(long = "schema", value_name = "value", allow_hyphen_values = true)]
    #[serde(default, rename = "schema")]
    pub data_schema: Option<String>,
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

pub fn validate_data_command(
    input: &ValidateDataInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let data_schema_path = require_value(&input.data_schema);
    let session = Session::new(console, input.silent, input.debug, json);

    let data_schema = session.task("Loading data schema", || load_data_schema(data_schema_path))?;
    let report = session.task("Validating data", || {
        validate_data(&Resource {
            data: Some(serde_json::Value::String(path.to_string())),
            data_schema: Some(PathOr::Value(data_schema)),
            ..Resource::default()
        })
    })?;

    Ok(session.render_report_result(&report))
}
