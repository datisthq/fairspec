use clap::Args;
use fairspec_library::fairspec_metadata::{Dataset, Resource};
use fairspec_library::infer_dataset;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InferDatasetInput {
    /// local paths to files
    #[arg(value_name = "path", num_args = 0..)]
    #[serde(default)]
    pub paths: Vec<String>,
    /// Enable debug mode to print exception details to stderr
    #[arg(long)]
    #[serde(default)]
    pub debug: bool,
    #[command(flatten)]
    #[serde(skip)]
    pub output: OutputParams,
    #[command(flatten)]
    #[serde(flatten)]
    pub table_schema_params: TableSchemaParams,
}

pub fn infer_dataset_command(
    input: &InferDatasetInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let session = Session::new(console, false, input.debug, json);
    let options = create_infer_table_schema_options(&input.table_schema_params);

    let dataset = session.task("Inferring dataset", || {
        let dataset = Dataset {
            resources: Some(
                input
                    .paths
                    .iter()
                    .map(|path| Resource {
                        data: Some(serde_json::Value::String(path.clone())),
                        ..Resource::default()
                    })
                    .collect(),
            ),
            ..Dataset::default()
        };
        infer_dataset(&dataset, &options)
    })?;

    Ok(session.render_data_result(serde_json::to_value(dataset).unwrap_or_default()))
}
