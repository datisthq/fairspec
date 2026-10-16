---
name: human-documentation
description: Use when writing a report or research note for people under `.humans/` (one self-contained HTML page by default), and before a branch merges — to find what AGENTS.md, the skills, READMEs and .humans now falsely claim after the diff.
---

# Documentation

Two jobs: writing documents for people (`.humans/`), and keeping every document true when the
code changes.

## Writing for `.humans/`

`.humans/` holds research, registers and reports that people read, as opposed to the agent-facing
`AGENTS.md` and skills, and to the published docs the website renders. **The default format is
one self-contained `.html` page with real visualisation.** Markdown is for working drafts
(a `-draft/` folder beside the page) and short plain notes only. A reader should take in the shape of the data
before reading a sentence of it.

- **One file, no build step, opens from disk.** Styles go inline in a `<style>` block. Fonts may
  come from Google Fonts. If a chart truly needs a library, load it from cdnjs or jsdelivr.
  Otherwise use no script at all.
- **Colour tokens on `:root`** (`--ground`, `--panel`, `--ink`, `--muted`, `--rule`, plus one
  token per status), redefined under `@media (prefers-color-scheme: dark)` for
  `:root:not([data-theme="light"])`. Status colours (adopt / decide / skip) are a set apart from
  category colours. Never use one hue for both meanings.
- **Visualise with HTML and CSS first**: stat tiles for the headline numbers, CSS bars for
  shares and ranks, chips for status and category, a legend wherever colour carries meaning, and
  tables for the detail. Use inline SVG for anything with geometry (a timeline, a flow, a
  distribution). Reach for a chart library only for genuine interaction. The `dataviz` skill
  covers chart choice and palettes; `artifact-design` covers layout and type.
- **Readable at 320px** with no sideways scroll: wide tables scroll inside their own wrapper,
  and the page never does.
- **Numbers carry their date and source.** A register goes stale, so say when it was measured
  and from where, near the top.
- `.humans/` is not published on fairspec.org. A page meant for the public belongs in a
  section's docs or `standard/overview/` instead.

`.scripts/-test/docs.unit.ts` reads only `.md` files, so the paths and commands an HTML page names
go unchecked. Verify them by hand.

## Keeping the docs true

Run this **before merging**, once the work is done and the diff is final.

```sh
git diff main...HEAD --stat
git diff main...HEAD
```

The guard in `.scripts/-test/docs.unit.ts` already fails the build on a path or `pnpm` command
that no longer exists, a skill missing from the `AGENTS.md` table, or a second `AGENTS.md`. That
is the mechanical half. This is the other half: a paragraph that still parses, still points at
real files, and is no longer true.

### What falsifies a doc

Work through the diff asking each of these:

- **A rename.** Grep the docs for the _old_ name, not just the code — package, module, action,
  CLI command, MCP tool, script, directory. This is the single most common miss: the name
  survives in prose after every code reference is gone.
- **A new or deleted package, script or command.** The package lists in the section skills, the
  section tables in `AGENTS.md` and `README.md`, and the command lists all enumerate them.
- **A changed default or constraint.** "a column is nullable unless …" stops being true the day
  the model makes it required.
- **A rule that moved.** If a guard or a lint rule now enforces what prose used to ask for, say
  so — a rule enforced by a test and a rule enforced by hope read identically on the page.
- **A removed mechanism.** Search the docs for the thing you deleted. A paragraph explaining why
  we do X is worse than useless once X is gone.
- **A generated artefact.** The profiles copied by `pnpm standard generate`, and the copies each
  implementation bundles, go stale without failing anything — treat them as documentation for
  this purpose and regenerate them.

### Where to look

- `AGENTS.md` — the only one, at the repo root. Section-level `AGENTS.md` files are not allowed;
  the guard enforces that.
- `.agents/skills/*/SKILL.md` — including the `description`, which is what makes a skill get
  picked up at all. A skill whose body drifted is bad; a skill whose _description_ drifted is
  invisible.
- `README.md`, `CONTRIBUTING.md` (published at `/overview/contributing/`), and each section's
  `README.md` and `CONTRIBUTING.md`.
- The published docs under `python/project/docs/`, `typescript/project/docs/` and
  `standard/` when the diff changes behaviour they describe.
- The pages under `.humans/`, HTML included.
- Comments that assert a repo-wide invariant, not just local ones — the header comment on a
  registry, a guard test's explanation of what it protects.

### Where a fact belongs

`AGENTS.md` loads into every session, so it carries only what must hold before anyone knows which
section they are in: hard rules, invariants that fail silently, the commit rules, one line per
root command, and the skills table. A section's commands, layout, conventions and traps go in
its `<section>-development` skill; a cross-cutting procedure goes in its own skill, with at most
a one-line pointer left in `AGENTS.md`. When a paragraph grows there, move it. Keep `AGENTS.md`
near 16 KB.

Contributor-facing prose for people (setup, how to send a PR) belongs in `CONTRIBUTING.md` and
the READMEs, which may point at `AGENTS.md` but never restate it.

### What not to do

Do not restate the change in the docs. A doc records what someone editing that line needs in
order not to break something; the story of what changed is what `git log` is for. If the diff
adds no invariant and removes none, the right outcome here is **no documentation change**, and
saying so is a complete answer.
