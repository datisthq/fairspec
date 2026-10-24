use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::query_dataset;

use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::models::output::{Exit, Records};
use crate::session::Session;

#[fireargs::input]
pub struct QueryDatasetInput {
    /// local or remote path to the descriptor
    #[fireargs(argument)]
    pub path: String,
    /// a SQL query to execute against a dataset (refer to each table by its resource name)
    #[fireargs(argument)]
    pub query: String,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn query_dataset_command(console: &Console) -> fireargs::Command<QueryDatasetInput, Records> {
    let console = console.clone();
    fireargs::command()
        .description("Query a dataset from a local or remote path with SQL")
        .handler(
            async move |input: QueryDatasetInput, context: fireargs::Context| {
                Ok(run_query_dataset(&input, context.json, &console)?)
            },
        )
}

fn run_query_dataset(
    input: &QueryDatasetInput,
    json: bool,
    console: &Console,
) -> Result<Records, Exit> {
    let path = input.path.as_str();
    let query = input.query.as_str();
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
