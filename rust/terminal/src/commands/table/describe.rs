use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::load_table;

use super::resource::create_table_resource;
use crate::helpers::console::Console;
use crate::helpers::frame::describe_frame;
use crate::helpers::resource::create_value_error;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{Exit, Records};
use crate::params::common::SelectParams;
use crate::params::file_dialect::DialectParams;
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[fireargs::input]
pub struct DescribeTableInput {
    /// local or remote path to the table
    #[fireargs(argument)]
    pub path: Option<String>,
    #[fireargs(flatten)]
    pub select: SelectParams,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
    #[fireargs(flatten)]
    pub dialect: DialectParams,
    #[fireargs(flatten)]
    pub table_schema_params: TableSchemaParams,
    /// path to a Table Schema descriptor, Fairspec or Frictionless
    #[fireargs(help_group = "Table Schema")]
    pub schema: Option<String>,
}

pub fn describe_table_command(console: &Console) -> fireargs::Command<DescribeTableInput, Records> {
    let console = console.clone();
    fireargs::command()
        .description("Show stats for a table from a local or remote path")
        .handler(
            async move |input: DescribeTableInput, context: fireargs::Context| {
                Ok(run_describe_table(&input, context.json, &console)?)
            },
        )
}

fn run_describe_table(
    input: &DescribeTableInput,
    json: bool,
    console: &Console,
) -> Result<Records, Exit> {
    let session = Session::new(console, false, input.debug, json);
    let options = create_infer_table_schema_options(&input.table_schema_params);
    let resource = create_table_resource(
        &session,
        input.path.as_deref(),
        &input.select,
        &input.dialect,
        input.schema.as_deref(),
    )?;

    let load_options = LoadTableOptions {
        infer_table_schema_options: options,
        ..LoadTableOptions::default()
    };
    let table = session.task("Loading table", || {
        load_table(&resource, &load_options)?
            .ok_or_else(|| create_value_error("Could not load table"))
    })?;

    let stats = session.task("Calculating stats", || {
        let frame = table
            .collect()
            .map_err(|error| create_value_error(&error.to_string()))?;
        describe_frame(&frame)
    })?;

    session.render_frame_result(&stats)
}
