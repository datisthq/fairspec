use clap::Args;
use fairspec_library::fairspec_dataset::preview_file;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::helpers::file::select_file;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::{OutputParams, SelectParams};
use crate::session::Session;
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewFileInput {
    /// local or remote path to the file
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    #[command(flatten)]
    #[serde(flatten)]
    pub select: SelectParams,
    /// maximum number of lines
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub lines: Option<usize>,
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

pub fn preview_file_command(
    input: &PreviewFileInput,
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

    let preview = session.task("Loading file preview", || {
        preview_file(&path, input.sample_bytes, input.lines)
    })?;
    session.render_raw_result(&preview.text);

    Ok(CommandOutput::new(
        serde_json::to_value(preview).unwrap_or_default(),
    ))
}
