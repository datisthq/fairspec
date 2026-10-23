use clap::Args;
use fairspec_library::fairspec_metadata::{PathOr, RenderTableSchemaOptions, resolve_table_schema};
use fairspec_library::render_table_schema_as;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

use crate::helpers::input::require_value;
use crate::helpers::resource::create_value_error;
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderTableSchemaInput {
    /// local or remote path to the file
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// target schema format
    #[arg(long, value_parser = ["markdown", "html"])]
    #[serde(default)]
    pub to_format: Option<String>,
    /// a local output path
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub to_path: Option<String>,
    /// suppress all output except errors
    #[arg(long)]
    #[serde(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[arg(long)]
    #[serde(default)]
    pub debug: bool,
    #[command(flatten)]
    #[serde(skip)]
    pub output: OutputParams,
}

pub fn render_table_schema_command(
    input: &RenderTableSchemaInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let to_format = require_value(&input.to_format);
    let session = Session::new(console, input.silent, input.debug, json);

    let table_schema = session.task("Loading table schema", || {
        resolve_table_schema(Some(&PathOr::Path(path.to_string())))?
            .ok_or_else(|| create_value_error("Could not load table schema"))
    })?;

    let rendered = session.task("Rendering table schema", || {
        let options = RenderTableSchemaOptions {
            format: to_format.to_string(),
        };
        Ok(render_table_schema_as(&table_schema, &options)?.unwrap_or_default())
    })?;

    let Some(to_path) = input.to_path.as_deref().filter(|path| !path.is_empty()) else {
        session.render_raw_result(&rendered);
        return Ok(CommandOutput::new(serde_json::json!({"result": rendered})));
    };

    session.task("Saving rendered schema", || {
        Ok(std::fs::write(to_path, &rendered)?)
    })?;

    Ok(session.render_text_result(
        &format!("Saved rendered schema to [bold]{to_path}[/bold]"),
        Some("success"),
    ))
}
