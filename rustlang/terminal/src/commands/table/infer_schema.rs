use clap::Args;
use fairspec_library::infer_table_schema;
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
pub struct InferTableSchemaInput {
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
}

pub fn infer_table_schema_command(
    input: &InferTableSchemaInput,
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
        None,
    )?;

    let table_schema = session.task("Inferring schema", || {
        infer_table_schema(&resource, &options)?
            .ok_or_else(|| create_value_error("Could not infer table schema"))
    })?;

    Ok(session.render_data_result(serde_json::to_value(table_schema).unwrap_or_default()))
}
