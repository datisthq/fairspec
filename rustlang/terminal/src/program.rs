use std::ffi::OsString;

use clap::error::ErrorKind;
use clap::{Args, CommandFactory, FromArgMatches, Parser, Subcommand};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::commands::data::infer_schema::{InferDataSchemaInput, infer_data_schema_command};
use crate::commands::data::validate::{ValidateDataInput, validate_data_command};
use crate::commands::data::validate_schema::{
    ValidateDataSchemaInput, validate_data_schema_command,
};
use crate::commands::dataset::copy::{CopyDatasetInput, copy_dataset_command};
use crate::commands::dataset::infer::{InferDatasetInput, infer_dataset_command};
use crate::commands::dataset::list::{ListDatasetInput, list_dataset_command};
use crate::commands::dataset::query::{QueryDatasetInput, query_dataset_command};
use crate::commands::dataset::script::{ScriptDatasetInput, script_dataset_command};
use crate::commands::dataset::validate::{ValidateDatasetInput, validate_dataset_command};
use crate::commands::file::copy::{CopyFileInput, copy_file_command};
use crate::commands::file::describe::{DescribeFileInput, describe_file_command};
use crate::commands::file::preview::{PreviewFileInput, preview_file_command};
use crate::commands::file::validate::{ValidateFileInput, validate_file_command};
use crate::commands::file_dialect::infer::{InferFileDialectInput, infer_file_dialect_command};
use crate::commands::general::mcp::{McpInput, mcp_command};
use crate::commands::table::describe::{DescribeTableInput, describe_table_command};
use crate::commands::table::infer_schema::{InferTableSchemaInput, infer_table_schema_command};
use crate::commands::table::preview::{PreviewTableInput, preview_table_command};
use crate::commands::table::query::{QueryTableInput, query_table_command};
use crate::commands::table::render_schema::{RenderTableSchemaInput, render_table_schema_command};
use crate::commands::table::script::{ScriptTableInput, script_table_command};
use crate::commands::table::validate::{ValidateTableInput, validate_table_command};
use crate::commands::table::validate_schema::{
    ValidateTableSchemaInput, validate_table_schema_command,
};
use crate::helpers::console::Console;
use crate::helpers::json::dump_json;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::{BARE_JSON, OutputParams};

const MANIFEST: &str = include_str!("../assets/manifest.json");

/// Fairspec Rust is a fast data management framework built on top of the Fairspec standard and Polars DataFrames
#[derive(Debug, Parser)]
#[command(
    name = "fairspec",
    disable_version_flag = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    subcommand_value_name = "COMMAND"
)]
struct Cli {
    /// output the version number
    #[arg(short = 'v', long)]
    version: bool,
    /// print the program manifest (subcommand schemas) as JSON for LLM tool-use
    #[arg(long)]
    llms: bool,
    #[arg(long, action = clap::ArgAction::Help, global = true, help = "Show this message and exit.")]
    help: Option<bool>,
    #[command(subcommand)]
    command: Option<Program>,
}

#[derive(Debug, Subcommand)]
enum Program {
    /// Dataset related commands
    Dataset(Box<Group<DatasetCommand>>),
    /// Table related commands
    Table(Box<Group<TableCommand>>),
    /// Data related commands
    Data(Box<Group<DataCommand>>),
    /// File related commands
    File(Box<Group<FileCommand>>),
    /// Start the MCP server
    Mcp(McpInput),
}

#[derive(Debug, Args)]
#[command(disable_help_subcommand = true)]
struct Group<T: Subcommand> {
    /// print the program manifest (subcommand schemas) as JSON for LLM tool-use
    #[arg(long)]
    llms: bool,
    #[command(subcommand)]
    command: Option<T>,
}

