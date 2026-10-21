---
name: typescript-development
description: Use when working in typescript/ — the fairspec and @fairspec/* npm packages, the CLI and the MCP server. Covers commands, the package DAG, concern-then-entity layout, -test/-shared folders, barrels, Windows-safe specs and recordings.
---

# TypeScript

The `typescript/` section is a data management framework built on the Fairspec standard and
Polars DataFrames: metadata models, a table engine with per-format plugins, dataset source
plugins, a CLI and an MCP server.

## Commands

Run from the repo root; `pnpm -C typescript <script>` runs a script of this section.

- `pnpm -C typescript test` — build + unit
- `pnpm -C typescript build` — `tsgo` per package, emitting to `<pkg>/build`; `terminal` then bundles its bin with `vp pack`
- `pnpm -C typescript check:types` — `tsgo --noEmit` per package; build first, packages type-check against each other's `build/`
- `pnpm -C typescript test:unit` — `vp test` across every package
- Single test — `pnpm -C typescript test:unit table/plugins/csv/actions/table/-test/load.unit.ts` (path relative to `typescript/`; bare `vitest` won't load the vite-plus test runtime and fails to collect `vite-plus/test` suites)
- `pnpm -C typescript measure:coverage` — the unit tests with coverage, which `vp test` leaves off (CI's Linux leg runs it for Codecov); `pnpm -C typescript show:coverage` opens the report
- `pnpm -C typescript release` — publish every package (CI only)
- Lint and format are repo-wide: `pnpm check:lint`, `pnpm format`

## Modules

Packages form a strict DAG — `metadata` depends on nothing and everything layers up from it.
Never introduce an edge that reverses this.

```
metadata ─┬─► dataset ───┐
          ├─► extension ─┼─► library ─► mcp-server ─► terminal ─► fairspec
          └─► table ─────┘
```

- `metadata` — zod models (`Resource`, `Dataset`, `TableSchema`, `Column`, `FileDialect`), descriptor load/save, path normalization, JSON Schema profiles. No `@fairspec/*` dependencies; the base of the graph and the only browser-safe core.
- `dataset` — file, folder and stream I/O (`loadFile`, `saveFile`, `prefetchFiles`, temp files) plus dataset-source plugins: ckan, descriptor, folder, github, zenodo, zip.
- `table` — the Polars-backed table engine (normalize/denormalize, column checks and types, schema inference, the dialect sniffer) plus file-format plugins: arrow, csv, inline, json, parquet, sqlite, xlxs.
- `extension` — renderers turning a `TableSchema` into markdown or HTML.
- `library` — the plugin registry (`system.ts`) and the facade actions every consumer calls: `loadTable`, `saveTable`, `loadDataset`, `saveDataset`, `validate*`, `infer*`.
- `mcp-server` — MCP tools over the library actions (the official `@modelcontextprotocol/sdk` `McpServer`), served over stdio as `fairspec mcp`. Clients see each tool under its `name` (`inferTableSchemaTool`); the Python server registers the same names, so renaming one renames it for both.
- `terminal` — the `fairspec` CLI, built with fireargs (zod input/output schemas over commander), one command group per entity. Its bin is a bundle (**CLI bundle**, below); its `program` export is plain tsgo output.
- `fairspec` — umbrella package re-exporting `@fairspec/library`; its bin only imports `@fairspec/terminal/run`.
- `project` — the documentation site in `typescript/project/docs/`. Private and outside the DAG: nothing depends on it.

## Code structure

Organise by **concern folder, then entity**: group a file by what it _is_ (an action, a
model, a command), then subgroup by the entity it acts on.

- `actions/<entity>/<verb>.ts` — the operations everything else wraps (`actions/table/save.ts`, `actions/column/create.ts`).
- `models/<entity>.ts` — zod schemas and their derived types.
- `plugins/<name>/` — a self-contained plugin: `plugin.ts` (the class), `settings.ts`, `index.ts`, and its own `actions/` tree.
- `commands/<entity>/<verb>.ts` (terminal) — CLI commands. `tools/<entity>/<verb>.ts` (mcp-server) — MCP tools.
- `helpers/<name>.ts` — small supporting functions. `utils/<name>.ts` — heavier self-contained modules (e.g. `typescript/table/utils/sniffer/`).
- `services/`, `profiles/`, `schemas/`, `params/`, `entrypoints/` — external clients, JSON Schema profiles, bundled schemas, CLI option definitions, executable entry points.
- `typescript/metadata/profiles/` is a copy of `standard/profiles/` written by `pnpm sync:profiles`; never edit it by hand, edit the standard and sync (the root guard fails while they differ)
- Root files, one concern each: `index.ts` (the public API surface), `plugin.ts`, `settings.ts`, `system.ts`, `main.ts`.

### `-test` and `-shared`

A leading `-` marks a directory that is **not a member of the structure around it**.

- **`-test/`** — unit tests and everything that exists only to serve them. `<module>.ts` is tested by `-test/<module>.unit.ts`, fixtures live in `-test/fixtures/`, and generated artifacts (snapshots, `useRecording` HAR recordings) in `-test/fixtures/generated/`. A shared test double goes in the same folder, named after its export.
- **`-shared/`** — _production_ code shared by the siblings around it that must not itself be one of them: a helper among one-action-per-file modules (`actions/column/-shared/helpers.ts`). Not fixtures, not mocks. A `-shared/` folder can hold its own `-test/`.

Test discovery is by the `*.unit.ts` **filename suffix**, not the folder name — the folder is
a structural convention. Move a test and its `fixtures/` together: `resolveSnapshotPath` and
`useRecording` both derive their paths from the test file's own directory.

## Conventions

- Package manager pnpm `^11`, Node `^24`, ES Modules throughout
- Formatting is configured in the root `vite.config.ts`: no semicolons, 90-column print width, arrow parens avoided
- Function names are **verbNoun** — a verb plus the noun it acts on (`saveArrowTable`, `inferTableSchemaFromTable`, `createColumnFromProperty`), never a bare verb or bare noun
- Files are camelCase, named after the verb **without repeating the folder** — `actions/column/create.ts` exports `createColumnFromProperty`, not `createColumn.ts`
- One export per file is the default, not an absolute: a file may export a closely related pair (`typescript/dataset/actions/file/temp.ts` exports `getTempFilePath` and `writeTempFile`)
- Use relative imports with the full `.ts`/`.tsx` file extension
- **`index.ts` barrels are deliberate and required.** Every package publishes `"exports": "./build/index.js"` (`metadata` adds only the test-only `testing` subpath), and its `index.ts` is a curated list of explicit named re-exports — never `export *`. Adding a public API means adding its line there.
- Add brief docstrings for exported functions and models (explain what it does)
- Nothing the MCP server loads may write to stdout — it is the stdio transport. `typescript/mcp-server/-test/main.unit.ts` spawns the real server and fails on any non-JSON line
- **Polars runs on the streaming engine through `POLARS_FORCE_STREAMING`**, set once in `typescript/table/settings.ts` (listed in `sideEffects`). Async `collect()` takes no engine and its `streaming` option is a dead flag, so call plain `collect()` — never `collect({ streaming: true })`, and never `collectSync` to reach the engine, which blocks the MCP server and the desktop app. Not every expression streams: `Expr.replace` holds the whole column (use an anchored `str.replace`), so one such check makes a validation's memory grow with the file. Judge a memory change by the smallest cgroup cap it survives (`systemd-run --user --scope -p MemoryMax=…`), not peak RSS, which counts the mmap'd source
- **Table validation collects once for every column.** A LazyFrame caches nothing, so each
  `collect()` re-parses the source: `inspectColumns` (`typescript/table/actions/column/inspect.ts`)
  puts each column's target, source, cell and UInt8 tag columns into one frame, filters rows
  with any tag, takes one global `head(maxErrors)` and expands errors column by column, so the
  spec order holds. Never add a per-column collect; only the row-parsed types (duration, wkb,
  wkt, array, object, geojson, topojson) and the row key checks keep their own scan
- **A row key check probes before it locates.** `inspectRows` first runs a `groupBy(key).len()`
  probe on the key columns alone, and only on a duplicate runs the row-indexed
  `pl.struct(key).isFirstDistinct()` query. Never locate rows with a join: joins drop null keys,
  while key checks compare partial nulls and skip only all-null keys (`createKeyPresenceExpr`).
  nodejs-polars' `pl.struct` takes plain strings as literals, not column names — pass
  `pl.col(name)` expressions, or every row looks distinct and duplicates pass silently.
  `LazyFrame.groupBy` defaults `maintainOrder` to true; pass `false` where order is never read.
  The Python twin hashes keys inside the cell checks' read instead of probing; nodejs-polars has
  no lazy batch function to do the same, so this probe's second read stays
- **nodejs-polars is patched** (`patches/nodejs-polars@0.26.1.patch`, `patchedDependencies` in
  `pnpm-workspace.yaml`): the `Expr` and `Series` wrappers set their computed symbol keys after
  the object literal, which makes each wrapper ~20x cheaper to build — about 0.4s of every
  validation. Bumping nodejs-polars means re-creating the patch (`pnpm patch`), or dropping it
  once upstream builds the wrappers the same way
- **CLI bundle**: `pnpm -C typescript build` bundles `terminal`'s bin with `vp pack`
  (`typescript/terminal/vite.config.ts`) into `build/entrypoints/`, which cuts startup from
  ~0.5s to ~0.3s by loading ~40 files instead of ~1250. Only `nodejs-polars` and `calamine-node`
  (native) and `wasm-xlsxwriter` (reads its `.wasm` beside it) stay external, and
  `deps.onlyImport` fails the build if the bundle imports anything else, so a new native, wasm
  or worker-thread dependency goes in that list, in `terminal`'s own `dependencies` and in
  `desktop/package.json`'s: the bundle resolves it from `terminal`, not from the package that uses it.
  `entrypoints/run.ts` sets the warning filter and the compile cache, then `await import`s the
  program so it lands in a later chunk; never re-add a side-effect-only module there, because
  `sideEffects: false` lets the bundler drop it and the bin then prints nothing.
  `typescript/terminal/entrypoints/-test/run.unit.ts` runs the built bin, so it needs a build first
- **Spreadsheets** (`typescript/table/plugins/xlxs/`) read with `calamine-node` and write with
  `wasm-xlsxwriter` and our own ODS writer. Python reads the same cells with `python-calamine`,
  and `helpers/table.ts` owns the typing rule both share (`testsuite/tests/table-load/format-xlsx-cells.json`):
  data starts after the last header row, a column keeps its cells' one type, and a mixed column
  becomes text (numbers as `String(n)`, dates as ISO). calamine-node caps input at 64 MiB unless
  `maxInputBytes` is raised, needs ODS opted into, and rejects whitespace between ODS cells,
  which is why the ODS writer emits none. Both dependencies are third-party and pinned exactly.
  calamine allocates the whole used range natively before `maxCells` is checked, so a 5 KB file
  naming `XFD1048576` aborts the process: `readSheetRows` first runs `assertSheetSize`
  (`helpers/guard.ts`), which streams the sheet XML and refuses more than 50M cells. It must
  see what calamine sees: entries from the zip's central directory (`helpers/zip.ts`, which allows
  prepended data), sheets from `xl/_rels/workbook.xml.rels`, UTF-16 decoded, duplicate names
  refused, and a tag pattern that honours quotes, since a `>` in an attribute can hide the `r`
  after it. A file passes once per size and mtime, so dialect inference and the load share it
- **Custom table checks** (`typescript/table/actions/check/inspect.ts`) add one scan for every
  row condition and one select per condition kind for the aggregates. Whether a check is a row
  or a table check comes from running it on two typed null rows (one row back means an
  aggregate); never guess it from the expression, or a broadcast aggregate fails every row
- **A zod model an MCP tool publishes holds only what JSON carries.** `z.custom` makes the
  server fail to list its tools ("Custom types cannot be represented in JSON Schema"), so
  `TableCheck` keeps `condition: z.string()` and widens its TypeScript type to accept `pl.Expr`
- **A table plugin's `loadTable` returns `undefined` when the data is not a table** — a JSON file whose rows (the document, or the value at `jsonPointer`) are not an array, such as a top-level object. Library `inferTableSchema` and `validateTable` read that as "no table", so the resource keeps only its `dataSchema`; never throw for it. `jsonPointer` is always an RFC 6901 pointer (`/items`, `""` for the root) — go through `parseJsonPointer`/`formatJsonPointer` in `typescript/table/plugins/json/helpers/pointer.ts`, never index by the raw string
- **A terminal command that declares the Table Schema inference flags must pass them on** — fireargs
  hands every flag to the handler in `input`, so pass that to `loadTable`/`inferTableSchemaFromTable`
  (`loadTable(resource, { ...input, previewBytes })`); declared-but-dropped flags parse fine and do nothing
- **A terminal command is `f.command(...).input(z.object).output(schema).handler((input, ctx) => …)`**
  (fireargs). Build its `Session` from `{ ...input, json: ctx.json }` and `return` a
  `session.render*Result(...)`: it prints only in human mode and returns what `--json` writes.
  Keep the output schema to what `--json` printed before, since `dataset list` and the frame
  commands print bare arrays; a frame command prints the frame itself and returns `[]` outside
  `--json`. fireargs parses every output, so the items of a large one (report errors,
  table rows) stay `z.unknown()` (`terminal/models/output.ts`): a full model costs seconds and a
  copy at a million errors. Shell syntax (comma lists, `false|1,2`, JSON) goes in `f.option({ parse })`, never
  `.transform()`, which breaks `--llms`; a repeatable flag such as `--check` is a `z.array(...)` option
- A fireargs `f.program` registers an action for `--llms`, which drops commander's implicit
  `help [command]` subcommand; `program.ts` restores it with `helpCommand: { name: "help [command]" }`
  on every program
- An exported function whose inferred type names a `nodejs-polars` type outside `table` fails
  the build with TS2883 ("cannot be named without a reference to … LazyDataFrame"); annotate it
  with the `Table` type (`Promise<Table>`), the one place an explicit return type is required
- **Schema inference picks the first regex in `createRegexMapping` that matches the sample**,
  each on its own, so a value set two patterns split (integers and decimals) falls to `string`.
  A number pattern therefore takes an optional fraction and sits after its integer pattern;
  the Python twin keeps the same keys in the same order
- `columnProperties` overrides are zod-parsed, so one without a `type` matches `UnknownColumnProperty`
  and keeps only its annotations (`title`, `description`, `rdfType`); constraints need the `type`
- **A remote API response is parsed at the service boundary** — a dataset plugin's
  `make*ApiRequest` takes a required zod `schema` and returns its output; never `as T`. A
  response model requires nothing the API may omit or null (real CKAN and Zenodo send `null`
  freely; mark those `.nullish()`) and leaves out what the code never reads; one endpoint, one model (`ZenodoRecord` for `/records`,
  `ZenodoDeposition` for `/deposit`). Test mocks return realistic bodies, never `{ id, url }` stubs
- `loadData` accepts any JSON document (array, scalar, `{}`, `[]`): test for `undefined`, never truthiness
- **nodejs-polars traps the table engine works around**: a bare `pl.Decimal()` aborts the
  whole process (pass precision and scale, as `pl.Decimal(38, 18)`), and a Decimal column
  cannot be collected to JS, so drop it before `toRecords()`; `str.strptime` hardcodes
  `exact=false`, matching the format anywhere in the text (`2100-02-29` parses as year 0000), so
  temporal parsers check that the value prints back to its text
  (`typescript/table/actions/column/types/-shared/temporal.ts`): a custom `temporalFormat` through
  the normalising `keepExactTemporal`, the zero-padded default patterns through the plain compare
  of `keepExactDefaultTemporal`, which is about a second cheaper on 1GB of date-times. A value
  with a UTC offset (`date-time`, `time`) cannot print back once converted to UTC, so
  `parseOffsetTemporal` cuts the offset off with a regex built from the pattern, checks the local
  rest, then applies the offset; a naive Datetime dtype has `timeZone: ""`, not undefined
  (`getTypeTimeZone`); `scanCSV` takes a one-character
  `eolChar`; an eager `pl.DataFrame(rows)` ignores `schemaOverrides`, and JS numbers load as
  Float64
- nodejs-polars has no `sqlExpr`: a SQL condition joins a frame as
  `SQLContext({ self }).execute("SELECT *, (…) AS … FROM self")`, and `execute` returns a frame
  wrapping a SQL error in `_ldf` instead of throwing it — check `result._ldf instanceof Error`

## MCP server

- Build a tool with `createTool` (`mcp-server/helpers/tool.ts`): `execute` takes typed input and is what tests call; `handle` is what `createServer()` registers. A throw reaches the client as `isError` with its message — never return an error object, which the output schema turns into an unreadable ZodError
- **One call must not flood the client's context.** A tool returning a list takes a `limit`/`maxErrors` input (default 100, max 1000), fetches one item more, and cuts it with `limitItems`/`limitReport` (`helpers/limit.ts`), which also trims to the 50k-character budget and sets `truncated`; any other response over the budget fails. A tool returning file content caps it in the input schema instead: `previewFileTool` takes `maxLines` (max 200) and `maxBytes` (max 20,000), while the CLI stays uncapped
- `validateTable` caps cell errors with one global head but splits `maxErrors` across the row key checks by column count, so `validateTableTool` asks for `(maxErrors + 1) × columns`; otherwise one bad key never shows truncation
- Nothing may write to stdout — it is the stdio transport; `-test/server.unit.ts` asserts a call prints nothing

## Twins

Every behaviour here has a twin in `python/`, and a change lands in both in the same branch,
with every other surface that shows it (root `AGENTS.md`, **Terminology**). Find the twin by
name, not by path:

- `loadTable` ↔ `load_table`, `-test/` ↔ `_test/`, `index.ts` ↔ `__init__.py`, an entity folder
  `tableSchema/` ↔ `table_schema/`
- Package layouts drift: `extension`'s renderers are Python's
  `python/library/fairspec_library/actions/table_schema/render.py`, and the CLI's
  `typescript/terminal/commands/tableSchema/infer.ts` is Python's
  `python/terminal/fairspec_terminal/commands/table/infer_schema.py` (the command,
  `table infer-schema`, is the same)
- The Python MCP server registers every tool under its `name` here, with the same input names
  and limits; renaming one renames it for both
- **A new CLI command or MCP tool is listed by hand outside the code**: its verb in the
  entity's `verbs` on `.livemark/routes/cli-tool.tsx`, a card (and the spelled-out count) on
  `.livemark/routes/mcp-server.tsx`, and a row in the `.skills/fairspec-*` skill that covers the task
- A behaviour both share gets a `testsuite/` case, which compares both outputs exactly
  (**`testsuite-development`**)

## Specs

- Unit tests are Vitest via vite-plus, named `*.unit.ts`, in collocated `-test` folders
- Import test symbols from `vite-plus/test`, not `vitest`
- Don't add useless comments like "Arrange", "Act", "Assert"
- **The suite must pass on Windows — CI runs `pnpm -C typescript test:unit` there.** Never assert on a raw path string: `readlink`, `join` and `import.meta.dirname` all speak the platform separator, so `expect(target).toBe("../AGENTS.md")` passes locally and fails on Windows with a backslashed target. Compare paths only after `resolve()`-ing both sides, and match file content against `\r?\n`, never a bare newline literal.
- `library/-test/testsuite.unit.ts` runs the shared `testsuite/` suite; a behaviour both
  libraries share gets a `testsuite/` test there, not only a unit test here (**`testsuite-development`**)
- Network tests use `useRecording()` from `@fairspec/metadata/testing` (a relative import inside `metadata`), a Polly fetch recorder kept in `typescript/metadata/-test/recording.ts` — a subpath export, so Polly stays out of the barrel and its packages stay `devDependencies`. The HAR lands in `-test/fixtures/generated/<test-name>_<hash>/`. Renaming a `describe`/`it` renames that directory, so the old recording is orphaned and the next run silently re-records from the live network — check `git status` after renaming a recorded test.

## Releases

This section is the `typescript` release-please component: tags `typescript-vX.Y.Z`, its own
Release PR, published to npm by `release.yaml`. All packages are lock-stepped to one version
via `extra-files`. Only changes to the published packages use `feat` or `fix`; tests, the docs
in `typescript/project/`, READMEs and tooling are `chore` or `docs`. When in doubt, ask whether
the change alters what someone installing a package gets.
