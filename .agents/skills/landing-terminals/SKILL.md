---
name: landing-terminals
description: Use when changing a landing page hero carousel or a terminal pane on a landing page — what each terminal session shows, the sample files and real runs behind them, and the layout rules that keep them readable.
---

# Landing terminals

Every landing hero (`.livemark/routes/{index,cli-tool,mcp-server,desktop}.tsx`) is livemark's
carousel of slides, each paired with a source chip. The home hero passes `aside` instead:
one fixed `BenchmarkCard` (nyc-311 10GB, full validators only, from
`.humans/benchmark-nyc-311.html`) that does not change with the slides — refresh its numbers
when that report is republished. Fairspec's tools live in a terminal or an assistant, so the
slides are **terminal sessions written as markup** — never screenshots of text, which shrink
to unreadable sizes in a narrow column. They live in `.livemark/components/Panes.tsx`:

| session              | shows                                                          | used on               |
| -------------------- | -------------------------------------------------------------- | --------------------- |
| `ValidateSession`    | `fairspec table validate broken.csv`, then its `--json` errors | home, CLI, desktop    |
| `InferSession`       | `fairspec table infer-schema`, then validating against it      | CLI                   |
| `PreviewSession`     | `fairspec table preview members.csv`                           | CLI                   |
| `QuerySession`       | `fairspec table query` with SQL                                | CLI                   |
| `McpValidateSession` | an assistant calling `validateTableTool`                       | home, MCP             |
| `McpInferSession`    | an assistant calling `inferTableSchemaTool`                    | MCP                   |
| `McpQuerySession`    | an assistant calling `queryTableTool`                          | MCP                   |
| `SkillsSession`      | an agent loading `fairspec-validation` and running its check   | home (hero and split) |

A hero slide wraps a session in `HeroTerminal`; a split section puts it in
`<WindowCard><TerminalPane>`. The only images are the desktop
app's own screens, `desktopShots` in `.livemark/public/desktop/` — one per tool (Dataset, Table,
Data, File), each a real run's result drawer, 1600×1000 webp in light and `-dark`. Recapture
them from the built app: `pnpm -C desktop build`, launch
`desktop/node_modules/.bin/electron desktop --remote-debugging-port=<port>` from a copy of
`samples/` so the fields hold bare file names, set the window to 1200×750 at scale 2 over CDP,
fill each route's input and submit. Toggle the `dark` class on `<html>` for the dark set rather
than the app's theme switch, which persists to the user's settings. The home Standard slide is the annotated
`dataset.json` (`SpecPreview` in `.livemark/routes/index.tsx`).

## Rules

- **Real output only.** Every `$` output and MCP result is pasted from a real run on the files
  in `samples/`: the Python CLI (`uv run --project python fairspec …` from `samples/`) and a
  `tools/call` to `fairspec mcp`. Only the `›` prompts and `●` narration are written by hand;
  they restate the result, never add to it. Change a sample, a command or a chip, and rerun
- **Let text wrap, not shrink.** JSON goes through `Json` as one string and wraps in the pane;
  only polars tables are `TableOutput` (`pre`, line height 1.2 so box drawing joins up)
- **Mind the hero frame.** The source chip hangs off the frame's bottom-left corner, so a
  slide's content stays in the top ~70%; from `xl` the window bleeds off the right edge, so
  `HeroTerminal` caps its width (`xl:max-w-[42rem]`)
- **Match the site.** Colours come from `tk` (Catppuccin Latte/Mocha), so both themes follow
  the page; never hard-code a colour in a session

Look at the heroes afterwards at 1024, 1280 and 1440 px wide, in both themes, and check no text
sits under a chip or past the window's cut-off edge.
