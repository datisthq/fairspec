---
title: Agent Skills
label: Agent Skills
description: Teach your coding agent to explore, infer and validate data with Fairspec.
path: /typescript/agent-skills/
order: 5
icon: bot
---

Fairspec ships [Agent Skills](https://agentskills.io/) that tell coding agents such as Claude Code, Cursor and Codex how to work with Fairspec data. Once installed, your agent knows which command, MCP tool or TypeScript call fits a task, and how to read the report it gets back. You don't have to explain any of it in the prompt.

| Skill                  | Your agent uses it when                         |
| ---------------------- | ----------------------------------------------- |
| `fairspec-exploration` | previewing, describing or querying data         |
| `fairspec-inference`   | describing files as dialects, schemas, datasets |
| `fairspec-validation`  | checking data or schemas and fixing the errors  |

## Install from the website

fairspec.org publishes its skills under `/.well-known/agent-skills/`, so the [`skills` CLI](https://github.com/vercel-labs/skills) can install them from the site URL. Run this at your project root:

```bash
npx skills add https://fairspec.org
```

It asks which skills and agents to install for. To skip the prompts, pass them as flags:

```bash
npx skills add https://fairspec.org --skill fairspec-validation --agent claude-code
```

Add `--global` to install for your user instead of the project, and `--list` to see what's available without installing anything. Run `npx skills update` later to pull newer versions.

## Use them

You don't need to invoke a skill yourself. Agents read each skill's description and load it when a task matches: asking to check a CSV against its schema loads `fairspec-validation`, and asking to describe a folder of files loads `fairspec-inference`. If your agent supports invoking skills by name, you can also call one directly, as with `/fairspec-validation` in Claude Code.

Each skill covers every way to run a task: the [terminal](/typescript/terminal/), the [MCP server](/mcp-server/) and the [TypeScript library](/typescript/library/). For example, validating a table with the library:

```ts
import { validateTable } from "fairspec"

const report = await validateTable({
  data: "broken.csv",
  tableSchema: "schema.json",
})

console.log(report.valid)
// false
```
