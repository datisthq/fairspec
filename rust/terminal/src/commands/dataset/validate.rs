use fairspec_library::{ValidateTableOptions, validate_dataset};

use crate::helpers::console::Console;
use crate::models::output::{Exit, ReportResult};
use crate::session::Session;

#[fireargs::input]
pub struct ValidateDatasetInput {
    /// local or remote path to the descriptor
    #[fireargs(argument)]
    pub path: String,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn validate_dataset_command(
    console: &Console,
) -> fireargs::Command<ValidateDatasetInput, ReportResult> {
    let console = console.clone();
    fireargs::command()
        .description("Validate a dataset from a local or remote path")
        .handler(
            async move |input: ValidateDatasetInput, context: fireargs::Context| {
                Ok(run_validate_dataset(&input, context.json, &console)?)
            },
        )
}

fn run_validate_dataset(
    input: &ValidateDatasetInput,
    json: bool,
    console: &Console,
) -> Result<ReportResult, Exit> {
    let path = input.path.as_str();
    let session = Session::new(console, false, input.debug, json);
    let report = session.task("Validating dataset", || {
        validate_dataset(path.to_string(), &ValidateTableOptions::default())
    })?;
    session.render_report_result(&report)
}
