use fairspec_library::fairspec_table::LoadTableOptions;
use fairspec_library::query_dataset;

use super::list::load_dataset_model;
use crate::commands::general::script::run_sql_session;
use crate::helpers::console::Console;
use crate::models::output::{Empty, Exit};
use crate::session::Session;

#[fireargs::input]
pub struct ScriptDatasetInput {
    /// local or remote path to the descriptor
    #[fireargs(argument)]
    pub path: String,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn script_dataset_command(console: &Console) -> fireargs::Command<ScriptDatasetInput, Empty> {
    let console = console.clone();
    fireargs::command()
        .description("Script a dataset descriptor")
        .json_option(fireargs::FlagConfig::Disabled)
        .handler(
            async move |input: ScriptDatasetInput, _: fireargs::Context| {
                Ok(run_script_dataset(&input, &console)?)
            },
        )
}

fn run_script_dataset(input: &ScriptDatasetInput, console: &Console) -> Result<Empty, Exit> {
    let path = input.path.as_str();
    let session = Session::new(console, false, input.debug, false);

    let dataset = session.task("Loading dataset", || load_dataset_model(path))?;

    session.render_text(
        "[dim]Each line runs as SQL against the dataset, a table per resource name; an empty line exits[/dim]",
        Some("warning"),
    );
    run_sql_session(console, |query| {
        query_dataset(dataset.clone(), query, &LoadTableOptions::default())
    });

    Ok(Empty {})
}
