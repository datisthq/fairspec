# Agents

This file provides guidance to coding agents when working with code in this repository.

Fairspec is one monorepo: the standard, its Python and TypeScript implementations, the desktop
application and the website. This file carries the invariants for all of it; each section's
conventions live in its skill (see **Skills**) — load it before working there.

| section       | what                                                              | released as                  |
| ------------- | ----------------------------------------------------------------- | ---------------------------- |
| `standard/`   | the specification: `specs/`, `profiles/`, `extensions/`           | profiles on fairspec.org     |
| `python/`     | uv workspace: `fairspec` and `fairspec-*` packages                | PyPI                         |
| `typescript/` | pnpm packages: `fairspec` and `@fairspec/*`, incl. the MCP server | npm                          |
| `desktop/`    | Electron application                                              | installers on GitHub release |
| `catalog/`    | reserved for the catalog application                              | —                            |

The website lives at the root (`livemark.config.ts`, `.livemark/`, `worker.ts`,
`wrangler.jsonc`) and renders the docs of every section on fairspec.org.

Dot-directories hold non-product code: `.agents/` skills, `.humans/` reports for people (HTML
by default), `.scripts/` repo-wide guards, `.release/` release-please config, `.livemark/` the
website.

## General

- **Never commit unless asked to** — except that a session working directly on `main` commits
  each task once done. The root's index is shared, so always commit with a pathspec:
  `git commit -m "…" -- <your paths>`
- **Never add co-authored by Claude Code to commits!**
- **Never push main or pull requests to origin unless asked to**
- **Never switch branches in the project root unless asked** — other sessions work in it
- **Never amend or rewrite existing commits unless asked** — other sessions build on them; fold
  in changes with a follow-up commit
- **Never mark a commit breaking** — no `!` and no `BREAKING CHANGE:` footer; release-please
  turns either into a major bump of the section (`1.0.0` from any `0.x`)
- Prioritize using LSP capabilities if possible
- When resolving a TODO, follow its instructions literally
- Run type checking as part of your tasks
- Don't run linting as part of your tasks
- Start plan from a summary a new/updated API summary (models/signatures) and most challenging points, continue plan using your default settings
- **After every task, check whether a skill in `.agents/skills/` needs updating, and update it in the same commit.** Read the task description for what it taught: a new convention, a correction to how the work was done, or a decision about how an area should look or behave. Write that into the skill for that area, so the next session follows it without being told again. When no skill covers the area and the lesson will recur, add one. Where a fact belongs: the **`human-documentation`** skill
- **End every response that finishes a task with a status line: one bold sentence of about ten
  words, as the final paragraph, nothing after it** (e.g. **Docs folded into skills; guard and
  section tests green.**). The agents summary shows only this line

## Invariants

- Packages in `python/` and `typescript/` form a strict DAG: `metadata` depends on nothing and
  everything layers up from it. Never introduce an edge that reverses it
- `index.ts` and `__init__.py` barrels are curated lists of explicit named re-exports, never
  `export *` / `import *`. Adding a public API means adding its line there
- The TypeScript suite must pass on Windows (CI runs it there): never assert on a raw path
  string or a bare `\n` (**`typescript-development`**)
- Renaming a recorded network test orphans its recording and re-records from the live network
  on the next run — check `git status` after renaming one
- `desktop/package.json` pins `productName: fairspec`; Electron derives the user-data directory
  from it, so renaming it orphans every user's settings

## Commands

Everything runs from the repo root. `pnpm <verb>` covers every section; `pnpm <section> <script>`
runs one section's script (`pnpm python test`, `pnpm typescript build`, `pnpm desktop start`,
`pnpm standard generate`). Each section's own scripts are in its skill.

- `pnpm install` — install Node dependencies, git hooks, the skills symlink, and the Python
  environment when `uv` is on the PATH (`pnpm python setup` does that step alone)
