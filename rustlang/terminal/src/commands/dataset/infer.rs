use fairspec_library::fairspec_metadata::{Dataset, Resource};
use fairspec_library::infer_dataset;

use crate::helpers::console::Console;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{Descriptor, Exit, create_descriptor};
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[fireargs::input]
pub struct InferDatasetInput {
    /// local paths to files
    #[fireargs(argument, name = "path")]
    pub paths: Vec<String>,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
    #[fireargs(flatten)]
    pub table_schema_params: TableSchemaParams,
}

pub fn infer_dataset_command(
    console: &Console,
) -> fireargs::Command<InferDatasetInput, Descriptor> {
    let console = console.clone();
    fireargs::command()
        .description("Infer a dataset from local or remote file paths")
        .handler(
            async move |input: InferDatasetInput, context: fireargs::Context| {
                Ok(run_infer_dataset(&input, context.json, &console)?)
            },
        )
}

fn run_infer_dataset(
    input: &InferDatasetInput,
    json: bool,
    console: &Console,
) -> Result<Descriptor, Exit> {
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

    Ok(session.render_data_result(create_descriptor(&dataset)))
}
