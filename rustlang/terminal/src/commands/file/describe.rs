use fairspec_library::fairspec_dataset::describe_file;

use crate::helpers::console::Console;
use crate::helpers::file::select_file;
use crate::models::output::{Descriptor, Exit, create_descriptor};
use crate::params::common::SelectParams;
use crate::params::file::HashType;
use crate::session::Session;
#[fireargs::input]
pub struct DescribeFileInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: Option<String>,
    #[fireargs(flatten)]
    pub select: SelectParams,
    /// hash type
    pub hash_type: HashType,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn describe_file_command(
    console: &Console,
) -> fireargs::Command<DescribeFileInput, Descriptor> {
    let console = console.clone();
    fireargs::command()
        .description("Show stats for a local or remote file")
        .handler(
            async move |input: DescribeFileInput, context: fireargs::Context| {
                Ok(run_describe_file(&input, context.json, &console)?)
            },
        )
}

fn run_describe_file(
    input: &DescribeFileInput,
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

    let hash_type = input.hash_type.into();
    let stats = session.task("Describing file", || describe_file(&path, Some(hash_type)))?;

    Ok(session.render_data_result(create_descriptor(&stats)))
}