#[derive(Debug, Subcommand)]
enum DatasetCommand {
    /// Infer a dataset from local or remote file paths
    Infer(Box<InferDatasetInput>),
    /// Copy a local or remote dataset to a local folder
    Copy(CopyDatasetInput),
    /// Validate a dataset from a local or remote path
    Validate(ValidateDatasetInput),
    /// List Dataset resources
    List(ListDatasetInput),
    /// Query a dataset from a local or remote path with SQL
    Query(QueryDatasetInput),
    /// Script a dataset descriptor
    Script(ScriptDatasetInput),
}

#[derive(Debug, Subcommand)]
enum TableCommand {
    /// Show stats for a table from a local or remote path
    Describe(Box<DescribeTableInput>),
    /// Preview a table from a local or remote path
    Preview(Box<PreviewTableInput>),
    /// Query a table from a local or remote path
    Query(Box<QueryTableInput>),
    /// Start a scripting session for a table
    Script(Box<ScriptTableInput>),
    /// Validate a table from a local or remote path
    Validate(Box<ValidateTableInput>),
    /// Infer a table schema from a table
    InferSchema(Box<InferTableSchemaInput>),
    /// Render a Table Schema as HTML or Markdown
    RenderSchema(RenderTableSchemaInput),
    /// Validate a Table Schema
    ValidateSchema(ValidateTableSchemaInput),
    /// Infer the dialect of a file
    InferDialect(InferFileDialectInput),
}

#[derive(Debug, Subcommand)]
enum DataCommand {
    /// Validate a JSON Data from a local or remote path
    Validate(ValidateDataInput),
    /// Infer a Data Schema
    InferSchema(InferDataSchemaInput),
    /// Validate a Data Schema
    ValidateSchema(ValidateDataSchemaInput),
    /// Infer the dialect of a file
    InferDialect(InferFileDialectInput),
}

#[derive(Debug, Subcommand)]
enum FileCommand {
    /// Copy a local or remote file to a local path
    Copy(CopyFileInput),
    /// Show stats for a local or remote file
    Describe(DescribeFileInput),
    /// Show the raw source of a local or remote file
    Preview(PreviewFileInput),
    /// Validate a file from a local or remote path
    Validate(ValidateFileInput),
    /// Infer the dialect of a file
    InferDialect(InferFileDialectInput),
}

/// Runs the `fairspec` program on the process arguments and exits with its code.
pub fn main() {
    let console = Console::new();
    let code = run_program(std::env::args_os(), &console);
    std::process::exit(code);
}

