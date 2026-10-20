---
title: Terminal
label: Terminal
path: /typescript/terminal/
order: 10
icon: square-terminal
---

The Fairspec Terminal provides a command-line interface for managing datasets, tables, files, and JSON data. It exposes the framework's core capabilities to scripts, CI pipelines, and interactive shells without requiring TypeScript code.

The pages in this section walk through the available commands grouped by what they operate on.

## Automation

Every command takes `--json` to write its result as JSON. Given a value, `--json` also reads the command's input from it, using the option names in camelCase, so a script or an agent can skip shell quoting:

```bash
fairspec table validate --json '{"path": "data.csv", "check": ["positive-price=price > 0"]}'
```

`fairspec --llms` prints every command's input and output as JSON Schema, in the shape of an MCP `tools/list` response. The Python CLI does not have `--llms` or JSON input yet.
