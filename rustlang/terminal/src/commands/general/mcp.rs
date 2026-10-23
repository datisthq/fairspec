use clap::Args;
use serde::Deserialize;

use crate::helpers::console::Console;
use crate::models::output::{CommandOutput, Exit};
use crate::params::common::LlmsParams;

#[derive(Debug, Clone, Default, Args, Deserialize)]
pub struct McpInput {
    #[command(flatten)]
    #[serde(skip)]
    pub output: LlmsParams,
}

pub fn mcp_command(_input: &McpInput, console: &Console) -> Result<CommandOutput, Exit> {
    if let Err(error) = fairspec_mcp_server::serve_stdio() {
        console.write_err(&format!("{error}\n"));
        return Err(Exit(1));
    }
    Ok(CommandOutput::new(serde_json::json!({})))
}
