use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::load_table;

use super::resource::create_table_resource;
use crate::commands::general::script::run_sql_session;
use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{Empty, Exit};
use crate::params::common::SelectParams;
use crate::params::file_dialect::DialectParams;
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[fireargs::input]
pub struct ScriptTableInput {
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

pub fn script_table_command(console: &Console) -> fireargs::Command<ScriptTableInput, Empty> {
    let console = console.clone();
    fireargs::command()
        .description("Start a scripting session for a table")
        .json_option(fireargs::FlagConfig::Disabled)
        .handler(async move |input: ScriptTableInput, _: fireargs::Context| {
            Ok(run_script_table(&input, &console)?)
        })
}

fn run_script_table(input: &ScriptTableInput, console: &Console) -> Result<Empty, Exit> {
    let session = Session::new(console, false, input.debug, false);
    let resource = create_table_resource(
        &session,
        input.path.as_deref(),
        &input.select,
        &input.dialect,
        input.schema.as_deref(),
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

    Ok(Empty {})
}
