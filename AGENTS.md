# Agents

This file provides guidance to coding agents when working with code in this repository.

Fairspec is one monorepo: the standard, its Python and TypeScript implementations, the desktop
application and the website. Each section has its own `AGENTS.md` with the conventions for
that section — read it before working there.

| section       | what                                                              | released as                  |
| ------------- | ----------------------------------------------------------------- | ---------------------------- |
| `standard/`   | the specification: `specs/`, `profiles/`, `extensions/`           | profiles on fairspec.org     |
| `python/`     | uv workspace: `fairspec` and `fairspec-*` packages                | PyPI                         |
| `typescript/` | pnpm packages: `fairspec` and `@fairspec/*`, incl. the MCP server | npm                          |
| `desktop/`    | Electron application                                              | installers on GitHub release |
| `catalog/`    | reserved for the catalog application                              | —                            |

The website lives at the root (`livemark.config.ts`, `.livemark/`, `worker.ts`,
`wrangler.jsonc`) and renders the docs of every section on fairspec.org.

## General

- **Never commit unless asked to**
- **Never add co-authored by Claude Code to commits!**
- **Never push main or pull requests to origin unless asked to**
- **Never amend or rewrite existing commits unless asked** — fold in changes with a follow-up commit
- Prioritize using LSP capabilities if possible
- When resolving a TODO, follow its instructions literally
- Run type checking as part of your tasks
- Don't run linting as part of your tasks
- Start plan from a summary a new/updated API summary (models/signatures) and most challenging points, continue plan using your default settings
- **After every task, check whether a skill in `.agents/skills/` needs updating, and update it in the same commit.** Read the task description for what it taught: a new convention, a correction to how the work was done, or a decision about how an area should look or behave. Write that into the skill for that area, so the next session follows it without being told again. When no skill covers the area and the lesson will recur, add one.

## Commands

Everything runs from the repo root. `pnpm <verb>` covers every section; `pnpm <section> <script>`
runs one section's script (`pnpm python test`, `pnpm typescript build`, `pnpm desktop start`,
`pnpm standard generate`).

- `pnpm install` — install Node dependencies, git hooks, the skills symlink, and the Python
  environment when `uv` is on the PATH (`pnpm python setup` does that step alone)
- `pnpm test` — the full gate: lint, then every section's tests
- `pnpm lint` — `vp fmt --check && vp lint` for the whole tree, plus `ruff check`
- `pnpm format` — `vp fmt` for the whole tree, plus `ruff format`
- `pnpm type` — root files, then every section
- `pnpm build` — TypeScript packages, the desktop app, Python distributions
- `pnpm unit` — every section's unit tests
- `pnpm site:start` / `pnpm site:build` — the website
- `pnpm leaks` / `pnpm vulns` / `pnpm scan` — gitleaks / semgrep / both
- `pnpm deps` — update all Node dependencies to their latest versions

Order matters across sections: desktop consumes `@fairspec/library` from the workspace, and
TypeScript packages type-check against each other's `build/`, so run `pnpm typescript build`
before typing or testing `typescript/` or `desktop/`.

## Commits

Each section is its own release-please component with its own version, tag
(`<section>-vX.Y.Z`), changelog and Release PR. A commit counts toward the section whose
files it touches. Scope commits with the section: `feat(python): …`, `docs(standard): …`.

- **IMPORTANT**: `feat` and `fix` mean "this changes what the section ships". In each section
  that is only:
  - `standard/` — the normative artifacts: `standard/specs/`, `standard/profiles/`,
    `standard/extensions/`
  - `python/`, `typescript/` — the published packages
  - `desktop/` — the shipped application
- Everything else MUST use `chore` or `docs` — the website and its landing pages,
  `standard/examples/`, `standard/overview/`, READMEs, tooling, CI, dependencies,
  configuration, this file. An example is a demonstration, not a normative artifact
- This is not a style preference. `feat` and `fix` are read by release-please: they open
  a Release PR that bumps the version, and for the standard generate a new profile directory
  under `.livemark/public/profiles/<version>/`. Using them for a landing page tweak announces a
  change that did not happen, and burns a version number that can never be reused
- **Merging a Release PR is the gate.** Review its diff as the release itself — for the
  standard it carries the generated profile JSON with its resolved
  `https://fairspec.org/profiles/<version>/…` URLs. Nothing is tagged until you merge.
  Release config lives in `.release/config.json` and `.release/manifest.json`; the
  manifest holds the current versions and release-please maintains it — never hand-edit it
- For the standard, when in doubt, ask whether the change alters what a conforming descriptor
  or implementation `MUST` do. If it does not, it is `chore` or `docs`
- **A normative change does not require a JSON diff.** Much of the standard is prose
  that JSON Schema cannot express — how a hash is computed, how multipart parts
  combine, what a dialect means. Tightening or correcting one of those is `feat` or
  `fix` even when every file under `standard/profiles/` stays byte-identical, because an
  implementer pinned to the previous version has no other way to learn the
  requirement changed
- Within `standard/specs/`, `standard/profiles/` and `standard/extensions/`, editorial edits
  that leave every requirement intact — typos, dead links, formatting, rewording that changes
  no `MUST` — are still `docs`

## Skills

Depth lives in `.agents/skills/` (reached by Claude Code through the `.claude/skills` symlink `pnpm install` creates), so
this file carries invariants and the skills carry procedure. Reach for one before working in
its area. Each is named `<subject>-<facet>`, subject first (`change-review`):

| skill             | when                                                                   |
| ----------------- | ---------------------------------------------------------------------- |
| `change-review`   | reviewing a pull request — also what the review workflow runs          |
| `monorepo-layout` | adding a section, a root script, a workflow job or a release component |

## Formats

- Use 2-space indentation (4 for Python), UTF-8 encoding, and LF line endings
- Formatting and linting for every JS/TS/JSON/Markdown file are configured once, in the root
  `vite.config.ts`; nested configs only carry `test` settings
- Use PascalCase for classes and interfaces, and camelCase for methods and variables
- Place high-level public items first in a file and low-level private items last
- Use ES modules with full import paths including the ".ts(x)" file extension

## Types

- Use strict TypeScript with null checks but don't add explicit return types to functions
- Never use TypeScript `any`, type casting `as`, or `!` without permission

## Docs

- Add Typedoc comments only for public APIs and don't add them for files or use @params directives
- Don't write `//` comments in the code
