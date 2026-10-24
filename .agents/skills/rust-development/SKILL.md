---
name: rust-development
description: Use when working in rust/ — the Cargo workspace porting Fairspec to Rust, a third twin with its CLI and MCP server. Covers commands, the crate layout mirroring Python, serde unions, offline tests, the conformance runner and divergences.
---

# Rust

The `rust/` section is a Cargo workspace, a **third twin** of the libraries. The goal is a
Fairspec implementation for users outside Python and TypeScript — a standalone binary they can
wrap as a CLI. `rust/metadata/` (`fairspec-metadata`), `rust/dataset/` (`fairspec-dataset`),
`rust/table/` (`fairspec-table`, on Polars) and `rust/library/` (`fairspec-library`) are
ported from their Python packages; the crates follow Python's package DAG. `rust/terminal/`
(`fairspec-terminal`) is the CLI and `rust/fairspec/` the umbrella crate whose `fairspec` binary
runs it (`cargo build --release -p fairspec`). Nothing is published and the section has no release component.

## Commands

Run from the repo root; `pnpm -C rust <script>` calls `cargo` inside `rust/`.

- `pnpm -C rust test` — type + unit
- `pnpm -C rust check:types` — `cargo check --all-targets`
- `pnpm -C rust test:unit` — `cargo test` (unit tests and the conformance runner)
- `pnpm -C rust check:lint` — `cargo fmt --check` and clippy with `-D warnings`
- `pnpm -C rust format` — `cargo fmt`
- `pnpm -C rust build` — `cargo build --release`
- `pnpm -C rust build:dist` — the build to ship and measure: py-polars' wheel profile (`dist-release`: fat LTO, one codegen unit) on its pinned nightly with polars' SIMD kernels (`nightly` feature); needs `rustup toolchain install nightly-2026-09-01`
- `pnpm -C rust sync:schemas` — rewrite `mcp-server/schemas/` from Python's MCP server (**MCP server**)

`rust/rust-toolchain.toml` pins **stable** (CI builds on stable on Linux, macOS and
Windows); a local nightly can accept code CI rejects. rustfmt keeps its 4-space default, a
deliberate deviation from the 2-space rule, like Python.

## Layout

Mirror Python's concern-then-entity tree so twins are found by name:

- `metadata/src/models/<entity>.rs`, `models/column/<kind>.rs`, `models/error/`, `models/file_dialect/`, `models/datacite/`
- `metadata/src/actions/<entity>/<verb>.rs` — Python's `assert_.py` is `assert.rs`
- `actions/<entity>/_test/<verb>.rs`, declared `#[cfg(test)] mod _test;` in the entity's `mod.rs`; fixtures in `_test/fixtures/`
- `metadata/src/lib.rs` is the barrel: modules stay private, every public name gets its own explicit `pub use` line, never a glob
- `metadata/profiles/` is written by `pnpm sync:profiles` and embedded with `include_str!`; `metadata/src/settings.rs` holds `FAIRSPEC_VERSION`, which the sync rewrites