/// Runs the program on `args` (the first is the binary name), writing to `console`;
/// returns the exit code.
pub fn run_program(
    args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    console: &Console,
) -> i32 {
    let matches = match create_command().try_get_matches_from(args) {
        Ok(matches) => matches,
        Err(error) => return render_clap_error(&error, console),
    };
    let cli = match Cli::from_arg_matches(&matches) {
        Ok(cli) => cli,
        Err(error) => return render_clap_error(&error, console),
    };

    if cli.version {
        console.write_out(&format!("{}\n", env!("CARGO_PKG_VERSION")));
        return 0;
    }
    if cli.llms {
        return print_manifest(console, "");
    }
    match cli.command {
        None => print_help(console, &[]),
        Some(Program::Mcp(input)) => {
            if input.output.llms {
                return print_manifest(console, "mcp");
            }
            finish(console, false, mcp_command(&input, console))
        }
        Some(Program::Dataset(group)) => run_group(
            console,
            "dataset",
            *group,
            |command, console| match command {
                DatasetCommand::Infer(input) => run_leaf(
                    console,
                    "dataset infer",
                    &*input,
                    &input.output,
                    get_empty,
                    infer_dataset_command,
                ),
                DatasetCommand::Copy(input) => run_leaf(
                    console,
                    "dataset copy",
                    &input,
                    &input.output,
                    |input| {
                        check_argument(&input.path, "path")
                            .into_iter()
                            .chain(check_option(&input.to_path, "toPath", "--to-path <value>"))
                            .collect()
                    },
                    copy_dataset_command,
                ),
                DatasetCommand::Validate(input) => run_leaf(
                    console,
                    "dataset validate",
                    &input,
                    &input.output,
                    |input| check_argument(&input.path, "path"),
                    validate_dataset_command,
                ),
                DatasetCommand::List(input) => run_leaf(
                    console,
                    "dataset list",
                    &input,
                    &input.output,
                    |input| check_argument(&input.path, "path"),
                    list_dataset_command,
                ),
                DatasetCommand::Query(input) => run_leaf(
                    console,
                    "dataset query",
                    &input,
                    &input.output,
                    |input| {
                        check_argument(&input.path, "path")
                            .into_iter()
                            .chain(check_argument(&input.query, "query"))
                            .collect()
                    },
                    query_dataset_command,
                ),
                DatasetCommand::Script(input) => {
                    if input.output.llms {
                        return print_manifest(console, "dataset script");
                    }
                    let missing = check_argument(&input.path, "path");
                    if !missing.is_empty() {
                        return render_missing(console, &missing, false);
                    }
                    finish(console, false, script_dataset_command(&input, console))
                }
            },
        ),
        Some(Program::Table(group)) => {
            run_group(console, "table", *group, |command, console| match command {
                TableCommand::Describe(input) => run_leaf(
                    console,
                    "table describe",
                    &*input,
                    &input.output,
                    get_empty,
                    describe_table_command,
                ),
                TableCommand::Preview(input) => run_leaf(
                    console,
                    "table preview",
                    &*input,
                    &input.output,
                    get_empty,
                    preview_table_command,
                ),
                TableCommand::Query(input) => run_leaf(
                    console,
                    "table query",
                    &*input,
                    &input.output,
                    get_empty,
                    query_table_command,
                ),
                TableCommand::Script(input) => {
                    if input.output.llms {
                        return print_manifest(console, "table script");
                    }
                    finish(console, false, script_table_command(&input, console))
                }
                TableCommand::Validate(input) => run_leaf(
                    console,
                    "table validate",
                    &*input,
                    &input.output,
                    get_empty,
                    validate_table_command,
                ),
                TableCommand::InferSchema(input) => run_leaf(
                    console,
                    "table infer-schema",
                    &*input,
                    &input.output,
                    get_empty,
                    infer_table_schema_command,
                ),
                TableCommand::RenderSchema(input) => run_leaf(
                    console,
                    "table render-schema",
                    &input,
                    &input.output,
                    |input| {
                        check_argument(&input.path, "path")
                            .into_iter()
                            .chain(check_option(
                                &input.to_format,
                                "toFormat",
                                "--to-format <value>",
                            ))
                            .collect()
                    },
                    render_table_schema_command,
                ),
                TableCommand::ValidateSchema(input) => run_leaf(
                    console,
                    "table validate-schema",
                    &input,
                    &input.output,
                    |input| check_argument(&input.path, "path"),
                    validate_table_schema_command,
                ),
                TableCommand::InferDialect(input) => run_leaf(
                    console,
                    "table infer-dialect",
                    &input,
                    &input.output,
                    get_empty,
                    infer_file_dialect_command,
                ),
            })
        }
        Some(Program::Data(group)) => {
            run_group(console, "data", *group, |command, console| match command {
                DataCommand::Validate(input) => run_leaf(
                    console,
                    "data validate",
                    &input,
                    &input.output,
                    |input| {
                        check_argument(&input.path, "path")
                            .into_iter()
                            .chain(check_option(
                                &input.data_schema,
                                "schema",
                                "--schema <value>",
                            ))
                            .collect()
                    },
                    validate_data_command,
                ),
                DataCommand::InferSchema(input) => run_leaf(
                    console,
                    "data infer-schema",
                    &input,
                    &input.output,
                    |input| check_argument(&input.path, "path"),
                    infer_data_schema_command,
                ),
                DataCommand::ValidateSchema(input) => run_leaf(
                    console,
                    "data validate-schema",
                    &input,
                    &input.output,
                    |input| check_argument(&input.path, "path"),
                    validate_data_schema_command,
                ),
                DataCommand::InferDialect(input) => run_leaf(
                    console,
                    "data infer-dialect",
                    &input,
                    &input.output,
                    get_empty,
                    infer_file_dialect_command,
                ),
            })
        }
        Some(Program::File(group)) => {
            run_group(console, "file", *group, |command, console| match command {
                FileCommand::Copy(input) => run_leaf(
                    console,
                    "file copy",
                    &input,
                    &input.output,
                    |input| check_option(&input.to_path, "toPath", "--to-path <value>"),
                    copy_file_command,
                ),
                FileCommand::Describe(input) => run_leaf(
                    console,
                    "file describe",
                    &input,
                    &input.output,
                    get_empty,
                    describe_file_command,
                ),
                FileCommand::Preview(input) => run_leaf(
                    console,
                    "file preview",
                    &input,
                    &input.output,
                    get_empty,
                    preview_file_command,
                ),
                FileCommand::Validate(input) => run_leaf(
                    console,
                    "file validate",
                    &input,
                    &input.output,
                    |input| check_argument(&input.path, "path"),
                    validate_file_command,
                ),
                FileCommand::InferDialect(input) => run_leaf(
                    console,
                    "file infer-dialect",
                    &input,
                    &input.output,
                    get_empty,
                    infer_file_dialect_command,
                ),
            })
        }
    }
}

