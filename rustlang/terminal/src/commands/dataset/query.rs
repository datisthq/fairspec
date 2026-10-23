use clap::Args;
use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::query_dataset;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::helpers::input::require_value;
use crate::helpers::resource::create_value_error;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDatasetInput {
    /// local or remote path to the descriptor
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// a SQL query to execute against a dataset (refer to each table by its resource name)
    #[arg(value_name = "query")]
    #[serde(default)]
    pub query: Option<String>,
    /// Enable debug mode to print exception details to stderr
    #[arg(long)]
    #[serde(default)]
    pub debug: bool,
    #[command(flatten)]
    #[serde(skip)]
    pub output: OutputParams,
}

pub fn query_dataset_command(
    input: &QueryDatasetInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let query = require_value(&input.query);
    let session = Session::new(console, false, input.debug, json);

    let table = session.task("Executing query", || {
        query_dataset(path.to_string(), query, &LoadTableOptions::default())
    })?;
    let frame = session.task("Collecting data", || {
        table
            .collect()
            .map_err(|error| create_value_error(&error.to_string()))
    })?;

    session.render_frame_result(&frame)
}
