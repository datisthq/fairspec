---
name: change-verification
description: Use when checking a change works before calling it done — the smallest fmt/lint/type/unit slice and visual check for the diff, not the full gate. Covers check costs, the slice ladder per section, and the guards a narrow slice cannot see.
---

# Verifying a change

**Verify the diff, not the repo.** `pnpm test` lints everything, then builds and tests every
section: TypeScript, desktop, Python and the standard, about a minute and a half. A fix in one
Python plugin cannot break the desktop renderer, so running its suite checks nothing. While
iterating, run the **slice** that could catch this diff breaking. The full gate belongs to the
lefthook pre-push hook and CI.

- Never re-run a check that passed if nothing it covers has changed.
- Never chain two heavy commands in one call. One that fails hides the cost of the other.
- Say in the report which slice ran (e.g. "fmt on 3 files, docs guard, `pnpm typescript unit
table/`, `pnpm typescript type`"). Never write "tests pass" for a slice.

## What each check costs

Measured on a laptop, warm, 2026-10-16:

| check                                        | wall  | catches                                   |
| -------------------------------------------- | ----- | ----------------------------------------- |
| `vp fmt --check <files>` / `vp lint <files>` | ~1 s  | format, oxlint, the desktop import fence  |
| `vp test` (root)                             | ~1 s  | the docs guard                            |
| `pnpm standard test`                         | ~3 s  | profiles compile, spec examples, fixtures |
| `pnpm typescript build`                      | ~8 s  | emits every package's `build/`            |
| `pnpm typescript type`                       | ~6 s  | types in every TypeScript package         |
| `pnpm -F ./typescript/<pkg> run type`        | ~2 s  | types in one package                      |
| `pnpm typescript unit <dir or file>`         | 3+ s  | the tests named                           |
| `pnpm typescript unit`                       | ~32 s | every TypeScript test                     |
| `pnpm desktop type` / `pnpm desktop unit`    | ~3 s  | the app's types / tests                   |
| `pnpm python type`                           | ~2 s  | `ty` over the workspace                   |
| `pnpm python unit <dir or file>`             | 4+ s  | the tests named                           |
| `pnpm python unit`                           | ~16 s | every Python test                         |
| `pnpm lint`                                  | ~3 s  | fmt + oxlint + ruff over the whole tree   |

All of these run from the repo root. Paths after `pnpm typescript unit` are relative to
`typescript/`, after `pnpm python unit` to `python/`. Bare `vitest` fails to collect
`vite-plus/test` suites. The changed files come from `git diff --name-only main...` plus
`git status --short`.

## The ladder

Pick the **highest** rung the diff reaches, and run everything that rung lists.

1. **Markdown, copy, a skill.** `vp fmt --check` on the files. For `AGENTS.md`, a skill, a
   `README.md`, a `CONTRIBUTING.md` or `.humans/`, add `vp test`: the docs guard checks every
   backticked path and `pnpm` command those documents name, the skills table and the skill
   frontmatter.
2. **Published docs or the site.** Rung 1, then look at the page: `pnpm start:site`, then open
   it through Claude in Chrome, falling back to chrome-devtools, then Playwright, only when the
   one above is unavailable (and say so). A landing route in `.livemark/routes/` also gets the
   root types (`pnpm exec tsgo --noEmit`).
3. **Logic in one package.** fmt on the files, the touched folder's tests
   (`pnpm typescript unit table/plugins/csv/` or `pnpm python unit table/`), and that package's
   types (`pnpm -F ./typescript/<pkg> run type`, or `pnpm python type`). A new or changed test is
   run on its own first, and fails before the fix. TypeScript packages type-check against each
   other's `build/`, so rebuild (`pnpm typescript build`) when the diff changed an export another
   package uses.
4. **A lower package**: `metadata`, `dataset` or `table`, whose consumers layer above it.
   `pnpm typescript build`, then `pnpm typescript type`, then `pnpm desktop type` (the app consumes
   `@fairspec/library` and `@fairspec/metadata`), and the consumer tests the change can reach.
   For Python, `pnpm python type` plus the tests of the packages above it.
5. **The desktop app.** fmt + `vp lint` on the files, `pnpm desktop type` and `pnpm desktop unit`
   (after `pnpm typescript build` if `typescript/` changed too). A UI change also gets looked at
   in the running app (`pnpm desktop start`): no unit test renders a screen.
6. **The standard.** A change under `standard/` runs `pnpm standard test`: every profile
   compiles, every JSON example in the Data Schema and Table Schema specs validates, and the
   fixtures in `standard/-test/profiles.unit.ts` hold. A profile fix adds a valid and an invalid
   case there, failing before the fix. Never run `pnpm standard generate` by hand:
   `release.yaml` regenerates the profiles on the Release PR. The implementations' profile tests
   (`pnpm typescript unit metadata/actions/profile/`,
   `pnpm python unit metadata/fairspec_metadata/actions/profile/`) load their own vendored
   copies, so they never see a `standard/profiles/` edit.
7. **Cross-cutting**: dependencies, `pnpm-workspace.yaml`, the root `vite.config.ts`,
   `tsconfig.json` or `package.json`, `lefthook.yaml`, a `pyproject.toml` or `uv.lock`. Run
   `pnpm test`; no cherry-picking here.

**A change made in both implementations is sliced per section.** A TypeScript + Python fix runs
both sections' slices.

## Guards a narrow slice cannot see

Some checks read files from disk or live outside a section's tests, so a folder-scoped run
never reaches them. When the diff touches a trigger, add the check by hand:

| the diff touches                                                   | add                                                                     |
| ------------------------------------------------------------------ | ----------------------------------------------------------------------- |
| a doc or skill, a renamed path, a renamed or removed `pnpm` script | `vp test` (`.scripts/-test/docs.unit.ts`)                               |
| a desktop renderer import                                          | `vp lint` on the file (the root `no-restricted-imports`)                |
| a user-facing desktop string                                       | `pnpm desktop extract`, then the catalogues (**`desktop-development`**) |
| a test path or path assertion in `typescript/`                     | read it for raw separators and bare `\n` — CI runs Windows              |
| a renamed recorded test                                            | `git status` for an orphaned recording                                  |
| a root script, especially a `-F` filter                            | run it, and check its quoting holds on Windows (**`monorepo-layout`**)  |

## When the slice is not enough

- **The slice fails somewhere unexpected.** The diff reaches further than you thought. Go up a
  rung; do not narrow the slice until it passes.
- **Stale `build/`.** A TypeScript type error in a package you did not touch usually means a
  sibling's `build/` predates your change. Rebuild before you chase it.
