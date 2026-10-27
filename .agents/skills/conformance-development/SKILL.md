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
- A dataset group takes `datapackage` instead of `dataset` to write a Frictionless package as
  `datapackage.json`. Its import is checked through `dataset-infer`/`dataset-validate`, its
  export through `dataset-copy`, which saves to `datapackage.json`: never compare a written
  `dataset.json`, whose `$schema` carries the release version
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
- Three runners agreeing is not the spec agreeing: a spec-decided case that every library fails
  is a divergence in all three, never an expectation to rewrite. When a bulk addition cannot
  fix them in the same branch, leave the case out and record it for a follow-up fix
- A fix that changes behaviour other unit tests asserted means updating those tests too. Check
  whether the other library's unit tests already encode the decision
- A check on an explicit `null` (`const: null`, a `null` in `enum`) reads the raw descriptor, as
  `inspect_table_schema_nulls` does: the Python and Rust models turn an explicit `null` into an
  absent field. A numeric keyword is tested against absent, never for truthiness: `maxLength: 0`
  is a constraint
- Polars' `strptime` (chrono) is looser than the spec in all three twins: it reads unpadded,
  signed and short years under `%Y-%m-%d` and `%S` 60 as a leap second. A default form is
  matched against its anchored pattern first, list items too, and a `temporalFormat` value goes
  through `reject_leap_second` (Python, Rust; TypeScript's `keepExactTemporal` round-trip catches it).
  Its pattern splits fields as chrono does: an unsigned `%Y` takes at most 4 digits, the rest 2.
  A Date pattern with time fields round-trips through a Datetime in TypeScript: a Date cannot
  print `%H`
- A `pattern` is checked for ECMA-262 syntax by one recognizer in all three twins:
  `jsonschema_regex::is_valid_ecma_regex` in Rust, its ports `isValidEcmaRegex` and
  `is_valid_ecma_regex` in `jsonSchema/regex.ts` and `json_schema/regex.py`. Never `re` or native
  `RegExp`: they disagree on `(?<name>…)`, `\p{L}`, `(?i)` and `\-`
- WKT and WKB cells follow GEOS (shapely in Python): TypeScript's `wkt.ts`/`wkb.ts` and Rust's
  `wkt.rs`/`wkb.rs` are one hand-written reader, so a fix lands in both; GEOS itself takes any
  WKB byte order but 0 as little-endian, which Python's `_skip_wkb_geometry` refuses, and a WKT
  type by prefix (`POINTX (1 2)`), which `_get_are_words_known` refuses. The
  GeoJSON cell schema, `schemas/geojson.json`, is one byte-identical file in all three twins
- A Polars native panic aborts the whole TypeScript worker, so every later test in the file goes
  unreported. Narrow the run with `-t "<operation>"` until the crash is fixed
- A whole number in a `number` column renders as `1.0` in Python and `1` in TypeScript, on every
  load path, so the Python runner's strict compare fails it. That is an open divergence, not
  a reason to drop the strict compare; until it is fixed, give `number` cases fractional values
