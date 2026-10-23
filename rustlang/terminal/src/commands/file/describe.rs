use clap::Args;
use fairspec_library::fairspec_dataset::describe_file;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::helpers::file::select_file;
use crate::helpers::input::parse_hash_type;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::{OutputParams, SelectParams};
use crate::session::Session;
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescribeFileInput {
    /// local or remote path to the file
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    #[command(flatten)]
    #[serde(flatten)]
    pub select: SelectParams,
    /// hash type
    #[arg(long, value_parser = ["md5", "sha1", "sha256", "sha512"], default_value = "sha256")]
    #[serde(default)]
    pub hash_type: Option<String>,
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

pub fn describe_file_command(
    input: &DescribeFileInput,
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

    let hash_type = parse_hash_type(input.hash_type.as_deref());
    let stats = session.task("Describing file", || describe_file(&path, Some(hash_type)))?;

    Ok(session.render_data_result(serde_json::to_value(stats).unwrap_or_default()))
}
