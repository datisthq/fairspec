use clap::Args;
use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::query_dataset;
use serde::Deserialize;

use super::list::load_dataset_model;
use crate::commands::general::script::run_sql_session;
use crate::helpers::console::Console;
use crate::helpers::input::require_value;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::LlmsParams;
use crate::session::Session;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptDatasetInput {
    /// local or remote path to the descriptor
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// Enable debug mode to print exception details to stderr
    #[arg(long)]
    #[serde(default)]
    pub debug: bool,
    #[command(flatten)]
    #[serde(skip)]
    pub output: LlmsParams,
}

pub fn script_dataset_command(
    input: &ScriptDatasetInput,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let session = Session::new(console, false, input.debug, false);

    let dataset = session.task("Loading dataset", || load_dataset_model(path))?;

    session.render_text(
        "[dim]Each line runs as SQL against the dataset, a table per resource name; an empty line exits[/dim]",
        Some("warning"),
    );
    run_sql_session(console, |query| {
        query_dataset(dataset.clone(), query, &LoadTableOptions::default())
    });

    Ok(CommandOutput::new(serde_json::json!({})))
}
