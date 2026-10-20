---
name: python-development
description: Use when working in python/ — the fairspec and fairspec-* PyPI packages, the CLI and the MCP server in the uv workspace. Covers commands, the package DAG, concern-then-entity layout, _test/_shared folders, __init__.py barrels, ty and VCR cassettes.
---

# Python

The `python/` section is a uv workspace: a data management framework built on the Fairspec
standard and Polars DataFrames — metadata models, a table engine with per-format plugins,
dataset source plugins, and a CLI.

## Commands

Run from the repo root; `pnpm -C python <script>` runs a script of this section, which calls
`uv` inside `python/`.

- `pnpm -C python setup` — `uv sync --all-packages` (`pnpm install` runs it when `uv` is on the PATH)
- `pnpm -C python test` — type + unit
- `pnpm -C python check:lint` — `ruff check`
- `pnpm -C python format` — `ruff format`
- `pnpm -C python check:types` — `ty check`
- `pnpm -C python test:unit` — pytest with coverage (html + json reports)
- Single test — `pnpm -C python test:unit metadata/fairspec_metadata/actions/descriptor/_test/load_unit.py`, or `pnpm -C python test:unit -k "name"`
- `pnpm -C python show:coverage` — open the generated `htmlcov` report
- `pnpm -C python build` — `uv build --all-packages` into `build/`
- `pnpm -C python release` — `uv publish build/*` (CI only)

## Modules

Packages form a strict DAG — `metadata` depends on nothing and everything layers up from it.
Never introduce an edge that reverses this.

```
metadata ─► dataset ─► table ─► library ─► mcp-server ─► terminal ─► fairspec
```

A package may depend on any layer below it, not only the adjacent one — `table` pulls in both
`dataset` and `metadata`, and `library` pulls in all three.

Each workspace directory `<name>/` holds one import package `<name>/fairspec_<name>/` (a hyphen
becomes an underscore: `mcp-server/fairspec_mcp_server/`), except the meta-package, which is
`python/fairspec/fairspec/`, and `project/`, which ships nothing.

