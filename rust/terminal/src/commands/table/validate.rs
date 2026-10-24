use fairspec_library::fairspec_metadata::{FairspecError, create_report, resolve_table_schema};
use fairspec_library::fairspec_table::{
    LoadTableOptions, TableCheck, TableCheckCondition, infer_table_schema_from_table, inspect_table,
};
use fairspec_library::load_table;

use super::resource::create_table_resource;
use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{Exit, ReportResult};
use crate::params::common::SelectParams;
use crate::params::file_dialect::DialectParams;
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[fireargs::input]
pub struct ValidateTableInput {
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
    /// path to a Table Schema descriptor
    #[fireargs(help_group = "Table Schema")]
    pub schema: Option<String>,
    /// a named SQL condition every row must satisfy, as name=condition (repeatable)
    #[fireargs(help_group = "Checks")]
    pub check: Option<Vec<String>>,
}

pub fn validate_table_command(
    console: &Console,
) -> fireargs::Command<ValidateTableInput, ReportResult> {
    let console = console.clone();
    fireargs::command()
        .description("Validate a table from a local or remote path")
        .handler(
            async move |input: ValidateTableInput, context: fireargs::Context| {
                Ok(run_validate_table(&input, context.json, &console)?)
            },
        )
}

fn run_validate_table(
    input: &ValidateTableInput,
    json: bool,
    console: &Console,
) -> Result<ReportResult, Exit> {
    for check in input.check.iter().flatten() {
        if !check
            .split_once('=')
            .is_some_and(|(name, _)| !name.is_empty())
        {
            console.write_err(&format!(
                "error: option '--check <value>...' argument '{check}' is invalid. Expected \"name=condition\"\n"
            ));
            return Err(Exit::default());
        }
    }
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
        denormalized: Some(true),
        ..LoadTableOptions::default()
    };
    let table = session.task("Loading table", || {
        load_table(&resource, &load_options)?
            .ok_or_else(|| create_value_error("Could not load table"))
    })?;

    let mut table_schema = session.task("Loading schema", || {
        resolve_table_schema(resource.table_schema.as_ref())
    })?;
    if table_schema.is_none() {
        let inferred = session.task("Inferring schema", || {
            infer_table_schema_from_table(table.clone(), &options)
        })?;
        table_schema = Some(inferred);
    }

    let checks: Vec<TableCheck> = input
        .check
        .iter()
        .flatten()
        .map(|check| create_table_check(check))
        .collect();
    let report = session.task("Validating table", || {
        let errors = inspect_table(
            table,
            table_schema.as_ref(),
            Some(&checks),
            None,
            None,
            None,
        )?;
        Ok(create_report(
            errors.into_iter().map(FairspecError::Table).collect(),
            None,
        ))
    })?;

    session.render_report_result(&report)
}

fn create_table_check(check: &str) -> TableCheck {
    let (name, condition) = check.split_once('=').unwrap_or((check, ""));
    TableCheck {
        name: name.to_string(),
        condition: TableCheckCondition::Sql(condition.to_string()),
        column_name: None,
        message: None,
    }
}
