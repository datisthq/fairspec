# Fairspec skills

Agent skills for **using** Fairspec: they teach a coding agent (Claude Code, Codex, Cursor, …) to
validate, infer and explore data with the `fairspec` CLI, the MCP server, or the TypeScript and
Python libraries. The skills for developing this repository live in `.agents/skills/` instead.

| skill                  | when                                                                  |
| ---------------------- | --------------------------------------------------------------------- |
| `fairspec-validation`  | checking data, a table, a dataset, a file or a schema; reading errors |
| `fairspec-inference`   | drafting a dialect, table schema, data schema or dataset descriptor   |
| `fairspec-exploration` | previewing, describing and querying tables; listing resources         |

To install one, copy its folder into your agent's skills directory, e.g. `~/.claude/skills/` for
Claude Code or `.agents/skills/` in your project. The skills expect `fairspec` installed
(`npm install fairspec` or `pip install fairspec`) or the MCP server connected (`fairspec mcp`).
