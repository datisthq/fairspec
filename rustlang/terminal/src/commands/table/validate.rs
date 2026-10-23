use clap::Args;
use fairspec_library::fairspec_metadata::{FairspecError, create_report, resolve_table_schema};
use fairspec_library::fairspec_table::{
    LoadTableOptions, TableCheck, TableCheckCondition, infer_table_schema_from_table, inspect_table,
};
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
pub struct ValidateTableInput {
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
    /// a named SQL condition every row must satisfy, as name=condition (repeatable)
    #[arg(long, value_name = "value", allow_hyphen_values = true, value_parser = parse_table_check, help_heading = "Checks")]
    #[serde(default)]
    pub check: Option<Vec<String>>,
}

pub fn validate_table_command(
    input: &ValidateTableInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let session = Session::new(console, false, input.debug, json);
    let options = create_infer_table_schema_options(&input.table_schema_params);
    let resource = create_table_resource(
        &session,
        input.path.as_deref(),
        &input.select,
        &input.dialect,
        input.table_schema.as_deref(),
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

    Ok(session.render_report_result(&report))
}

pub fn parse_table_check(value: &str) -> Result<String, String> {
    match value.split_once('=') {
        Some((name, _)) if !name.is_empty() => Ok(value.to_string()),
        _ => Err("Expected \"name=condition\"".to_string()),
    }
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
