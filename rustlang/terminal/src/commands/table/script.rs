use clap::Args;
use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::load_table;
use serde::Deserialize;

use super::resource::create_table_resource;
use crate::commands::general::script::run_sql_session;
use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::{LlmsParams, SelectParams};
use crate::params::file_dialect::DialectParams;
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTableInput {
    /// local or remote path to the table
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    #[command(flatten)]
    #[serde(flatten)]
    pub select: SelectParams,
    /// Enable debug mode to print exception details to stderr
    #[arg(long)]
    #[serde(default)]
    pub debug: bool,
    #[command(flatten)]
    #[serde(skip)]
    pub output: LlmsParams,
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

pub fn script_table_command(
    input: &ScriptTableInput,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let session = Session::new(console, false, input.debug, false);
    let resource = create_table_resource(
        &session,
        input.path.as_deref(),
        &input.select,
        &input.dialect,
        input.table_schema.as_deref(),
    )?;

    let load_options = LoadTableOptions {
        infer_table_schema_options: create_infer_table_schema_options(&input.table_schema_params),
        denormalized: Some(true),
        ..LoadTableOptions::default()
    };
    let table = session.task("Loading table", || {
        load_table(&resource, &load_options)?
            .ok_or_else(|| create_value_error("Could not load table"))
    })?;

    session.render_text(
        "[dim]Each line runs as SQL against `self`, the table; an empty line exits[/dim]",
        Some("warning"),
    );
    run_sql_session(console, |query| {
        fairspec_library::fairspec_table::query_table(table.clone(), query)
    });

    Ok(CommandOutput::new(serde_json::json!({})))
}