- `metadata` — pydantic models (`Resource`, `Dataset`, `TableSchema`, `Column`, `FileDialect`), descriptor load/save, path normalization, JSON Schema profiles. No `fairspec-*` dependencies; the base of the graph.
- `dataset` — file, folder and stream I/O plus dataset-source plugins: ckan, descriptor, folder, github, zenodo, zip.
- `table` — the Polars-backed table engine (normalize/denormalize, column checks and types, schema inference, the dialect sniffer) plus file-format plugins: arrow, csv, inline, json, parquet, sqlite, xlsx.
- `library` — the plugin registry and the facade actions every consumer calls: `load_table`, `save_table`, `load_dataset`, `save_dataset`, `validate_*`, `infer_*`.
- `mcp-server` — MCP tools over the library actions (the `mcp` SDK's `MCPServer`), served over stdio as `fairspec mcp`.
- `terminal` — the `fairspec` CLI (typer), one command group per entity.
- `fairspec` — umbrella package re-exporting `fairspec_library` and shipping the CLI binary.
- `project` — the documentation site in `python/project/docs/`. Not a uv workspace member and nothing imports it.

Unlike `typescript/`, this section has no `extension` package.

## Code structure

Organise by **concern folder, then entity**: group a file by what it _is_ (an action, a
model, a command), then subgroup by the entity it acts on.

- `actions/<entity>/<verb>.py` — the operations everything else wraps (`actions/table/save.py`, `actions/column/create.py`).
- `models/<entity>.py` — pydantic models and their derived types.
- `plugins/<name>/` — a self-contained plugin: `plugin.py` (the class), `settings.py`, `__init__.py`, and its own `actions/` tree.
- `commands/<entity>/<verb>.py` (terminal) — CLI commands. `tools/<entity>/<verb>.py` (mcp-server) — MCP tools.
- `helpers/<name>.py` — small supporting functions. `utils/<name>/` — heavier self-contained modules (e.g. `python/table/fairspec_table/utils/sniffer/`).
- `services/`, `profiles/`, `schemas/`, `params/` — external clients, JSON Schema profiles, bundled schemas, CLI option definitions.
- `python/metadata/fairspec_metadata/profiles/` is a copy of `standard/profiles/` written by `pnpm sync:profiles`; never edit it by hand, edit the standard and sync (the root guard fails while they differ)
- Root files, one concern each: `__init__.py` (the public API surface), `plugin.py`, `settings.py`.

### `_test` and `_shared`

A leading underscore marks a directory that is **not a member of the structure around it**.
TypeScript uses a leading `-` for this; Python cannot, because `_shared` is imported by its
siblings and `-shared` is not a valid identifier.

The same constraint sets the test suffix. TypeScript names tests `<module>.unit.ts`; Python
uses `<module>_unit.py`, not `<module>.unit.py`, because a dot is not legal in a module name
and specs import their subject relatively — `.unit.py` would break every `from ..load import`.

- **`_test/`** — unit tests and everything that exists only to serve them. `<module>.py` is tested by `_test/<module>_unit.py`, fixtures live in `_test/fixtures/`, and generated artifacts (VCR cassettes) in `_test/fixtures/generated/`. A shared test double goes in the same folder, named after its export (`python/table/fairspec_table/plugins/xlsx/actions/table/_test/test_data.py`).
- **`_shared/`** — _production_ code shared by the siblings around it that must not itself be one of them: a helper among one-action-per-file modules (`actions/table/_shared/helpers.py`). Not fixtures, not mocks. A `_shared/` folder can hold its own `_test/`.

Test discovery is by the `*_unit.py` **filename suffix** (`python_files` in `python/pyproject.toml`),
not the folder name — the folder is a structural convention. Move a test and its `fixtures/`
together: fixture paths resolve from `os.path.dirname(__file__)`, and the `vcr_cassette_dir`
fixture resolves from the test file's own directory.

**Every directory needs an `__init__.py`**, `_test/` and `_shared/` included. Specs import
their subject relatively (`from ..load import load_descriptor`), and pytest walks up the
`__init__.py` chain to name the module — one missing marker makes the walk stop early and the
relative import reaches beyond the top-level package.

## Conventions

- uv workspace, Python `>=3.12`, hatchling per member; tasks are scripts in `python/package.json`
- Ruff is configured at `line-length = 90` with `E501` ignored
- Function names are **verb_noun** — a verb plus the noun it acts on (`load_descriptor`, `inspect_json_schema`, `create_column_from_property`), never a bare verb or bare noun
- Files are snake_case, named after the verb **without repeating the folder** — `actions/descriptor/load.py` exports `load_descriptor`, not `load_descriptor.py`
- One export per file is the default, not an absolute: a file may export a closely related pair
- **`__init__.py` barrels are deliberate and required.** Each package's `__init__.py` is a curated list of explicit named re-exports — never `import *` outside the meta-package. Adding a public API means adding its line there.
- Type checking is `ty` (`pnpm -C python check:types`), configured under `[tool.ty.rules]`
- Add docstrings only for public APIs and don't use them for files
- **A terminal command that declares the Table Schema inference flags must pass them on**: build them
  with `create_infer_table_schema_options(...)` and splat into the library call
  (`load_table(res, **options)`); declared-but-dropped flags parse fine and do nothing
- **A remote API response is parsed at the service boundary** — a dataset plugin's
  `make_*_api_request` takes a required `model` and returns `model.model_validate(...)`; never
  `Model(**dict)` or raw `dict["key"]` reads. Response models keep the same required fields as
  their zod twins; what the plugin _sends_ gets its own all-optional model (`NewCkanDataset`), so
  the response model can stay strict. Test mocks return realistic bodies from `fixtures/responses/`
- Terminal output of raw file content goes through `Session.render_raw_result`: `render_text_result`
  prints through Rich, whose markup swallows `[...]` in the content
- **Table validation collects once for every column.** A LazyFrame caches nothing, so each
  `collect` re-parses the source: `inspect_columns` (`python/table/fairspec_table/actions/column/inspect.py`)
  puts each column's target, source, cell and UInt8 tag columns into one frame, filters rows
  with any tag, takes one global `head(max_errors)` and expands errors column by column, so the
  spec order holds. Never add a per-column collect; only the row-parsed types (duration, wkb,
  wkt, array, object, geojson, topojson) and the row key checks keep their own scan. Collect
  with `engine=QUERY_ENGINE`

## MCP server

- A tool is a plain sync function returning a `ToolResult` model: `MCPServer` runs sync tools on a worker thread and derives the input and output schemas from the annotations
- Register it in `server.py` wrapped in `guard_tool` (`helpers/tool.py`) under the **name the TypeScript server puts on the wire** — the tool's `name` in `typescript/mcp-server/tools/` (`inferTableSchemaTool`) — so client configs and allowlists carry across. `guard_tool` re-raises any exception as `ToolError` (the SDK shows only `Error executing tool X` for anything else), enforces the 50k-character budget and returns compact JSON text (the SDK's own copy is indented)
- Parameters carry their wire names too (`maxErrors`, not `max_errors`): the SDK passes a `Field` alias as the keyword, so an aliased parameter breaks the call
- A tool returning a list mirrors the TypeScript limits: `limit`/`maxErrors` (default 100, max 1000), one item more fetched, cut with `limit_items`/`limit_report` (`helpers/limit.py`), `truncated` in the result. A tool returning file content takes the same caps as its TypeScript twin in `Field(ge=1, le=…)` (`tools/file/preview.py`)
- Results extend `ToolResult`: `mcp` dumps output without `exclude_none`, so a plain model sends every unset field as `null`
- Nothing may write to stdout — it is the stdio transport. Never use the terminal's `Session` in a tool; `_test/server_unit.py` asserts a call prints nothing. That in-memory `capfd` check is the only guard: while serving stdio the SDK points fd 1 at stderr, so a stray `print` never fails the spawned-process test in `python/terminal/fairspec_terminal/commands/general/_test/mcp_unit.py`
- `fairspec mcp` imports `fairspec_mcp_server` inside the command, so the rest of the CLI does not load the MCP stack

