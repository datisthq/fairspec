from fireargs import command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import Empty


@command(description="Start the MCP server", json_option=False)
def mcp_command(input: CommandInput) -> Empty:
    # Imported here so the rest of the CLI does not pay for loading the MCP stack
    from fairspec_mcp_server import server

    server.run("stdio")
    return Empty()
