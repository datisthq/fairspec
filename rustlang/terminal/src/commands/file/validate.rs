use clap::Args;
use fairspec_library::fairspec_dataset::validate_file;
use fairspec_library::fairspec_metadata::{Integrity, Resource};
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::OutputParams;
use crate::session::Session;

use crate::helpers::input::{parse_hash_type, require_value};
#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateFileInput {
    /// local or remote path to the file
    #[arg(value_name = "path")]
    #[serde(default)]
    pub path: Option<String>,
    /// expected file hash calculated with the specified hash type
    #[arg(long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub hash: Option<String>,
    /// hash type
    #[arg(long, value_parser = ["md5", "sha1", "sha256", "sha512"], default_value = "sha256")]
    #[serde(default)]
    pub hash_type: Option<String>,
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

pub fn validate_file_command(
    input: &ValidateFileInput,
    json: bool,
    console: &Console,
) -> Result<CommandOutput, Exit> {
    let path = require_value(&input.path);
    let session = Session::new(console, input.silent, input.debug, json);
    let integrity = input
        .hash
        .clone()
        .filter(|hash| !hash.is_empty())
        .map(|hash| Integrity {
            extra: Default::default(),
            r#type: parse_hash_type(input.hash_type.as_deref()),
            hash,
        });

    let report = session.task("Validating file", || {
        validate_file(
            &Resource {
                data: Some(serde_json::Value::String(path.to_string())),
                integrity,
                ..Resource::default()
            },
            None,
        )
    })?;

    Ok(session.render_report_result(&report))
}
