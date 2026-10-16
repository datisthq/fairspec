# fairspec-python

Python uv workspace monorepo — a data management framework built on the Fairspec standard
and Polars DataFrames: metadata models, a table engine with per-format plugins, dataset
source plugins, and a CLI.

## Rules

- **Never commit unless asked to**
- **Never add co-authored by Claude Code to commits!**
- **Never push main or pull requests to origin unless asked to**
- **Never amend or rewrite existing commits unless asked** (`git commit --amend`, `git reset` of already-made commits, `git rebase`, force-squash) — other worktrees or sessions may be based on those commits. Fold in changes with a follow-up commit instead.
- Prioritize using LSP capabilities if possible
- When resolving a TODO, follow its instructions literally
- Run type checking, specs and linting as part of your tasks
- Start a plan from a new/updated API summary (models/signatures) and the most challenging points, then continue with your default settings
- Update docs when a change requires it
- **After every task, check whether a skill in `.agents/skills/` needs updating, and update it in the same commit.** Read the task description for what it taught: a new convention, a correction to how the work was done, or a decision about how an area should look or behave. Write that into the skill for that area, so the next session follows it without being told again. When no skill covers the area and the lesson will recur, add one.

## Skills

Depth lives in `.agents/skills/` (reached by Claude Code through the `.claude/skills` symlink `uv run task install` creates), so
this file carries invariants and the skills carry procedure. Reach for one before working in
its area. Each is named `<subject>-<facet>`, subject first (`change-review`):

| skill            | when                                                          |
| ---------------- | ------------------------------------------------------------- |
| `change-review`  | reviewing a pull request — also what the review workflow runs |

## Commands

- `uv run task install` — `uv sync --all-packages` plus the lefthook hooks
- `uv run task test` — full gate: lint + type + unit
- `uv run task lint` — `ruff check`
- `uv run task format` — `ruff format`, auto-fix formatting
- `uv run task type` — `ty check`
- `uv run task unit` — pytest with coverage (html + json reports)
- Single test — `uv run pytest metadata/fairspec_metadata/actions/descriptor/_test/load_unit.py`, or `uv run pytest -k "name"`
- `uv run task coverage` — open the generated `htmlcov` report
- `uv run task build` — `uv build --all-packages` into `build/`
- `uv run task publish` — `uv publish build/*`
- `pnpm leaks` / `pnpm vulns` / `pnpm scan` — gitleaks / semgrep / both
- `pnpm deps` — update all dependencies to their latest versions
- `pnpm docs:start` / `pnpm docs:build` — the livemark documentation site

## Modules

Packages form a strict DAG — `metadata` depends on nothing and everything layers up from it.
Never introduce an edge that reverses this.

```
metadata ─► dataset ─► table ─► library ─► terminal ─► fairspec
```

A package may depend on any layer below it, not only the adjacent one — `table` pulls in both
`dataset` and `metadata`, and `library` pulls in all three.

Each workspace directory `<name>/` holds one import package `<name>/fairspec_<name>/`, except
the meta-package, which is `fairspec/fairspec/`, and `project/`, which ships nothing.

- `metadata` — pydantic models (`Resource`, `Dataset`, `TableSchema`, `Column`, `FileDialect`), descriptor load/save, path normalization, JSON Schema profiles. No `fairspec-*` dependencies; the base of the graph.
- `dataset` — file, folder and stream I/O plus dataset-source plugins: ckan, descriptor, folder, github, zenodo, zip.
- `table` — the Polars-backed table engine (normalize/denormalize, column checks and types, schema inference, the dialect sniffer) plus file-format plugins: arrow, csv, inline, json, parquet, sqlite, xlsx.
- `library` — the plugin registry and the facade actions every consumer calls: `load_table`, `save_table`, `load_dataset`, `save_dataset`, `validate_*`, `infer_*`.
- `terminal` — the `fairspec` CLI (typer), one command group per entity.
- `fairspec` — umbrella package re-exporting `fairspec_library` and shipping the CLI binary.
- `project` — the documentation site in `docs/` and the docs guard in `_test/`. Not a uv workspace member and nothing imports it.

Unlike `fairspec-typescript`, this repo has no `mcp-server` and no `extension` package.

## Code structure

Organise by **concern folder, then entity**: group a file by what it _is_ (an action, a
model, a command), then subgroup by the entity it acts on.

- `actions/<entity>/<verb>.py` — the operations everything else wraps (`actions/table/save.py`, `actions/column/create.py`).
- `models/<entity>.py` — pydantic models and their derived types.
- `plugins/<name>/` — a self-contained plugin: `plugin.py` (the class), `settings.py`, `__init__.py`, and its own `actions/` tree.
- `commands/<entity>/<verb>.py` (terminal) — CLI commands.
- `helpers/<name>.py` — small supporting functions. `utils/<name>/` — heavier self-contained modules (e.g. `table/fairspec_table/utils/sniffer/`).
- `services/`, `profiles/`, `schemas/`, `params/` — external clients, JSON Schema profiles, bundled schemas, CLI option definitions.
- Root files, one concern each: `__init__.py` (the public API surface), `plugin.py`, `settings.py`.

### `_test` and `_shared`