const HELP_TEMPLATE: &str = "Usage: {usage}\n\n{about-with-newline}\n{all-args}";

/// The command tree with click's help layout: usage first, wrapped at 80 columns.
fn create_command() -> clap::Command {
    fn apply(command: clap::Command) -> clap::Command {
        command
            .help_template(HELP_TEMPLATE)
            .max_term_width(80)
            .mut_subcommands(apply)
    }
    apply(Cli::command())
}

/// A required input the command was run without.
struct Missing {
    key: &'static str,
    message: String,
    is_argument: bool,
}

fn check_argument(value: &Option<String>, name: &'static str) -> Vec<Missing> {
    let is_missing = value.as_ref().filter(|value| !value.is_empty()).is_none();
    let missing = Missing {
        key: name,
        message: format!("error: missing required argument '{name}'"),
        is_argument: true,
    };
    if is_missing {
        vec![missing]
    } else {
        Vec::new()
    }
}

fn check_option(value: &Option<String>, key: &'static str, flags: &str) -> Vec<Missing> {
    let is_missing = value.as_ref().filter(|value| !value.is_empty()).is_none();
    let missing = Missing {
        key,
        message: format!("error: required option '{flags}' not specified"),
        is_argument: false,
    };
    if is_missing {
        vec![missing]
    } else {
        Vec::new()
    }
}

fn get_empty<I>(_input: &I) -> Vec<Missing> {
    Vec::new()
}

fn run_group<T: Subcommand>(
    console: &Console,
    name: &str,
    group: Group<T>,
    dispatch: impl FnOnce(T, &Console) -> i32,
) -> i32 {
    if group.llms {
        return print_manifest(console, name);
    }
    match group.command {
        Some(command) => dispatch(command, console),
        None => print_help(console, &[name]),
    }
}

/// A leaf: `--llms` prints its manifest; `--json <value>` replaces argv input with the
/// JSON value; then the required inputs are checked and the command runs.
fn run_leaf<I: Clone + DeserializeOwned>(
    console: &Console,
    path: &str,
    input: &I,
    output: &OutputParams,
    get_missing: impl Fn(&I) -> Vec<Missing>,
    run: impl FnOnce(&I, bool, &Console) -> Result<CommandOutput, Exit>,
) -> i32 {
    if output.llms {
        return print_manifest(console, path);
    }
    let json_input = output.json.as_deref().filter(|value| *value != BARE_JSON);
    let input = match json_input {
        Some(value) => match parse_json_input::<I>(value) {
            Ok(input) => input,
            Err(message) => {
                console.write_err(&format!("{message}\n"));
                return 1;
            }
        },
        None => input.clone(),
    };
    let missing = get_missing(&input);
    if !missing.is_empty() {
        return render_missing(console, &missing, json_input.is_some());
    }
    finish(
        console,
        output.json.is_some(),
        run(&input, output.json.is_some(), console),
    )
}

