use fairspec_library::fairspec_dataset::SaveDatasetOptions;
use fairspec_library::fairspec_metadata::Dataset;
use fairspec_library::{load_dataset, save_dataset};

use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::models::output::{Exit, TextResult};
use crate::session::Session;

#[fireargs::input]
pub struct CopyDatasetInput {
    /// local or remote path to the descriptor
    #[fireargs(argument)]
    pub path: String,
    /// a local output path
    pub to_path: String,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn copy_dataset_command(console: &Console) -> fireargs::Command<CopyDatasetInput, TextResult> {
    let console = console.clone();
    fireargs::command()
        .description("Copy a local or remote dataset to a local folder")
        .handler(
            async move |input: CopyDatasetInput, context: fireargs::Context| {
                Ok(run_copy_dataset(&input, context.json, &console)?)
            },
        )
}

fn run_copy_dataset(
    input: &CopyDatasetInput,
    json: bool,
    console: &Console,
) -> Result<TextResult, Exit> {
    let path = input.path.as_str();
    let to_path = input.to_path.as_str();
    let session = Session::new(console, input.silent, input.debug, json);

    session.task("Copy dataset", || {
        let descriptor =
            load_dataset(path)?.ok_or_else(|| create_value_error("Could not load dataset"))?;
        let dataset: Dataset = serde_json::from_value(serde_json::Value::Object(descriptor))
            .map_err(|error| create_value_error(&error.to_string()))?;
        let options = SaveDatasetOptions {
            target: to_path.to_string(),
            with_remote: false,
        };
        save_dataset(&dataset, &options)?.ok_or_else(|| {
            create_value_error(&format!("Could not copy dataset to \"{to_path}\""))
        })?;
        Ok(())
    })?;

    Ok(session.render_text_result(
        &format!("Copied dataset from [bold]{path}[/bold] to [bold]{to_path}[/bold]"),
        Some("success"),
    ))
}