A leading underscore marks a directory that is **not a member of the structure around it**.
TypeScript uses a leading `-` for this; Python cannot, because `_shared` is imported by its
siblings and `-shared` is not a valid identifier.

The same constraint sets the test suffix. TypeScript names tests `<module>.unit.ts`; Python
uses `<module>_unit.py`, not `<module>.unit.py`, because a dot is not legal in a module name
and specs import their subject relatively — `.unit.py` would break every `from ..load import`.

- **`_test/`** — unit tests and everything that exists only to serve them. `<module>.py` is tested by `_test/<module>_unit.py`, fixtures live in `_test/fixtures/`, and generated artifacts (VCR cassettes) in `_test/fixtures/generated/`. A shared test double goes in the same folder, named after its export (`plugins/xlsx/actions/table/_test/test_data.py`).
- **`_shared/`** — _production_ code shared by the siblings around it that must not itself be one of them: a helper among one-action-per-file modules (`actions/table/_shared/helpers.py`). Not fixtures, not mocks. A `_shared/` folder can hold its own `_test/`.

Test discovery is by the `*_unit.py` **filename suffix** (`python_files` in `pyproject.toml`),
not the folder name — the folder is a structural convention. Move a test and its `fixtures/`
together: fixture paths resolve from `os.path.dirname(__file__)`, and the `vcr_cassette_dir`
fixture resolves from the test file's own directory.

**Every directory needs an `__init__.py`**, `_test/` and `_shared/` included. Specs import
their subject relatively (`from ..load import load_descriptor`), and pytest walks up the
`__init__.py` chain to name the module — one missing marker makes the walk stop early and the
relative import reaches beyond the top-level package.

## Formats

- Use 4-space indentation, UTF-8 encoding, and LF line endings
- Ruff is configured at `line-length = 90` with `E501` ignored
- Use PascalCase for classes, snake_case for functions, methods, and variables
- Place high-level public items first in a file and low-level private items last

## Conventions

- uv workspace, Python `>=3.12`, hatchling per member, taskipy for tasks
- Function names are **verb_noun** — a verb plus the noun it acts on (`load_descriptor`, `inspect_json_schema`, `create_column_from_property`), never a bare verb or bare noun
- Files are snake_case, named after the verb **without repeating the folder** — `actions/descriptor/load.py` exports `load_descriptor`, not `load_descriptor.py`
- One export per file is the default, not an absolute: a file may export a closely related pair
- **`__init__.py` barrels are deliberate and required.** Each package's `__init__.py` is a curated list of explicit named re-exports — never `import *` outside the meta-package. Adding a public API means adding its line there.
- Commit style: Conventional Commits (`feat`, `fix`, `chore`, `docs`, `refactor`); python-semantic-release derives the changelog and version bumps from them

## Types

- Use Python type hints; target Python 3.12+
- Type checking is `ty` (`uv run task type`), configured under `[tool.ty.rules]`
- Never use `Any` without permission

## Specs

- Unit tests are pytest, named `*_unit.py`, in collocated `_test` folders
- Use `class TestXxx:` with `def test_...` methods
- Don't add useless comments like "Arrange", "Act", "Assert"
- Network tests use `@pytest.mark.vcr` from pytest-recording; the cassette lands in `_test/fixtures/generated/<TestClass>.<test_name>.yaml`. Renaming a class or test renames that file, so the old cassette is orphaned and the next run silently re-records from the live network — check `git status` after renaming a recorded test.

## Releases

Conventional Commits drive [release-please](https://github.com/googleapis/release-please): a
`feat` or `fix` landing on `main` opens a Release PR, and merging that PR cuts the version,
tags it and publishes to PyPI. Merging is the gate — nothing ships without it.

- **IMPORTANT: only changes to the published packages use `feat` or `fix`.** Everything else MUST
  be `chore` or `docs` — CI and workflows, release config, dependencies, tooling, `AGENTS.md`,
  the README, the docs site
- This is not a style preference. release-please reads the commit type: a `fix:` on a workflow
  file cuts a release and burns a version number on a change no consumer can observe. PyPI never
  lets a version be re-uploaded
- When in doubt, ask whether the change alters what someone installing a package gets. If it does
  not, it is `chore`
- Release config lives in `.release/config.json` and `.release/manifest.json`. The manifest holds
  the current version and release-please maintains it — never hand-edit it. All six packages are
  lock-stepped to one version via `extra-files`

## Docs

- Add docstrings only for public APIs and don't use them for files
- **Comments must not outnumber code.** If a file has more comment lines than code lines, the comments get cut, not the code. A single comment is at most ~3 lines. The bar is not "true" or "useful context" — it is: **name the specific mistake this comment prevents**. If you cannot name one in a sentence, delete it. Most code needs no comment at all: a file with zero is the normal case, not a gap to fill.
- **Design rationale is git history, not file content.** `git log`/`git blame` already hold why a thing was chosen and what it replaced. A file carries only what someone editing THAT line needs in order not to break something — a non-obvious invariant, a failure that presents as success, a deliberate deviation from a rule in this file.
- Never worth a line: restating what the next statement plainly does · explaining what a well-named function already names · narrating rejected alternatives · defending a choice nobody would question · a preamble on every entry in a config object
