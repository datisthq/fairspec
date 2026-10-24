use fairspec_library::fairspec_metadata::validate_table_schema;

use crate::helpers::console::Console;
use crate::models::output::{Exit, ReportResult};
use crate::session::Session;

#[fireargs::input]
pub struct ValidateTableSchemaInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: String,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn validate_table_schema_command(
    console: &Console,
) -> fireargs::Command<ValidateTableSchemaInput, ReportResult> {
    let console = console.clone();
    fireargs::command()
        .description("Validate a Table Schema")
        .handler(
            async move |input: ValidateTableSchemaInput, context: fireargs::Context| {
                Ok(run_validate_table_schema(&input, context.json, &console)?)
            },
        )
}

fn run_validate_table_schema(
    input: &ValidateTableSchemaInput,
    json: bool,
    console: &Console,
) -> Result<ReportResult, Exit> {
    let path = input.path.as_str();
    let session = Session::new(console, input.silent, input.debug, json);
    let result = session.task("Validating table schema", || {
        validate_table_schema(path.to_string(), None)
    })?;
    let report = fairspec_library::fairspec_metadata::Report {
        valid: result.valid,
        errors: result.errors,
    };
    session.render_report_result(&report)
}