Function names are Python's verbatim (`validate_table_schema`); keyword arguments become
positional parameters (`Option` where optional). Fallible actions return the crate `Result`
(`Error` in `models/exception.rs`; `Error::Invalid` is Python's `FairspecException`).

## Models

- serde with camelCase on the wire and every `Option` skipped when `None`, so output equals Python's `model_dump(by_alias=True, exclude_none=True)`; `profile` is `$schema`
- Unknown fields are kept, as in both twins: every descriptor struct not flattened into another carries `#[serde(flatten)] extra: Map<String, Value>`. Validators type a descriptor only through `inspect_model` (`actions/descriptor/inspect.rs`, `serde_path_to_error`), never `from_value` after the profile
- Pydantic constraints (patterns, lengths, bounds) are not enforced by serde — the JSON Schema profiles enforce them
- `ColumnProperty` has a hand-written `Deserialize` that routes on base type and `format`, mirroring `actions/column/create.py`; never replace it with `#[serde(untagged)]`, which picks variants by order and loses the error
- A descriptor-or-path input takes `impl Into<PathOr<Descriptor>>`

## Behaviour

- JSON Schema: jsonschema-rs, Draft 2020-12, format assertion off, a registry of the bundled profiles under both `latest` and `FAIRSPEC_VERSION` URLs; its fancy-regex engine handles the ECMA lookahead in the `InternalPath` pattern. Error messages are jsonschema-rs wording; the conformance suite ignores `message`
- `normalize_dataset` follows TypeScript and never re-parses its output into a `Dataset`: Python's re-validation rejects a schema path outside the cwd
- Local paths come out with `/` separators; Python's `urlsplit`/`urlunsplit` rules are reimplemented by hand
- HTTP goes through `fetch_remote_text` / `resolve_remote_url` in `actions/descriptor/load.rs` with the `fairspec-rust` user agent; under `cfg(test)` they refuse every host but 127.0.0.1, so tests never touch the network
- **Every ureq agent comes from `create_http_agent`** (`actions/remote/agent.rs`), and its errors go through `create_http_error`: under the `Public` host policy (`set_host_policy`, `FAIRSPEC_HOST_POLICY`; `serve_stdio` sets it) the agent's resolver refuses non-public addresses on every hop and the proxy is off. The resolver is ureq's semver-exempt `unversioned` API, so `ureq` is pinned exactly (`=3.5.0`) in metadata and dataset; bump it on purpose and re-run the agent tests. The policy is process-wide, so a test that sets it lives in its own `tests/*.rs` binary, never among the parallel unit tests
- Slugify tables are generated from Python's and must stay byte-exact with both twins

## Dataset

- Sync API: streams are `FileStream = Box<dyn Read + Send>`; Python's thread pool is `helpers/concurrency.rs` (scoped threads per chunk, nested calls run inline)
- All dataset HTTP goes through `dataset/src/helpers/http.rs` (`send_http_request`, `load_remote_stream`), which builds its agents with metadata's `create_http_agent`; under `cfg(test)` it refuses every host but 127.0.0.1, so remote tests use `helpers/test_server.rs` or a replayed cassette
- Temp files and folders are deleted at exit through `libc::atexit` (`helpers/cleanup.rs`), Python's `atexit`, unless `persist`. Every `tempfile::Builder` takes `.prefix("tmp")` like Python's `mkstemp`: the default `.tmp` makes a dot-leading file name that fails the dataset profile's path pattern
- `infer_textual` is "no control bytes and valid UTF-8": Python's charset-normalizer fallback can never answer `utf-8`/`ascii` for non-UTF-8 bytes (it names codecs `utf_8`), so no detector is needed
- Plugins implement `MetadataPlugin` (every hook defaults to `Ok(None)`) and `DatasetPlugin`

## Table

- polars is patched (`[patch.crates-io]` in `rust/Cargo.toml`) to the commit py-polars 2.0.0
  was built from (`pl.show_versions()` prints it as `Build:`), with polars' own `hashbrown`
  fork, which a dependency's `[patch]` never passes on. crates.io 0.55.2 was two months and 661
  commits older; with `build:dist` on top, Rust went from 1.5x Python's time to parity. Bump both revs with the py-polars the Python
  section pins, so both twins run the same engine
- `rust/.cargo/config.toml` gives x86-64 py-polars' wheel CPU features (AVX2, FMA, BMI2:
  Haswell and later), about 20% at 10GB; py-polars' `performant` feature is on too
- `Table` is a polars `LazyFrame`. Wherever Python passes `engine=QUERY_ENGINE`, Rust calls
  `collect_table()` / `sink_table()` (`helpers::table::LazyFrameExt`, the crate's `streaming`
  feature): the default `collect()` is the in-memory engine, which held 2.8GB per GB of CSV
  and cannot validate 10GB under a 10GB cap. Small head reads stay on `collect()`, as in
  Python. Polars failures map to `Error::Load` through `helpers::error::PolarsResultExt`
- A CSV sink sets `check_extension: false`, or a `.tsv` path fails a check py-polars' `sink_csv` skips
- Column normalization builds the same expressions as Python (`str().strptime` with `strict:
false`, `%#z`, `replace_strict`, `list().eval` over `col("")`); a scale-only `pl.Decimal` is
  `Decimal(38, 18)`. `withText` runs first, through `strip_number_text` (`_shared/text.rs`,
  twin of Python's `_shared/text.py` and TypeScript's `-shared/text.ts`), before group and
  decimal characters are handled; its patterns avoid lookaround, which no twin's polars regex has
- shapely and isodate have no crates: WKT/WKB readers follow GEOS 3.13 and the duration parser
  ports `isodate.parse_duration`; both are checked against the venv on sample lists