## Twins

Every behaviour here has a twin in `typescript/`, and a change lands in both in the same branch,
with every other surface that shows it (root `AGENTS.md`, **Terminology**). Find the twin by
name, not by path:

- `load_table` ↔ `loadTable`, `_test/` ↔ `-test/`, `__init__.py` ↔ `index.ts`, an entity folder
  `table_schema/` ↔ `tableSchema/`
- Package layouts drift: `library/fairspec_library/actions/table_schema/render.py` is
  TypeScript's `extension` package, and the CLI's
  `python/terminal/fairspec_terminal/commands/table/infer_schema.py` is TypeScript's
  `typescript/terminal/commands/tableSchema/infer.ts` (the command, `table infer-schema`, is the
  same)
- MCP tools carry the TypeScript wire names, inputs and limits (**MCP server**)
- Where Python must produce what a TypeScript dependency produces (resource-name slugs from
  `@sindresorhus/slugify`, data schema inference from `json-schema-it`), port that behaviour
  instead of reaching for a near-equivalent PyPI package
- **A new CLI command or MCP tool is listed by hand outside the code**: its verb in the
  entity's `verbs` on `.livemark/routes/cli-tool.tsx`, a card (and the spelled-out count) on
  `.livemark/routes/mcp-server.tsx`, and a row in the `.skills/fairspec-*` skill that covers the task
- A behaviour both share gets a `testsuite/` case, which compares both outputs exactly
  (**`testsuite-development`**)

## Specs

- Unit tests are pytest, named `*_unit.py`, in collocated `_test` folders
- Use `class TestXxx:` with `def test_...` methods
- Don't add useless comments like "Arrange", "Act", "Assert"
- `library/fairspec_library/_test/testsuite_unit.py` runs the shared `testsuite/` suite; a
  behaviour both libraries share gets a `testsuite/` test there, not only a unit test here
  (**`testsuite-development`**)
- Network tests use `@pytest.mark.vcr` from pytest-recording; the cassette lands in `_test/fixtures/generated/<TestClass>.<test_name>.yaml`. Renaming a class or test renames that file, so the old cassette is orphaned and the next run silently re-records from the live network — check `git status` after renaming a recorded test.

## Releases

This section is the `python` release-please component: tags `python-vX.Y.Z`, its own Release
PR, published to PyPI by `release.yaml`. All seven packages are lock-stepped to one version via
`extra-files`. Only changes to the published packages use `feat` or `fix`; tests, the docs in
`python/project/`, READMEs and tooling are `chore` or `docs`. PyPI never lets a version be
re-uploaded.
