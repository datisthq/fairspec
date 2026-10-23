use clap::Args;
use fairspec_library::fairspec_metadata::Resource;
use fairspec_library::infer_data_schema;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

use crate::helpers::input::require_value;
use crate::helpers::resource::create_value_error;
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InferDataSchemaInput {
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

pub fn infer_data_schema_command(
    input: &InferDataSchemaInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let session = Session::new(console, input.silent, input.debug, json);

    let data_schema = session.task("Inferring data schema", || {
        let resource = Resource {
            data: Some(serde_json::Value::String(path.to_string())),
            ..Resource::default()
        };
        infer_data_schema(&resource)?
            .ok_or_else(|| create_value_error("Could not infer data schema"))
    })?;

    Ok(session.render_data_result(serde_json::Value::Object(data_schema)))
}
