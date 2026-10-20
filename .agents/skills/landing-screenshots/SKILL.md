---
name: landing-screenshots
description: Use when changing a landing page hero carousel or regenerating its screenshots — what each terminal slide shows, the sample files and real runs behind them, the size and theme rules, and how to capture them.
---

# Landing screenshots

Every landing hero (`.livemark/routes/{index,cli-tool,mcp-server,desktop}.tsx`) is livemark's
carousel of screenshots, each paired with a source chip. Fairspec's tools live in a terminal or
an assistant, so the slides are **terminal sessions** rendered as images, listed in `shots` in
`.livemark/components/Panes.tsx`:

| shot           | shows                                                          | used on   |
| -------------- | -------------------------------------------------------------- | --------- |
| `standard`     | `cat dataset.json`, then `fairspec dataset validate`           | home      |
| `validate`     | `fairspec table validate broken.csv`, then its `--json` errors | home, CLI |
| `infer`        | `fairspec table infer-schema`, then validating against it      | CLI       |
| `preview`      | `fairspec table preview members.csv`                           | CLI       |
| `query`        | `fairspec table query` with SQL                                | CLI       |
| `mcp-validate` | an assistant calling `validateTableTool`                       | home, MCP |
| `mcp-infer`    | an assistant calling `inferTableSchemaTool`                    | MCP       |
| `mcp-query`    | an assistant calling `queryTableTool`                          | MCP       |
| `skills`       | an agent loading `fairspec-validation` and running its check   | home      |

Desktop's slide is the app's own screenshot, `.livemark/public/application.webp`.

## Rules

- **Real output only.** `render.ts` runs every `$` command with the Python CLI on the files in
  `samples/`, and gets every MCP result from a real `tools/call` to `fairspec mcp`. Only the
  `›` prompts and `●` narration are written by hand; they restate the result, never add to it.
  Change a sample, a command or a chip, and re-render, so the image and the chip agree
- **Keep content in the top half.** The source chip covers the frame's lower-left from 55%
  down, and from `xl` the window bleeds off the right edge, so a slide's text stays in the
  top-left ~48% × 70%. Wrap JSON (`jq -c` plus the renderer's wrap) rather than let it run
- **Match the site.** Light background `#ffffff`, dark `#0a0a0a`, Catppuccin Latte/Mocha token
  colours, JetBrains Mono 13px with ligatures off; table output keeps line height 1.2 so box
  drawing joins up
- **Size.** 960×600 CSS px at device scale 2 → 1920×1200 webp, `<id>.webp` and
  `<id>-dark.webp` in `.livemark/public/screenshots/`

## Regenerating

```bash
node .agents/skills/landing-screenshots/render.ts
```

It needs `uv` (the Python CLI and MCP server), `jq`, Chrome (`google-chrome`, or set
`CHROME`) and ImageMagick's `convert`, and writes straight into
`.livemark/public/screenshots/`. Look at the heroes afterwards at 1024, 1280 and 1440 px wide,
in both themes, and check no text sits under a chip.
