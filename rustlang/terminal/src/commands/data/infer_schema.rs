use fairspec_library::fairspec_metadata::Resource;
use fairspec_library::infer_data_schema;

use crate::helpers::console::Console;
use crate::models::output::{Descriptor, Exit};
use crate::session::Session;

use crate::helpers::resource::create_value_error;
#[fireargs::input]
pub struct InferDataSchemaInput {
    /// local or remote path to the file
    #[fireargs(argument)]
    pub path: String,
    /// suppress all output except errors
    #[fireargs(default)]
    pub silent: bool,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
}

pub fn infer_data_schema_command(
    console: &Console,
) -> fireargs::Command<InferDataSchemaInput, Descriptor> {
    let console = console.clone();
    fireargs::command()
        .description("Infer a Data Schema")
        .handler(
            async move |input: InferDataSchemaInput, context: fireargs::Context| {
                Ok(run_infer_data_schema(&input, context.json, &console)?)
            },
        )
}

fn run_infer_data_schema(
    input: &InferDataSchemaInput,
    json: bool,
    console: &Console,
) -> Result<Descriptor, Exit> {
    let path = input.path.as_str();
    let session = Session::new(console, input.silent, input.debug, json);

    let data_schema = session.task("Inferring data schema", || {
        let resource = Resource {
            data: Some(serde_json::Value::String(path.to_string())),
            ..Resource::default()
        };
        infer_data_schema(&resource)?
            .ok_or_else(|| create_value_error("Could not infer data schema"))
    })?;

    Ok(session.render_data_result(data_schema))
}
