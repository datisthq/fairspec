use crate::helpers::console::Console;
use crate::models::output::{Empty, Exit};

#[fireargs::input]
pub struct McpInput {}

pub fn mcp_command(console: &Console) -> fireargs::Command<McpInput, Empty> {
    let console = console.clone();
    fireargs::command()
        .description("Start the MCP server")
        .json_option(fireargs::FlagConfig::Disabled)
        .handler(async move |input: McpInput, _: fireargs::Context| {
            Ok(run_serve_mcp(&input, &console)?)
        })
}

fn run_serve_mcp(_input: &McpInput, console: &Console) -> Result<Empty, Exit> {
    if let Err(error) = fairspec_mcp_server::serve_stdio() {
        console.write_err(&format!("{error}\n"));
        return Err(Exit::default());
    }
    Ok(Empty {})
}
