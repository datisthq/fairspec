use fairspec_library::fairspec_metadata::{PathOr, Resource, load_data_schema};
use fairspec_library::validate_data;

use crate::helpers::console::Console;
use crate::models::output::{Exit, ReportResult};
use crate::session::Session;

#[fireargs::input]
pub struct ValidateDataInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: String,
    /// path to a data schema descriptor (JSON Schema)
    pub schema: String,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn validate_data_command(
    console: &Console,
) -> fireargs::Command<ValidateDataInput, ReportResult> {
    let console = console.clone();
    fireargs::command()
        .description("Validate a JSON Data from a local or remote path")
        .handler(
            async move |input: ValidateDataInput, context: fireargs::Context| {
                Ok(run_validate_data(&input, context.json, &console)?)
            },
        )
}

fn run_validate_data(
    input: &ValidateDataInput,
    json: bool,
    console: &Console,
) -> Result<ReportResult, Exit> {
    let path = input.path.as_str();
    let data_schema_path = input.schema.as_str();
    let session = Session::new(console, input.silent, input.debug, json);

    let data_schema = session.task("Loading data schema", || load_data_schema(data_schema_path))?;
    let report = session.task("Validating data", || {
        validate_data(&Resource {
            data: Some(serde_json::Value::String(path.to_string())),
            data_schema: Some(PathOr::Value(data_schema)),
            ..Resource::default()
        })
    })?;

    session.render_report_result(&report)
}
