---
title: Terminal
label: Terminal
path: /python/terminal/
order: 10
icon: square-terminal
---

The Fairspec Terminal provides a command-line interface for managing datasets, tables, files, and JSON data. It exposes the framework's core capabilities to scripts, CI pipelines, and interactive shells without requiring Python code.

The pages in this section walk through the available commands grouped by what they operate on.

## Automation

Every command takes `--json` to write its result as JSON. Given a value, `--json` also reads the command's input from it, using the option names in camelCase, so a script or an agent can skip shell quoting:

```bash
fairspec table validate --json '{"path": "data.csv", "check": ["positive-price=price > 0"]}'
```

`fairspec --llms` prints every command's input and output as JSON Schema, in the shape of an MCP `tools/list` response.

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

## Remote URLs

Commands fetch any `http(s)` URL you give them, intranet and `localhost` included. Set `FAIRSPEC_HOST_POLICY=public` to refuse hosts that resolve to a loopback, private, link-local (cloud metadata at `169.254.169.254` among them) or other non-public address. The check runs on the address actually connected to and again on every redirect, and proxy settings are ignored under it.

`fairspec mcp` takes URLs from model input, so it starts with the `public` policy unless `FAIRSPEC_HOST_POLICY` is set; `FAIRSPEC_HOST_POLICY=any` lets it reach intranet hosts again:

```json
{
  "mcpServers": {
    "fairspec": {
      "command": "uvx",
      "args": ["fairspec@latest", "mcp"],
      "env": { "FAIRSPEC_HOST_POLICY": "any" }
    }
  }
}
```
