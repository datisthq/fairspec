---
name: conformance-development
description: Use when working in conformance/ — the language-neutral conformance test suite both libraries run. Covers the JSON-Schema-Test-Suite file format, the operations, the runners, and fixing a failing test in the library rather than the runner.
---

# Conformance

The `conformance/` section is pure data: JSON test files that the TypeScript and Python
libraries both run. It has no `package.json`, ships nothing, and is not a release component.
It is how a pair of twins proves it agrees: a behaviour both libraries share gets its case here.
`conformance/README.md` is the format reference; this skill is how to work with it.

## Commands

- `pnpm -C typescript test:unit library/-test/conformance.unit.ts` — the TypeScript runner (build first:
  `pnpm -C typescript build`, since the runner imports sibling packages from their `build/`)
- `pnpm -C python test:unit library/fairspec_library/_test/conformance_unit.py` — the Python runner
- A section-wide `pnpm -C typescript test:unit` or `pnpm -C python test:unit` runs it too. CI runs both runners
  on Linux, macOS and Windows whenever `conformance/**` changes
- `pnpm -C rust test:unit` — the Rust runner, `rust/library/tests/conformance.rs`, which runs
  every operation

## Format

- The layout follows JSON-Schema-Test-Suite: `tests/<operation>/<feature>.json`, each an array
  of groups `{description, …subject, tests}`, each test `{description, data, outcome}`
- The subject (`dataset`, `tableSchema`, `fileDialect`, …) lives on the group, never on a test:
  a test varies only `data`. Two descriptors to compare are two groups
- A test file is a top-level array, so it cannot carry `$schema`. Both runners validate every
  file against `conformance/test-schema.json` instead; a malformed file fails as a test
- An operation directory is named after the CLI command both libraries share, so
  `table infer-schema` becomes `table-infer-schema`; never a function name from either language
- `data` as a string is the exact file content. Put `\r\n` in the JSON string for CRLF; the
  repo's `eol=lf` never touches it, so hashes and byte counts hold on Windows
- A file with only a header row is ambiguous to the dialect sniffer, which runs only for a
  resource without a dialect. When a test needs an empty table, give it any `fileDialect`
  (`{ "format": "csv" }`): a declared dialect is never sniffed, its absent properties take the
  spec's defaults
- The only runner-side rules: `data`/`metadata`/`cell/json` errors drop `message` (validator
  text, not normative), and temporal columns render as ISO text through the library's table
  query, and `data-validate-schema` compares the report without its parsed `dataSchema`.
  Anything else that differs is a library divergence
- Binary inputs are tiny files written once by the Python library and embedded as base64; say
  how in the group's `comment`. A dataset test can only hold files at its top level, since the
  runners create no directories

## Adding a test

1. Find the expected value in the spec (`standard/specs/`), not in either library's output. The
   spec now pins error order (`table-schema.md` and `dataset.md` "Validation"), null foreign keys,
   exact temporal parsing and RFC 6901 pointers. When it is silent and the answer is normative,
   ask, then write the decision into the spec as `fix(standard)`; when it is only library shape
   (inferred dialects, data-schema inference, error field rendering), follow TypeScript and say so
   in the group's `comment`
2. Link the spec section in `specification` when one decides the outcome
3. Run both runners. A test that fails on one side is a divergence to fix in that library

## Adding an operation

Add the directory and a function to the `OPERATIONS` map in **both**
`typescript/library/-test/conformance.unit.ts` and
`python/library/fairspec_library/_test/conformance_unit.py`, under the same name. Each runner also
checks that every operation directory is one it knows, so an operation added to only one runner
fails the other. The Rust runner checks the same, so a new operation also goes into its
`OPERATIONS` or its `SKIPPED` list. Document the operation in the table in `conformance/README.md`.

## When a test fails

- **Never loosen a runner** to get a test passing: no sorting, no number coercion, no subset
  matching. The runners only serialise to JSON. The Python runner also compares
  `json.dumps(..., sort_keys=True)`, because `1 == 1.0 == True` in Python
- Fix the library that disagrees with the spec, in its own `fix(typescript)` / `fix(python)`
  commit together with the test. The suite itself is `chore`
- A fix that changes behaviour other unit tests asserted means updating those tests too. Check
  whether the other library's unit tests already encode the decision
- A Polars native panic aborts the whole TypeScript worker, so every later test in the file goes
  unreported. Narrow the run with `-t "<operation>"` until the crash is fixed
