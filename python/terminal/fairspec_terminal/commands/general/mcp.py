from fairspec_terminal.program import program


@program.command()
def mcp() -> None:
    """Start the MCP server."""
    # Imported here so the rest of the CLI does not pay for loading the MCP stack
    from fairspec_mcp_server import server

    server.run("stdio")