fn parse_json_input<I: DeserializeOwned>(value: &str) -> Result<I, String> {
    let parsed: Value = serde_json::from_str(value)
        .map_err(|error| format!("error: --json value is not valid JSON: {error}"))?;
    if !parsed.is_object() {
        return Err("error: invalid --json input: Input should be an object".to_string());
    }
    serde_json::from_value(parsed).map_err(|error| format!("error: invalid --json input: {error}"))
}

/// fireargs' wording: under `--json` every missing field; from argv the first missing
/// argument, else every missing option.
fn render_missing(console: &Console, missing: &[Missing], is_json_input: bool) -> i32 {
    let lines: Vec<String> = if is_json_input {
        missing
            .iter()
            .map(|missing| {
                format!(
                    "error: invalid --json input at '{}': Field required",
                    missing.key
                )
            })
            .collect()
    } else if let Some(argument) = missing.iter().find(|missing| missing.is_argument) {
        vec![argument.message.clone()]
    } else {
        missing
            .iter()
            .map(|missing| missing.message.clone())
            .collect()
    };
    console.write_err(&format!("{}\n", lines.join("\n")));
    1
}

fn finish(console: &Console, json: bool, result: Result<CommandOutput, Exit>) -> i32 {
    match result {
        Ok(output) => {
            if json {
                console.write_out(&format!("{}\n", dump_json(&output.data)));
            }
            output.code
        }
        Err(Exit(code)) => code,
    }
}

/// The `--llms` manifest of a route: the `help` tool, then every tool below `path`, each
/// named by its path beneath the route. The tool schemas are Python's, snapshotted in
/// `assets/manifest.json`, so both CLIs describe the same input and output.
fn print_manifest(console: &Console, path: &str) -> i32 {
    let manifest: Value = serde_json::from_str(MANIFEST).unwrap_or_default();
    let tools = manifest["tools"].as_array().cloned().unwrap_or_default();
    let leaf = path.rsplit(' ').next().unwrap_or_default();
    let selected: Vec<Value> = tools
        .into_iter()
        .filter_map(|mut tool| {
            let name = tool["name"].as_str()?.to_string();
            let renamed = if name == "help" || path.is_empty() {
                name
            } else if name == path {
                leaf.to_string()
            } else {
                name.strip_prefix(&format!("{path} "))?.to_string()
            };
            tool["name"] = Value::String(renamed);
            Some(tool)
        })
        .collect();
    console.write_out(&format!(
        "{}\n",
        dump_json(&serde_json::json!({"tools": selected}))
    ));
    0
}

fn print_help(console: &Console, path: &[&str]) -> i32 {
    let mut command = create_command();
    command.build();
    for name in path {
        command = match command.find_subcommand(name) {
            Some(subcommand) => subcommand.clone(),
            None => break,
        };
    }
    console.write_out(&format!("{}\n", command.render_help()));
    0
}

fn render_clap_error(error: &clap::Error, console: &Console) -> i32 {
    let text = error.render().to_string();
    match error.kind() {
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
            console.write_out(&text);
            0
        }
        // A rejected CLI value exits 1 as fireargs does; other usage errors exit 2 as click does
        ErrorKind::ValueValidation | ErrorKind::InvalidValue => {
            console.write_err(&text);
            1
        }
        _ => {
            console.write_err(&text);
            2
        }
    }
}
