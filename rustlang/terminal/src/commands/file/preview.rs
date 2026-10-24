use fairspec_library::fairspec_dataset::preview_file;

use crate::helpers::console::Console;
use crate::helpers::file::select_file;
use crate::models::output::{Descriptor, Exit, create_descriptor};
use crate::params::common::SelectParams;
use crate::session::Session;
#[fireargs::input]
pub struct PreviewFileInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: Option<String>,
    #[fireargs(flatten)]
    pub select: SelectParams,
    /// maximum number of lines
    pub lines: Option<usize>,
    /// sample size in bytes
    pub sample_bytes: Option<usize>,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn preview_file_command(console: &Console) -> fireargs::Command<PreviewFileInput, Descriptor> {
    let console = console.clone();
    fireargs::command()
        .description("Show the raw source of a local or remote file")
        .handler(
            async move |input: PreviewFileInput, context: fireargs::Context| {
                Ok(run_preview_file(&input, context.json, &console)?)
            },
        )
}

fn run_preview_file(
    input: &PreviewFileInput,
    json: bool,
    console: &Console,
) -> Result<Descriptor, Exit> {
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

    Ok(create_descriptor(&preview))
}