- `pnpm test` — the full gate: lint, the docs guard, then every section's tests
- `pnpm lint` — `vp fmt --check && vp lint` for the whole tree, plus `ruff check`
- `pnpm format` — `vp fmt` for the whole tree, plus `ruff format`
- `pnpm type` — root files, then every section
- `pnpm build` — TypeScript packages, the desktop app, Python distributions
- `pnpm unit` — the docs guard, then every section's unit tests
- `pnpm start:site` / `pnpm build:site` — the website
- `pnpm leaks` / `pnpm vulns` / `pnpm scan` — gitleaks / semgrep / both
- `pnpm deps` — update all Node dependencies to their latest versions

Order matters across sections: desktop consumes `@fairspec/library` from the workspace, and
TypeScript packages type-check against each other's `build/`, so run `pnpm typescript build`
before typing or testing `typescript/` or `desktop/`. While iterating, verify the slice the
diff reaches, not the full gate (**`change-verification`**).

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
  `standard/examples/`, `standard/overview/`, READMEs, tests, the docs under `*/project/`,
  tooling, CI, dependencies, configuration, this file and the skills. An example is a
  demonstration, not a normative artifact
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
  or implementation `MUST` do. For a package or the app, ask whether it alters what someone
  installing it gets. If it does not, it is `chore` or `docs`
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

Depth lives in `.agents/skills/` (reached by Claude Code through the `.claude/skills` symlink
`pnpm install` creates), so this file carries invariants and the skills carry procedure. Reach
for one before working in its area. **A repo skill beats a plugin skill on the same subject** —
`change-review` over a generic code review, `change-verification` over
verification-before-completion. Each is named `<subject>-<facet>`, subject first:

| skill                    | when                                                                   |
| ------------------------ | ---------------------------------------------------------------------- |
| `typescript-development` | the npm packages, CLI or MCP server in `typescript/`                   |
| `python-development`     | the PyPI packages or CLI in `python/`                                  |
| `desktop-development`    | the Electron app in `desktop/`, its i18n catalogues                    |
| `monorepo-layout`        | adding a section, a root script, a workflow job or a release component |
| `human-documentation`    | a report for people under `.humans/`, and the docs pass before a merge |
| `issue-resolution`       | taking one issue or bug report to a tested change                      |
| `change-verification`    | checking a change works — the smallest slice that covers the diff      |
| `change-review`          | reviewing a pull request or the local diff — also what CI runs         |

**There is exactly one `AGENTS.md`, at the root.** `.scripts/-test/docs.unit.ts` enforces that,
along with this table and every path and `pnpm` command the docs and skills name.

## Formats

- Use 2-space indentation (4 for Python), UTF-8 encoding, and LF line endings
- Formatting and linting for every JS/TS/JSON/Markdown file are configured once, in the root
  `vite.config.ts`; nested configs only carry `test` settings
- Use PascalCase for classes and interfaces; camelCase for TypeScript methods and variables,
  snake_case for Python functions, methods and variables
- Name functions verbNoun (`loadTable`, `load_table`), never a bare verb or noun; factories are
  `create*`. Methods too
- Name `package.json` scripts `verb[:object]` (`start:site`, `build:site`), never noun-first
- Place high-level public items first in a file and low-level private items last
- Use ES modules with full import paths including the ".ts(x)" file extension

## Types

- Use strict TypeScript with null checks but don't add explicit return types to functions
- Never use TypeScript `any`, type casting `as`, or `!` without permission
- Use Python type hints targeting 3.12+; never use `Any` without permission

## Docs

- Add Typedoc comments or docstrings only for public APIs; don't add them for files or use
  `@params` directives
- **Comments must not outnumber code**, and one is at most ~3 lines. The bar is: **name the
  specific mistake this comment prevents** — if you cannot, delete it. A file with none is the
  normal case, not a gap to fill
- **Design rationale is git history, not file content.** A file carries only what someone
  editing that line needs in order not to break something: a non-obvious invariant, a failure
  that presents as success, a deliberate deviation from a rule here
- Never worth a line: restating what the next statement plainly does · explaining what a
  well-named function already names · narrating rejected alternatives · defending a choice
  nobody would question · a preamble on every entry in a config object
