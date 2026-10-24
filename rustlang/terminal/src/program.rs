use std::ffi::OsString;
use std::io::Write;

use clap::{Arg, ArgAction};

use crate::commands::data::infer_schema::infer_data_schema_command;
use crate::commands::data::validate::validate_data_command;
use crate::commands::data::validate_schema::validate_data_schema_command;
use crate::commands::dataset::copy::copy_dataset_command;
use crate::commands::dataset::infer::infer_dataset_command;
use crate::commands::dataset::list::list_dataset_command;
use crate::commands::dataset::query::query_dataset_command;
use crate::commands::dataset::script::script_dataset_command;
use crate::commands::dataset::validate::validate_dataset_command;
use crate::commands::file::copy::copy_file_command;
use crate::commands::file::describe::describe_file_command;
use crate::commands::file::preview::preview_file_command;
use crate::commands::file::validate::validate_file_command;
use crate::commands::file_dialect::infer::infer_file_dialect_command;
use crate::commands::general::mcp::mcp_command;
use crate::commands::table::describe::describe_table_command;
use crate::commands::table::infer_schema::infer_table_schema_command;
use crate::commands::table::preview::preview_table_command;
use crate::commands::table::query::query_table_command;
use crate::commands::table::render_schema::render_table_schema_command;
use crate::commands::table::script::script_table_command;
use crate::commands::table::validate::validate_table_command;
use crate::commands::table::validate_schema::validate_table_schema_command;
use crate::helpers::console::Console;
use crate::models::output::Exit;

/// The `fairspec` program, its commands writing human output to `console`.
pub fn create_program(console: &Console) -> fireargs::Program {
    fireargs::program()
        .name("fairspec")
        .description("Fairspec Rust is a fast data management framework built on top of the Fairspec standard and Polars DataFrames")
        .program("dataset", create_dataset_program(console))
        .program("table", create_table_program(console))
        .program("data", create_data_program(console))
        .program("file", create_file_program(console))
        .command("mcp", mcp_command(console))
        .configure(apply_layout)
}

fn create_dataset_program(console: &Console) -> fireargs::Program {
    fireargs::program()
        .description("Dataset related commands")
        .command("infer", infer_dataset_command(console))
        .command("copy", copy_dataset_command(console))
        .command("validate", validate_dataset_command(console))
        .command("list", list_dataset_command(console))
        .command("query", query_dataset_command(console))
        .command("script", script_dataset_command(console))
}

fn create_table_program(console: &Console) -> fireargs::Program {
    fireargs::program()
        .description("Table related commands")
        .command("describe", describe_table_command(console))
        .command("preview", preview_table_command(console))
        .command("query", query_table_command(console))
        .command("script", script_table_command(console))
        .command("validate", validate_table_command(console))
        .command("infer-schema", infer_table_schema_command(console))
        .command("render-schema", render_table_schema_command(console))
        .command("validate-schema", validate_table_schema_command(console))
        .command("infer-dialect", infer_file_dialect_command(console))
}

fn create_data_program(console: &Console) -> fireargs::Program {
    fireargs::program()
        .description("Data related commands")
        .command("validate", validate_data_command(console))
        .command("infer-schema", infer_data_schema_command(console))
        .command("validate-schema", validate_data_schema_command(console))
        .command("infer-dialect", infer_file_dialect_command(console))
}

fn create_file_program(console: &Console) -> fireargs::Program {
    fireargs::program()
        .description("File related commands")
        .command("copy", copy_file_command(console))
        .command("describe", describe_file_command(console))
        .command("preview", preview_file_command(console))
        .command("validate", validate_file_command(console))
        .command("infer-dialect", infer_file_dialect_command(console))
}

/// Runs the `fairspec` program on the process arguments and exits with its code.
pub fn main() {
    let console = Console::new();
    let code = run_program(std::env::args_os(), &console);
    std::process::exit(code);
}

/// Runs the program on `args` (the first is the binary name), writing to `console`;
/// returns the exit code.
pub fn run_program(args: impl IntoIterator<Item = impl Into<OsString>>, console: &Console) -> i32 {
    let args: Vec<OsString> = args.into_iter().map(Into::into).collect();
    let (mut out, mut err) = console.create_writers();
    if get_is_version(&args) {
        let _ = writeln!(out, "{}", env!("CARGO_PKG_VERSION"));
        return 0;
    }
    let program = create_program(console);
    match pollster::block_on(program.try_run_from(args, &mut out, &mut err)) {
        Ok(code) => i32::from(code),
        // A failed command has written its human output; under --json its result is left
        Err(error) => {
            match error.downcast::<Exit>() {
                Ok(exit) => {
                    if let Some(data) = exit.data {
                        let text = serde_json::to_string_pretty(&data).unwrap_or_default();
                        let _ = writeln!(out, "{text}");
                    }
                }
                Err(error) => {
                    let _ = writeln!(err, "error: {error}");
                }
            }
            1
        }
    }
}

// Python's eager `-v, --version` prints the bare version; clap's prefixes the name
fn get_is_version(args: &[OsString]) -> bool {
    args.iter()
        .skip(1)
        .take_while(|arg| arg.to_string_lossy().starts_with('-'))
        .any(|arg| arg == "-v" || arg == "--version")
}

const HELP_TEMPLATE: &str = "Usage: {usage}\n\n{about-with-newline}\n{all-args}";

/// Python's click layout on every route: usage first, 80 columns, `-v, --version`,
/// no `help` subcommand, and single option values that may start with a hyphen.
fn apply_layout(command: clap::Command) -> clap::Command {
    let command = command.arg(
        Arg::new("version")
            .short('v')
            .long("version")
            .action(ArgAction::SetTrue)
            .help("output the version number"),
    );
    apply_route_layout(command)
}

fn apply_route_layout(command: clap::Command) -> clap::Command {
    // A long about switches clap to its spaced long help
    command
        .long_about(None)
        .help_template(HELP_TEMPLATE)
        .max_term_width(80)
        .subcommand_value_name("COMMAND")
        .disable_help_subcommand(true)
        .disable_help_flag(true)
        .arg(
            Arg::new("help")
                .long("help")
                .action(ArgAction::Help)
                .help("Show this message and exit."),
        )
        .mut_args(|arg| match takes_one_value(&arg) {
            true => arg.allow_hyphen_values(true),
            false => arg,
        })
        .mut_subcommands(apply_route_layout)
}

// A variadic or optional value (`--json [value]`) would swallow the flag after it
fn takes_one_value(arg: &Arg) -> bool {
    let is_optional = arg
        .get_num_args()
        .is_some_and(|range| range.min_values() == 0);
    matches!(arg.get_action(), ArgAction::Set) && arg.get_long().is_some() && !is_optional
}
