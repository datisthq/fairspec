use clap::Args;
use fairspec_library::fairspec_dataset::copy_file;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::helpers::file::select_file;
use crate::helpers::input::require_value;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::{OutputParams, SelectParams};
use crate::session::Session;
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyFileInput {
    /// local or remote path to the file
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// a local output path
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub to_path: Option<String>,
    #[command(flatten)]
    #[serde(flatten)]
    pub select: SelectParams,
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

pub fn copy_file_command(
    input: &CopyFileInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let to_path = require_value(&input.to_path);
    let session = Session::new(console, input.silent, input.debug, json);
    let path = match input.path.clone().filter(|path| !path.is_empty()) {
        Some(path) => path,
        None => select_file(
            &session,
            input.select.dataset.as_deref(),
            input.select.resource.as_deref(),
        )?,
    };

    session.task("Copying file", || copy_file(&path, to_path, None))?;

    Ok(session.render_text_result(
        &format!("Copied file from [bold]{path}[/bold] to [bold]{to_path}[/bold]"),
        Some("success"),
    ))
}
