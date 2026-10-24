use fairspec_library::fairspec_dataset::copy_file;

use crate::helpers::console::Console;
use crate::helpers::file::select_file;
use crate::models::output::{Exit, TextResult};
use crate::params::common::SelectParams;
use crate::session::Session;
#[fireargs::input]
pub struct CopyFileInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: Option<String>,
    /// a local output path
    pub to_path: String,
    #[fireargs(flatten)]
    pub select: SelectParams,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn copy_file_command(console: &Console) -> fireargs::Command<CopyFileInput, TextResult> {
    let console = console.clone();
    fireargs::command()
        .description("Copy a local or remote file to a local path")
        .handler(
            async move |input: CopyFileInput, context: fireargs::Context| {
                Ok(run_copy_file(&input, context.json, &console)?)
            },
        )
}

fn run_copy_file(input: &CopyFileInput, json: bool, console: &Console) -> Result<TextResult, Exit> {
    let to_path = input.to_path.as_str();
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
