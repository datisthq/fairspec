use fairspec_library::fairspec_dataset::validate_file;
use fairspec_library::fairspec_metadata::{Integrity, Resource};

use crate::helpers::console::Console;
use crate::models::output::{Exit, ReportResult};
use crate::params::file::HashType;
use crate::session::Session;

#[fireargs::input]
pub struct ValidateFileInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: String,
    /// expected file hash calculated with the specified hash type
    pub hash: Option<String>,
    /// hash type
    pub hash_type: HashType,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn validate_file_command(
    console: &Console,
) -> fireargs::Command<ValidateFileInput, ReportResult> {
    let console = console.clone();
    fireargs::command()
        .description("Validate a file from a local or remote path")
        .handler(
            async move |input: ValidateFileInput, context: fireargs::Context| {
                Ok(run_validate_file(&input, context.json, &console)?)
            },
        )
}

fn run_validate_file(
    input: &ValidateFileInput,
    json: bool,
    console: &Console,
) -> Result<ReportResult, Exit> {
    let path = input.path.as_str();
    let session = Session::new(console, input.silent, input.debug, json);
    let integrity = input
        .hash
        .clone()
        .filter(|hash| !hash.is_empty())
        .map(|hash| Integrity {
            extra: Default::default(),
            r#type: input.hash_type.into(),
            hash,
        });

    let report = session.task("Validating file", || {
        validate_file(
            &Resource {
                data: Some(serde_json::Value::String(path.to_string())),
                integrity,
                ..Resource::default()
            },
            None,
        )
    })?;

    session.render_report_result(&report)
}
