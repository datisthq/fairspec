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
   spec now pins error order (`table-schema.md` and `dataset.md` "Validation", including the
   keyword order of several failures on one cell), null foreign keys, typed key comparison, the
   number grammar and int64 range, RFC 3339 temporal defaults, the string formats, list splitting,
   CSV/TSV row structure, blank lines and the BOM, `headerRows`, `columnNames`, object-row key
   order, invalid dialects and RFC 6901 pointers. When it is silent and the answer is
   normative, ask, then write the decision into the spec as `fix(standard)`; when it is only
   library shape (inferred dialects, data-schema inference, error field rendering), follow
   TypeScript and say so in the group's `comment`
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
  signed and short years under `%Y-%m-%d` and `%S` 60 as a leap second anywhere. A default form
  is uppercased and matched against its anchored pattern first, list items too, through one
  function per type (`parse_default_temporal` and its twins): its second `60` is read as `59`
  and kept only at 23:59 in UTC. A `temporalFormat` value goes through `reject_leap_second`
  (Python, Rust; TypeScript's `keepExactTemporal` round-trip catches it).
  Its pattern splits fields as chrono does: an unsigned `%Y` takes at most 4 digits, the rest 2.
  A Date pattern with time fields round-trips through a Datetime in TypeScript: a Date cannot
  print `%H`
- A `pattern` is checked for ECMA-262 syntax by one recognizer in all three twins:
  `jsonschema_regex::is_valid_ecma_regex` in Rust, its ports `isValidEcmaRegex` and
  `is_valid_ecma_regex` in `jsonSchema/regex.ts` and `json_schema/regex.py`. Never `re` or native
  `RegExp`: they disagree on `(?<name>…)`, `\p{L}`, `(?i)` and `\-`
- The URL, email, duration, hex and base64 grammars are one regex each in every twin's
  `settings` (`URL_REGEX` and its kin), byte-identical and spelled in ASCII classes: `\d` and
  `(?i)` match non-ASCII in Polars' engine. Inference reuses them, so an inferred format accepts
  its data
- WKT and WKB cells follow GEOS (shapely in Python): TypeScript's `wkt.ts`/`wkb.ts` and Rust's
  `wkt.rs`/`wkb.rs` are one hand-written reader, so a fix lands in both; GEOS itself takes any
  WKB byte order but 0 as little-endian, which Python's `_skip_wkb_geometry` refuses, and a WKT
  type by prefix (`POINTX (1 2)`), which `_get_are_words_known` refuses. The
  GeoJSON cell schema, `schemas/geojson.json`, is one byte-identical file in all three twins
- Polars' CSV `comment_prefix` drops prefixed rows before `skip_rows` or any numbering, while
  `headerRows` and `commentRows` count every row of the file: the CSV loaders renumber both past
  the prefixed rows above them before scanning (`renumberPrefixedRows` and its twins). Polars' native JSONL scan reads only
  object lines, so a default dialect peeks at the first item and takes array rows when it is one
- Polars reads a blank line as a row of nulls (a `,,` row looks the same afterwards) and a quoted
  `nullSequence` as null: the CSV loaders' `cleanCsvRows` (and twins) copy a file holding a
  blank line, or read with a `nullSequence`, with blank lines dropped and unquoted sequences
  emptied, in a quote-aware pass that leaves header and comment rows alone. A file without
  either is never copied; the copy drops a leading BOM, so a blank line or quote after it reads
  as at the start. Long rows load with `truncateRaggedLines` off. Several header rows are read
  headerless at the widest header row's width (inferred from the header rows only, never 100
  rows, or a long data row widens it); nodejs-polars refuses a first row shorter than that
  schema, so TypeScript truncates there and `cleanCsvRows` copies the file to fail on a wider
  data row. `validateTable` reports the long-row error as `resource/type`
  (`getIsLongRowError`), never another load failure
- Polars renames a repeated header name (`_duplicated_`) and an empty-named Series of a new
  frame (`column_0`): the CSV loaders read the header row as data to check it
  (`assertUniqueColumnNames`), and the sheet loaders rename by position after building the frame
- Polars orders the union of uneven JSON object rows by its own merge rule: the native JSON and
  JSONL reads keep their columns only when they equal the first row's keys, reordered to them,
  and otherwise fall back to decoding, which takes keys first-seen. Neither native reader skips
  a UTF-8 BOM, so a file starting with one is decoded too. Every JSON read, native or from
  decoded records, infers its columns from all rows: Polars' default 50 or 100 drops a key
  first seen later
- A row in both `headerRows` and `commentRows` is checked by `inspectFileDialect` (and twins) in
  `validateFileDialect` and in the dataset validator, at `…/commentRows`; `resolveFileDialect`
  asserts every inline dialect, so a load never reads a file through an invalid one. Profile
  errors for a dialect differ by validator, so those cases compare `valid` only
- Polars splits CSV cells on one byte, while a `delimiter` is one Unicode character: the CSV
  loaders' `swapDelimiter` (and its twins) copies the files with a multi-byte delimiter swapped
  for a control byte none of them holds, then swaps it back in every cell and name. The
  single-byte path never reaches it, a preview copies only its `previewBytes`; keep the
  candidate bytes and their order equal
- Cell checks build one bitmask column per column, bit n for check n of `CHECK_FUNCTIONS`
  (`check_cells` in Rust), whose order is the spec's keyword order: `type` and `missing`
  short-circuit, every failing constraint after them is reported. Keep the three lists equal
- `table-query` passes every query through `setNullsLast` and its twins, a token scanner (not
  a parser) that appends `NULLS LAST` to each ORDER BY item without a null order, since Polars
  sorts nulls first in descending order. Sorting a `withOrder` Categorical by category order
  is not implemented in any twin: it needs a polars `Enum`, which nodejs-polars lacks
- A Polars native panic aborts the whole TypeScript worker, so every later test in the file goes
  unreported. Narrow the run with `-t "<operation>"` until the crash is fixed
- Number and integer text passes the JSON number regex before any cast (`parseNumberText`,
  `parseIntegerText` and twins), since Polars' casts take `+1`, `007`, `NaN` and `inf`. An
  integer is read exactly from its digits and exponent, never through Float64, so
  `9223372036854775807.0` holds; the integer column, an integer Categorical and list items
  share it. A Categorical target is the label, so its numeric checks read the cell value again
  (`getNumericValue` and twins)
- Integer arithmetic over cell text (exponent plus digit counts) is clamped first: an exponent
  near the Int64 limit wraps, passes the range check and asks Polars for a huge padding, which
  aborts the whole query on one cell
- Schema inference patterns follow the same grammar as the cell reading, so an inferred schema
  accepts the data it came from: `00123` infers `string`, not `integer`
- A whole number in a `number` column renders as `1.0` in Python and `1` in TypeScript, on every
  load path, so the Python runner's strict compare fails it. That is an open divergence, not
  a reason to drop the strict compare; until it is fixed, give `number` cases fractional values
