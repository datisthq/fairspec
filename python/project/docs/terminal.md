---
title: Terminal
label: Terminal
path: /python/terminal/
order: 10
icon: square-terminal
---

The Fairspec Terminal provides a command-line interface for managing datasets, tables, files, and JSON data. It exposes the framework's core capabilities to scripts, CI pipelines, and interactive shells without requiring Python code.

The pages in this section walk through the available commands grouped by what they operate on.

## MCP Server

`fairspec mcp` starts an [MCP](https://modelcontextprotocol.io) server over stdio, exposing the same tools as the TypeScript server — inferring and validating datasets, tables, schemas and file dialects, querying tables with SQL, and previewing a file's raw source. A query returns at most `limit` records and a validation report at most `maxErrors` errors (both default to 100), and either is cut short to keep a response under ~50k characters, with `truncated: true` when anything was left out. A file preview returns at most `maxLines` lines (default 20, max 200) or `maxBytes` bytes (default 10,000, max 20,000), whichever comes first. Point an MCP client at it:

```json
{
  "mcpServers": {
    "fairspec": {
      "command": "uvx",
      "args": ["fairspec@latest", "mcp"]
    }
  }
}
```
