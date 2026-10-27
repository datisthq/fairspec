use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::load_table;

use super::resource::create_table_resource;
use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{Exit, Records};
use crate::params::common::SelectParams;
use crate::params::file_dialect::DialectParams;
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[fireargs::input]
pub struct PreviewTableInput {
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

pub fn preview_table_command(console: &Console) -> fireargs::Command<PreviewTableInput, Records> {
    let console = console.clone();
    fireargs::command()
        .description("Preview a table from a local or remote path")
        .handler(
            async move |input: PreviewTableInput, context: fireargs::Context| {
                Ok(run_preview_table(&input, context.json, &console)?)
            },
        )
}

fn run_preview_table(
    input: &PreviewTableInput,
    json: bool,
    console: &Console,
) -> Result<Records, Exit> {
    let session = Session::new(console, false, input.debug, json);
    let resource = create_table_resource(
        &session,
        input.path.as_deref(),
        &input.select,
        &input.dialect,
        input.schema.as_deref(),
    )?;

    let load_options = LoadTableOptions {
        infer_table_schema_options: create_infer_table_schema_options(&input.table_schema_params),
        preview_bytes: Some(10_000),
        ..LoadTableOptions::default()
    };
    let table = session.task("Loading table preview", || {
        load_table(&resource, &load_options)?
            .ok_or_else(|| create_value_error("Could not load table"))
    })?;

    let frame = session.task("Collecting data", || {
        table
            .limit(100)
            .collect()
            .map_err(|error| create_value_error(&error.to_string()))
    })?;

    session.render_frame_result(&frame)
}
