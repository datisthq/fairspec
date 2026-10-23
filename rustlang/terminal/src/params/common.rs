use clap::Args;
use serde::Deserialize;

/// `--json [value]` and `--llms`, which every command but `mcp` and `script` takes.
#[derive(Debug, Clone, Default, Args)]
pub struct OutputParams {
    /// write output as JSON; pass a JSON string to also drive input
    #[arg(long, value_name = "value", num_args = 0..=1, default_missing_value = BARE_JSON)]
    pub json: Option<String>,
    /// print this command's schema (name, input, output) as JSON for LLM tool-use
    #[arg(long)]
    pub llms: bool,
}

/// `--llms` alone, for the commands without `--json`.
#[derive(Debug, Clone, Default, Args)]
pub struct LlmsParams {
    /// print this command's schema (name, input, output) as JSON for LLM tool-use
    #[arg(long)]
    pub llms: bool,
}

// `--json` given without a value; a real JSON value can never equal it
pub const BARE_JSON: &str = "\u{0}";

#[derive(Debug, Clone, Default, Args, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectParams {
    /// dataset to select resource from
    #[arg(short = 'd', long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub dataset: Option<String>,
    /// resource in provided dataset
    #[arg(short = 'r', long, value_name = "value", allow_hyphen_values = true)]
    #[serde(default)]
    pub resource: Option<String>,
}
