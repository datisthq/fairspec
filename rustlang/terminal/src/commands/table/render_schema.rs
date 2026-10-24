use fairspec_library::fairspec_metadata::{PathOr, RenderTableSchemaOptions, resolve_table_schema};
use fairspec_library::render_table_schema_as;

use crate::helpers::console::Console;
use crate::models::output::{Exit, TextResult};
use crate::params::common::get_choice_text;
use crate::session::Session;

#[fireargs::option]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaFormat {
    Markdown,
    Html,
}

use crate::helpers::resource::create_value_error;
#[fireargs::input]
pub struct RenderTableSchemaInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: String,
    /// target schema format
    pub to_format: SchemaFormat,
    /// a local output path
    pub to_path: Option<String>,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn render_table_schema_command(
    console: &Console,
) -> fireargs::Command<RenderTableSchemaInput, TextResult> {
    let console = console.clone();
    fireargs::command()
        .description("Render a Table Schema as HTML or Markdown")
        .handler(
            async move |input: RenderTableSchemaInput, context: fireargs::Context| {
                Ok(run_render_table_schema(&input, context.json, &console)?)
            },
        )
}

fn run_render_table_schema(
    input: &RenderTableSchemaInput,
    json: bool,
    console: &Console,
) -> Result<TextResult, Exit> {
    let path = input.path.as_str();
    let to_format = get_choice_text(&input.to_format);
    let session = Session::new(console, input.silent, input.debug, json);

    let table_schema = session.task("Loading table schema", || {
        resolve_table_schema(Some(&PathOr::Path(path.to_string())))?
            .ok_or_else(|| create_value_error("Could not load table schema"))
    })?;

    let rendered = session.task("Rendering table schema", || {
        let options = RenderTableSchemaOptions {
            extra: Default::default(),
            format: to_format,
        };
        Ok(render_table_schema_as(&table_schema, &options)?.unwrap_or_default())
    })?;

    let Some(to_path) = input.to_path.as_deref().filter(|path| !path.is_empty()) else {
        session.render_raw_result(&rendered);
        return Ok(TextResult { result: rendered });
    };

    session.task("Saving rendered schema", || {
        Ok(std::fs::write(to_path, &rendered)?)
    })?;

    Ok(session.render_text_result(
        &format!("Saved rendered schema to [bold]{to_path}[/bold]"),
        Some("success"),
    ))
}
