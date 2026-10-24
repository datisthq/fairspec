use fairspec_library::fairspec_metadata::{Dataset, infer_resource_name};
use fairspec_library::load_dataset;
use serde_json::Value;

use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::models::output::Exit;
use crate::session::Session;

#[fireargs::input]
pub struct ListDatasetInput {
    /// local or remote path to the descriptor
    #[fireargs(argument)]
    pub path: String,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn list_dataset_command(console: &Console) -> fireargs::Command<ListDatasetInput, Vec<String>> {
    let console = console.clone();
    fireargs::command()
        .description("List Dataset resources")
        .handler(
            async move |input: ListDatasetInput, context: fireargs::Context| {
                Ok(run_list_dataset(&input, context.json, &console)?)
            },
        )
}

fn run_list_dataset(
    input: &ListDatasetInput,
    json: bool,
    console: &Console,
) -> Result<Vec<String>, Exit> {
    let path = input.path.as_str();
    let session = Session::new(console, false, input.debug, json);

    let dataset = session.task("Loading dataset", || load_dataset_model(path))?;
    let names: Vec<String> = dataset
        .resources
        .iter()
        .flatten()
        .enumerate()
        .map(|(index, resource)| {
            resource
                .name
                .clone()
                .unwrap_or_else(|| infer_resource_name(resource, Some(index + 1)))
        })
        .collect();

    Ok(session.render_data_result(names))
}

pub fn load_dataset_model(
    path: &str,
) -> Result<Dataset, fairspec_library::fairspec_metadata::Error> {
    let descriptor =
        load_dataset(path)?.ok_or_else(|| create_value_error("Could not load dataset"))?;
    serde_json::from_value(Value::Object(descriptor))
        .map_err(|error| create_value_error(&error.to_string()))
}
