use fairspec_library::infer_table_schema;

use super::resource::create_table_resource;
use crate::helpers::console::Console;
use crate::helpers::resource::create_value_error;
use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::models::output::{Descriptor, Exit, create_descriptor};
use crate::params::common::SelectParams;
use crate::params::file_dialect::DialectParams;
use crate::params::table_schema::TableSchemaParams;
use crate::session::Session;

#[fireargs::input]
pub struct InferTableSchemaInput {
    /// local or remote path to the table
    #[fireargs(argument)]
    pub path: Option<String>,
    #[fireargs(flatten)]
    pub select: SelectParams,
    /// Enable debug mode to print exception details to stderr
    #[fireargs(default)]
    pub debug: bool,
    #[fireargs(flatten)]
    pub dialect: DialectParams,
    #[fireargs(flatten)]
    pub table_schema_params: TableSchemaParams,
}

pub fn infer_table_schema_command(
    console: &Console,
) -> fireargs::Command<InferTableSchemaInput, Descriptor> {
    let console = console.clone();
    fireargs::command()
        .description("Infer a table schema from a table")
        .handler(
            async move |input: InferTableSchemaInput, context: fireargs::Context| {
                Ok(run_infer_table_schema(&input, context.json, &console)?)
            },
        )
}

fn run_infer_table_schema(
    input: &InferTableSchemaInput,
    json: bool,
    console: &Console,
) -> Result<Descriptor, Exit> {
    let session = Session::new(console, false, input.debug, json);
    let options = create_infer_table_schema_options(&input.table_schema_params);
    let resource = create_table_resource(
        &session,
        input.path.as_deref(),
        &input.select,
        &input.dialect,
        None,
    )?;

    let table_schema = session.task("Inferring schema", || {
        infer_table_schema(&resource, &options)?
            .ok_or_else(|| create_value_error("Could not infer table schema"))
    })?;

    Ok(session.render_data_result(create_descriptor(&table_schema)))
}
