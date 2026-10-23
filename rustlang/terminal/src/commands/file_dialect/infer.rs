use clap::Args;
use fairspec_library::fairspec_dataset::InferFileDialectOptions;
use fairspec_library::fairspec_metadata::Resource;
use fairspec_library::infer_file_dialect;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::helpers::file::select_file;
use crate::helpers::resource::create_value_error;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::{OutputParams, SelectParams};
use crate::session::Session;
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InferFileDialectInput {
    /// local or remote path to the file
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    #[command(flatten)]
    #[serde(flatten)]
    pub select: SelectParams,
    /// sample size in bytes
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub sample_bytes: Option<usize>,
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

pub fn infer_file_dialect_command(
    input: &InferFileDialectInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let session = Session::new(console, input.silent, input.debug, json);
    let path = match input.path.clone().filter(|path| !path.is_empty()) {
        Some(path) => path,
        None => select_file(
            &session,
            input.select.dataset.as_deref(),
            input.select.resource.as_deref(),
        )?,
    };

    let file_dialect = session.task("Inferring dialect", || {
        let resource = Resource {
            data: Some(serde_json::Value::String(path.clone())),
            ..Resource::default()
        };
        let options = InferFileDialectOptions {
            sample_bytes: input.sample_bytes,
        };
        infer_file_dialect(&resource, &options)?
            .ok_or_else(|| create_value_error("Could not infer dialect"))
    })?;

    Ok(session.render_data_result(serde_json::to_value(file_dialect).unwrap_or_default()))
}
