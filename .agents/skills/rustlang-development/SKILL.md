---
name: rustlang-development
description: Use when working in rustlang/ — the Cargo workspace porting Fairspec to Rust, a third twin (metadata, dataset, table, library). Covers commands, the crate layout mirroring Python, serde unions, offline tests, the testsuite runner and divergences.
---

# Rust

The `rustlang/` section is a Cargo workspace, a **third twin** of the libraries. The goal is a
Fairspec implementation for users outside Python and TypeScript — a standalone binary they can
wrap as a CLI. `rustlang/metadata/` (`fairspec-metadata`), `rustlang/dataset/` (`fairspec-dataset`),
`rustlang/table/` (`fairspec-table`, on Polars) and `rustlang/library/` (`fairspec-library`) are
ported from their Python packages; the crates follow Python's package DAG. Nothing is published and the section has no release component.

## Commands

Run from the repo root; `pnpm -C rustlang <script>` calls `cargo` inside `rustlang/`.

- `pnpm -C rustlang test` — type + unit
- `pnpm -C rustlang check:types` — `cargo check --all-targets`
- `pnpm -C rustlang test:unit` — `cargo test` (unit tests and the testsuite runner)
- `pnpm -C rustlang check:lint` — `cargo fmt --check` and clippy with `-D warnings`
- `pnpm -C rustlang format` — `cargo fmt`
- `pnpm -C rustlang build` — `cargo build --release`

`rustlang/rust-toolchain.toml` pins **stable** (CI builds on stable on Linux, macOS and
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
- Unknown fields are dropped on parse, as in Python (TypeScript keeps them)
- Pydantic constraints (patterns, lengths, bounds) are not enforced by serde — the JSON Schema profiles enforce them
- `ColumnProperty` has a hand-written `Deserialize` that routes on base type and `format`, mirroring `actions/column/create.py`; never replace it with `#[serde(untagged)]`, which picks variants by order and loses the error
- A descriptor-or-path input takes `impl Into<PathOr<Descriptor>>`

## Behaviour

- JSON Schema: jsonschema-rs, Draft 2020-12, format assertion off, a registry of the bundled profiles under both `latest` and `FAIRSPEC_VERSION` URLs; its fancy-regex engine handles the ECMA lookahead in the `InternalPath` pattern. Error messages are jsonschema-rs wording; the testsuite ignores `message`
- `normalize_dataset` follows TypeScript and never re-parses its output into a `Dataset`: Python's re-validation rejects a schema path outside the cwd
- Local paths come out with `/` separators; Python's `urlsplit`/`urlunsplit` rules are reimplemented by hand
- HTTP goes through `fetch_remote_text` / `resolve_remote_url` in `actions/descriptor/load.rs` with the `fairspec-rust` user agent; under `cfg(test)` they refuse every host but 127.0.0.1, so tests never touch the network
- Slugify tables are generated from Python's and must stay byte-exact with both twins

## Dataset

- Sync API: streams are `FileStream = Box<dyn Read + Send>`; Python's thread pool is `helpers/concurrency.rs` (scoped threads per chunk, nested calls run inline)
- All dataset HTTP goes through `dataset/src/helpers/http.rs` (`send_http_request`, `load_remote_stream`) — the one place a host policy applies; under `cfg(test)` it refuses every host but 127.0.0.1, so remote tests use `helpers/test_server.rs` or a replayed cassette
- Temp files and folders are deleted at exit through `libc::atexit` (`helpers/cleanup.rs`), Python's `atexit`, unless `persist`. Every `tempfile::Builder` takes `.prefix("tmp")` like Python's `mkstemp`: the default `.tmp` makes a dot-leading file name that fails the dataset profile's path pattern
- `infer_textual` is "no control bytes and valid UTF-8": Python's charset-normalizer fallback can never answer `utf-8`/`ascii` for non-UTF-8 bytes (it names codecs `utf_8`), so no detector is needed
- Plugins implement `MetadataPlugin` (every hook defaults to `Ok(None)`) and `DatasetPlugin`

## Table

- `Table` is a polars `LazyFrame`; queries run on the default `collect()`, not py-polars'
  streaming engine. Polars failures map to `Error::Load` through `helpers::error::PolarsResultExt`
- Column normalization builds the same expressions as Python (`str().strptime` with `strict:
false`, `%#z`, `replace_strict`, `list().eval` over `col("")`); a scale-only `pl.Decimal` is
  `Decimal(38, 18)`
- shapely and isodate have no crates: WKT/WKB readers follow GEOS 3.13 and the duration parser
  ports `isodate.parse_duration`; both are checked against the venv on sample lists
- `stringify_number` renders an integral float as Python's `str(int(x))` at any magnitude
- Row key checks find a duplicate with `cum_count().over(key columns) > 1`, and the key probe
  records raw key values, not hashes: `is_first_distinct` and `hash` need polars features the
  crate does not enable. Nulls group as in `is_first_distinct`, so the same rows are flagged

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

## Testsuite

`library/tests/testsuite.rs` is the third runner; it lives in the highest crate and calls the
library API (`validate_table`, `query_dataset`, …) as the Python runner does. It checks every
file against `testsuite/test-schema.json` and runs every operation; an unknown operation directory
fails, so a new operation in `testsuite/` lands here too (**`testsuite-development`**).
