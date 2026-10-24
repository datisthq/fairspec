use fairspec_library::fairspec_dataset::InferFileDialectOptions;
use fairspec_library::fairspec_metadata::Resource;
use fairspec_library::infer_file_dialect;

use crate::helpers::console::Console;
use crate::helpers::file::select_file;
use crate::helpers::resource::create_value_error;
use crate::models::output::{Descriptor, Exit, create_descriptor};
use crate::params::common::SelectParams;
use crate::session::Session;
#[fireargs::input]
pub struct InferFileDialectInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: Option<String>,
    #[fireargs(flatten)]
    pub select: SelectParams,
    /// sample size in bytes
    pub sample_bytes: Option<usize>,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn infer_file_dialect_command(
    console: &Console,
) -> fireargs::Command<InferFileDialectInput, Descriptor> {
    let console = console.clone();
    fireargs::command()
        .description("Infer the dialect of a file")
        .handler(
            async move |input: InferFileDialectInput, context: fireargs::Context| {
                Ok(run_infer_file_dialect(&input, context.json, &console)?)
            },
        )
}

fn run_infer_file_dialect(
    input: &InferFileDialectInput,
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

    Ok(session.render_data_result(create_descriptor(&file_dialect)))
}