- `stringify_number` renders an integral float as Python's `str(int(x))` at any magnitude
- Row key checks find a duplicate with `cum_count().over(key columns) > 1`
  (`is_first_distinct` needs a feature the crate does not enable); nulls group as in
  `is_first_distinct`, so the same rows are flagged; a key with any null cell is never flagged
  (`create_key_presence_expr`). The key probe hashes inside polars
  (`row_hash`) before its callback, as Python does, so it keeps 8 bytes per key, not the keys

## Library

- `SYSTEM` is Python's `system`: a `LazyLock<RwLock<System>>` whose `register` inserts at the
  front, so xlsx is asked first and ckan last. `SystemPlugin` stores each plugin by its widest
  trait (`Metadata`/`Dataset`/`Table`) and stands in for Python's `isinstance` checks
- `lib.rs` re-exports `fairspec_metadata`, `fairspec_dataset` and `fairspec_table` whole where
  Python star-imports them, then lists its own names one per line
- A `Dataset | Descriptor | str` source is `impl Into<DatasetSource>`; Python's `ValueError` is
  `Error::Invalid` with no report
- The render actions only ask the plugins, as in Python, and no plugin renders yet, so they
  return `None`
- `validate_dataset` keeps the inferred resource names on the dataset, so foreign key errors carry
  them too (Python mutates the same resource objects). It sets `resourceName` on every error by
  going through JSON

## MCP server

- `mcp-server` serves the twelve tools over rmcp stdio; `serve_stdio()` blocks on its own tokio
  runtime and runs the sync library through `spawn_blocking`. Nothing may write to stdout:
  `mcp-server/tests/stdout.rs` runs without the test harness, which would capture a stray
  `println!`, and fails on any byte written to fd 1 during a tool call
- The tool list is Python's, byte for byte: `mcp-server/schemas/defs.json` holds the shared
  `$defs` and `tools.json` each schema with `$defs` as a list of names, which
  `helpers/schema.rs` puts back in place. **Run `pnpm -C rust sync:schemas` after any
  change to a Python MCP tool or to a model its schemas reach** — part of the twin rule; a
  second run leaves no diff
- Tool inputs are serde structs with the wire names (`maxErrors`); bounds the schema states
  are checked by hand (`tools/_shared/schemas.rs`). A failure is `isError` with
  `Error executing tool <name>: <message>`, as in Python

## Terminal

- Built on the `fireargs` crate, the Rust twin of Python's: `program.rs` mounts the same
  routes as Python's `program.py`, and each command is `fireargs::command()` over a
  `#[fireargs::input]` struct (shared blocks in `params/`, mixed in with `#[fireargs(flatten)]`)
  and a typed output from `models/output.rs`. Field names, docs, defaults and choices follow
  Python's params, so `--llms` (generated by fireargs from schemars) carries the same tool
  names, input keys and required fields; check with a `jq` diff against `fairspec --llms`
- Commands write human output through `Session` on the shared `Console` (`Arc<Mutex>`, since
  handlers are `Send + Sync`); `run_program` hands fireargs the same writers, so tests see one
  stream. A failure, or an invalid report, returns `Exit`: `run_program` prints its `data`
  (the `--json` result) and exits 1. Never add a `render_error` hook: it would also take
  fireargs' input errors and lose their commander wording
- `apply_layout` gives every route click's help (usage first, 80 columns, no long-about
  spacing, `--help` "Show this message and exit."). It allows hyphen-leading values only on
  single-value options: on a variadic or optional value (`--json [value]`) it swallows the
  next flag. `-v, --version` is answered before fireargs, as Python's bare version
- `--json` output is fireargs' (`serde_json::to_string_pretty`); `helpers/json.rs` (Python's
  `json.dumps`) is only for human output. Frames become records with pydantic's JSON-mode
  temporal text (`helpers/frame.rs`)
- A field with `parse` takes one value, so a repeatable option (`--check`) is a plain `Vec`
  validated in its handler. fireargs exits 1 on every usage error, where click exits 2
- `script` commands cannot embed Python's REPL: each line runs as SQL against the table or dataset
- Spawned-process tests in `fairspec/tests/` read the child's stdout on a thread with a timeout and
  close stdin, so a stuck `fairspec mcp` fails the test instead of hanging it

## Conformance

`library/tests/conformance.rs` is the third runner; it lives in the highest crate and calls the
library API (`validate_table`, `query_dataset`, …) as the Python runner does. It checks every
file against `conformance/test-schema.json` and runs every operation; an unknown operation directory
fails, so a new operation in `conformance/` lands here too (**`conformance-development`**).
