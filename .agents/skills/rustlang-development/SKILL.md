---
name: rustlang-development
description: Use when working in rustlang/ — the Cargo workspace porting Fairspec to Rust, a third twin (metadata so far). Covers commands, the crate layout mirroring Python, serde union conventions, offline tests, the testsuite runner and divergences.
---

# Rust

The `rustlang/` section is a Cargo workspace, a **third twin** of the libraries. The goal is a
Fairspec implementation for users outside Python and TypeScript — a standalone binary they can
wrap as a CLI. So far only `rustlang/metadata/` (`fairspec-metadata`, lib `fairspec_metadata`)
exists, ported from `python/metadata/`; nothing is published and it has no release component.

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

## Testsuite

`metadata/tests/testsuite.rs` is the third runner: it checks every file against
`testsuite/test-schema.json`, runs `data-validate-schema` and `dataset-list`, and lists every
other operation in `SKIPPED`. An unknown operation directory fails, so a new operation in
`testsuite/` lands here too — implemented or skipped (**`testsuite-development`**).
