# Agents

This file provides guidance to coding agents when working with code in this repository.

## General

- Never commit code to git!
- Don't change shadcn code in `elements/`
- Prioritize using LSP capabilities if possible
- When resolving a TODO, follow its instructions literally
- Run type checking as part of your tasks
- Run specs as part of your tasks
- Don't run linting as part of your tasks
- **After every task, check whether a skill in `.agents/skills/` needs updating, and update it in the same commit.** Read the task description for what it taught: a new convention, a correction to how the work was done, or a decision about how an area should look or behave. Write that into the skill for that area, so the next session follows it without being told again. When no skill covers the area and the lesson will recur, add one.

## Skills

Depth lives in `.agents/skills/` (reached by Claude Code through the `.claude/skills` symlink `pnpm install` creates), so
this file carries invariants and the skills carry procedure. Reach for one before working in
its area. Each is named `<subject>-<facet>`, subject first (`change-review`):

| skill           | when                                                          |
| --------------- | ------------------------------------------------------------- |
| `change-review` | reviewing a pull request — also what the review workflow runs |

## Commands

- Run `pnpm lint` to lint the code
- Run `pnpm format` to auto-fix formatting issues
- Run `pnpm type` to check TypeScript types
- Run `pnpm test` to run the full test suite including linting, type checking, and tests
- Run `pnpm unit` to run only the Vitest tests
- Run `pnpm exec vitest run -t "test name"` or `pnpm exec vitest run path/to/test.ts` to run a single test
- Run `pnpm docs:start` to serve the landing site and `pnpm docs:build` to build it

## Structure

- The renderer (React SPA) lives flat at the repository root: `routes/`, `components/`,
  `elements/`, `hooks/`, `helpers/`, `services/`, `constants/`, `styles/`, `locales/`
- Electron process code lives under `processes/main/` and `processes/preload/`
- `models/` holds the zod schemas shared by the renderer and the engine — the oRPC wire contract
- The renderer must never import from `#processes/*`, `electron` or `node:*`; the only legal
  crossing is the type-only `Router` import in `services/engine.ts` (enforced by oxlint)
- `.livemark/` is the landing site published at application.fairspec.org, built with livemark and
  deployed to Cloudflare as static assets. It is independent of the Electron app, is English-only
  and outside the i18n workflow, and its dependencies must stay in `devDependencies` so they never
  reach the installer

## i18n

- Any change that adds or edits a user-facing string must run `pnpm extract` and then fill
  `msgstr` for every new or changed message in all seven non-source catalogues
- `pt` is European Portuguese, never Brazilian
- `lingui extract` reports a Total count that **includes obsolete entries**. A `#~ msgid` whose
  string no longer exists still counts, and grepping `^msgstr ""` will never find it because the
  line is `#~ msgstr ""`. Parse with `@lingui/format-po`'s own `parse()` to see what is genuinely
  untranslated, or run `pnpm extract --clean` to drop obsolete entries
- Long msgids wrap across lines in the PO, so naive text replacement silently misses them. Fill
  those through the same parser and `serialize()` rather than string surgery

## Formats

- Use 2-space indentation, UTF-8 encoding, and LF line endings
- Use PascalCase for classes and interfaces, and camelCase for methods and variables
- Place high-level public items first in a file and low-level private items last
- Use ES modules with full import paths including the ".ts(x)" file extension

## Types

- Use strict TypeScript with null checks but don't add explicit return types to functions
- Never use TypeScript `any`, type casting `as`, or `!` without permission

## Specs

- Place unit tests in `<module>.unit.ts` files and don't add useless comments like "Arrange", "Act", "Assert"

## Releases

Conventional Commits drive [release-please](https://github.com/googleapis/release-please): a
`feat` or `fix` landing on `main` opens a Release PR, and merging that PR cuts the version,
tags it and publishes. Merging is the gate — nothing ships without it.

- **IMPORTANT: only changes to the shipped application use `feat` or `fix`.** Everything else MUST be
  `chore` or `docs` — CI and workflows, release config, dependencies, tooling, `AGENTS.md`,
  the README, the docs site
- This is not a style preference. release-please reads the commit type: a `fix:` on a workflow
  file cuts a release and burns a version number on a change no consumer can observe. Version
  numbers can never be reused
- When in doubt, ask whether the change alters what someone installing the app gets.
  If it does not, it is `chore`
- Release config lives in `.release/config.json` and `.release/manifest.json`. The manifest
  holds the current version and release-please maintains it — never hand-edit it

## Docs

- Add Typedoc comments only for public APIs and don't add them for files or use @params directives
- Don't write `//` comments in the code
