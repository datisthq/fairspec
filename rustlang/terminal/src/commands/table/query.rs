use clap::Args;
use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::fairspec_table::query_table;
use fairspec_library::load_table;
use serde::Deserialize;

use super::resource::create_table_resource;
use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::{OutputParams, SelectParams};
use crate::params::file_dialect::DialectParams;
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTableInput {
    /// local or remote path to the table
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// a SQL query to execute against a table (use `self` to refer to the table)
    #[arg(value_name = "query")]
    #[serde(default)]
    pub query: Option<String>,
    #[command(flatten)]
    #[serde(flatten)]
    pub select: SelectParams,
    /// Enable debug mode to print exception details to stderr
    #[arg(long)]
    #[serde(default)]
    pub debug: bool,
    #[command(flatten)]
    #[serde(skip)]
    pub output: OutputParams,
    #[command(flatten)]
    #[serde(flatten)]
    pub dialect: DialectParams,
    #[command(flatten)]
    #[serde(flatten)]
    pub table_schema_params: TableSchemaParams,
    /// path to a Table Schema descriptor
    #[arg(
        long = "schema",
        value_name = "value",
        allow_hyphen_values = true,
        help_heading = "Table Schema"
    )]
    #[serde(default, rename = "schema")]
    pub table_schema: Option<String>,
}

pub fn query_table_command(
    input: &QueryTableInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let session = Session::new(console, false, input.debug, json);
    let resource = create_table_resource(
        &session,
        input.path.as_deref(),
        &input.select,
        &input.dialect,
        input.table_schema.as_deref(),
    )?;

    let load_options = LoadTableOptions {
        infer_table_schema_options: create_infer_table_schema_options(&input.table_schema_params),
        ..LoadTableOptions::default()
    };
    let mut table = session.task("Loading table", || {
        load_table(&resource, &load_options)?
            .ok_or_else(|| create_value_error("Could not load table"))
    })?;

    if let Some(query) = input.query.as_deref().filter(|query| !query.is_empty()) {
        table = session.task("Executing query", || query_table(table, query))?;
    }

    let frame = session.task("Collecting data", || {
        table
            .collect()
            .map_err(|error| create_value_error(&error.to_string()))
    })?;

    session.render_frame_result(&frame)
}
