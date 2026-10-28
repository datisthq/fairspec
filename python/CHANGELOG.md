# Changelog

## [0.3.0](https://github.com/datisthq/fairspec/compare/python-v0.2.0...python-v0.3.0) (2026-10-28)


### Features

* add file preview to see a file's raw source ([de998ed](https://github.com/datisthq/fairspec/commit/de998edacc445281821c9177cf8d074fb5539258))
* add local-date-time and local-time, and require a UTC offset in date-time and time ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* check the JSON type of object and array cells in Polars ([2cc0e6a](https://github.com/datisthq/fairspec/commit/2cc0e6a530d3042257bdb0c83e6fddf362f16f60))
* **desktop:** render custom check errors in the report ([1323ee6](https://github.com/datisthq/fairspec/commit/1323ee69eff741a8a5cf2da5ccd6fba5c98a8905))
* **desktop:** show column/extra errors ([4fe5d2f](https://github.com/datisthq/fairspec/commit/4fe5d2f038d775e16a09d257f57e7414e9c2ed1b))
* load and save Frictionless data packages ([462e5ed](https://github.com/datisthq/fairspec/commit/462e5ed867f1bccd3b58db9aee9730416f777927))
* **python:** accept a JSON Schema 2020-12 subset as a table schema ([4fe5d2f](https://github.com/datisthq/fairspec/commit/4fe5d2f038d775e16a09d257f57e7414e9c2ed1b))
* **python:** add an MCP server ([9a76889](https://github.com/datisthq/fairspec/commit/9a768895fc18298474159ad5f704637f1e364f43))
* **python:** add checks to validate_table, inspect_table, the CLI and the MCP tool ([1323ee6](https://github.com/datisthq/fairspec/commit/1323ee69eff741a8a5cf2da5ccd6fba5c98a8905))
* **python:** add query_dataset, query_tables, the dataset query command and queryDatasetTool ([77957e0](https://github.com/datisthq/fairspec/commit/77957e0aa73f014af0aed12d5295ae8de9e34463))
* **python:** add the columnProperties inference option and --column-properties flag ([9bf761c](https://github.com/datisthq/fairspec/commit/9bf761cf366c99b4ebf25d1ca5c25ae9c3194005))
* **python:** add the file preview command, preview_file and previewFileTool ([de998ed](https://github.com/datisthq/fairspec/commit/de998edacc445281821c9177cf8d074fb5539258))
* **python:** build the CLI with fireargs ([599627a](https://github.com/datisthq/fairspec/commit/599627ad72309bd88dd20fe830f542c2ac05df8c))
* **python:** check table keys in the cell checks' read ([522bc37](https://github.com/datisthq/fairspec/commit/522bc37adfbb23633192234b58442b3c260c861c))
* **python:** check the JSON type of object and array cells in Polars ([2cc0e6a](https://github.com/datisthq/fairspec/commit/2cc0e6a530d3042257bdb0c83e6fddf362f16f60))
* **python:** print every command's input and output schema with fairspec --llms ([599627a](https://github.com/datisthq/fairspec/commit/599627ad72309bd88dd20fe830f542c2ac05df8c))
* **python:** read date-time and time values with an offset as UTC and add the local types ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* **python:** read JSON arrays of objects with the native Polars reader ([c5441c4](https://github.com/datisthq/fairspec/commit/c5441c403f836c0e409edf9e53fc08a1690f62d1))
* **python:** run on polars 2.0 ([9c15b4e](https://github.com/datisthq/fairspec/commit/9c15b4ef82d55e049d2524889cb8e4677f0cd2a9))
* **python:** take a command's input as JSON with --json '&lt;value&gt;' ([599627a](https://github.com/datisthq/fairspec/commit/599627ad72309bd88dd20fe830f542c2ac05df8c))
* query a whole dataset with SQL ([#44](https://github.com/datisthq/fairspec/issues/44)) ([77957e0](https://github.com/datisthq/fairspec/commit/77957e0aa73f014af0aed12d5295ae8de9e34463))
* read and write spreadsheets with calamine and xlsxwriter ([82539a3](https://github.com/datisthq/fairspec/commit/82539a3a0435bf712544ea0bad0bcbf3e92a6fae))
* read JSON arrays of objects with the native Polars reader ([c5441c4](https://github.com/datisthq/fairspec/commit/c5441c403f836c0e409edf9e53fc08a1690f62d1))
* **standard:** accept a JSON Schema 2020-12 subset as a table schema ([4fe5d2f](https://github.com/datisthq/fairspec/commit/4fe5d2f038d775e16a09d257f57e7414e9c2ed1b))
* **standard:** define the temporalFormat specifiers and the offset rule per temporal type ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4)), closes [#42](https://github.com/datisthq/fairspec/issues/42)
* **standard:** mark removed datasets in a catalog with del ([9837583](https://github.com/datisthq/fairspec/commit/9837583ba1e5b4e0a073e093501dd5a2a2dfa3ad))
* support partial column properties in schema inference ([#41](https://github.com/datisthq/fairspec/issues/41)) ([9bf761c](https://github.com/datisthq/fairspec/commit/9bf761cf366c99b4ebf25d1ca5c25ae9c3194005))
* **typescript:** accept a JSON Schema 2020-12 subset as a table schema ([4fe5d2f](https://github.com/datisthq/fairspec/commit/4fe5d2f038d775e16a09d257f57e7414e9c2ed1b))
* **typescript:** add checks to validateTable, inspectTable, the CLI and the MCP tool ([1323ee6](https://github.com/datisthq/fairspec/commit/1323ee69eff741a8a5cf2da5ccd6fba5c98a8905))
* **typescript:** add queryDataset, queryTables, the dataset query command and queryDatasetTool ([77957e0](https://github.com/datisthq/fairspec/commit/77957e0aa73f014af0aed12d5295ae8de9e34463))
* **typescript:** add the columnProperties inference option and --column-properties flag ([9bf761c](https://github.com/datisthq/fairspec/commit/9bf761cf366c99b4ebf25d1ca5c25ae9c3194005))
* **typescript:** add the file preview command, previewFile and previewFileTool ([de998ed](https://github.com/datisthq/fairspec/commit/de998edacc445281821c9177cf8d074fb5539258)), closes [#43](https://github.com/datisthq/fairspec/issues/43)
* **typescript:** check the JSON type of object and array cells in Polars ([2cc0e6a](https://github.com/datisthq/fairspec/commit/2cc0e6a530d3042257bdb0c83e6fddf362f16f60))
* **typescript:** mark removed datasets in a catalog with del ([9837583](https://github.com/datisthq/fairspec/commit/9837583ba1e5b4e0a073e093501dd5a2a2dfa3ad))
* **typescript:** read date-time and time values with an offset as UTC and add the local types ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* **typescript:** read JSON arrays of objects with the native Polars reader ([c5441c4](https://github.com/datisthq/fairspec/commit/c5441c403f836c0e409edf9e53fc08a1690f62d1))
* **typescript:** read xlsx and ods with calamine-node and write them with wasm-xlsxwriter, dropping SheetJS ([82539a3](https://github.com/datisthq/fairspec/commit/82539a3a0435bf712544ea0bad0bcbf3e92a6fae))
* validate ckan, github and zenodo API responses at runtime ([#46](https://github.com/datisthq/fairspec/issues/46)) ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* validate Frictionless schemas and dialects and map their errors back ([c94e96f](https://github.com/datisthq/fairspec/commit/c94e96f4e264fd63b64b0c5883e8872065a5ffe0))
* validate tables against named custom checks ([1323ee6](https://github.com/datisthq/fairspec/commit/1323ee69eff741a8a5cf2da5ccd6fba5c98a8905))


### Bug Fixes

* a default date and a list date item must match ^\d{4}-\d{2}-\d{2}$ before strptime runs. reject_leap_second (python types/_shared/temporal.py, rust types/temporal.rs) nulls a temporalFormat value whose %S field reads 60. It splits fields the way chrono does (an unsigned %Y takes at most 4 digits), so compact patterns are covered too. TypeScript's keepExactTemporal round trip catches the same values. In TypeScript, a date pattern with time fields now round-trips through a Datetime. Every twin's duration parse rejects text ending in T (case-insensitive, after a final newline) before the parser reads it. ([5fa5090](https://github.com/datisthq/fairspec/commit/5fa50906d489ead67074c16a1e84c80db1db5b9d))
* a new cleanCsvRows / clean_csv_rows (csv helpers/rows.*) makes a quote-aware copy only when a file holds a blank line, the dialect has a nullSequence, or (TypeScript) there are several header rows. The copy drops a leading BOM and blank lines, empties unquoted data cells equal to the sequence, and in TypeScript fails on a data row wider than every header row. Truncation is off. Several header rows are read at the widest header row's width, inferred from the header rows only; nodejs-polars refuses a short first row against a schema, so TypeScript truncates there after the copy's width check. The header row is read as data and checked by assertUniqueColumnNames. columnNames with a header renames by position: more names add null columns, fewer is an error. Headerless columnNames use missing-columns insert. joinHeaderRows drops every row up to the last header row, errors on a header row past EOF (with or without columnNames), and keeps empty labels as empty names. JSON/JSONL native reads infer from all rows and keep their columns only when they equal the first row's keys (reordered to them), else they decode; decoded records infer from all rows too, and a BOM forces decoding. Sheet loaders error on a past-EOF header row, repeated names and short columnNames, and rename by position to keep empty names. A new inspectFileDialect / inspect_file_dialect in metadata is called from validateFileDialect and the dataset validator; resolveFileDialect asserts every inline dialect. validateTable reports the long-row error (getIsLongRowError) as resource/type, expectedResourceType table. Python's row cleaner rescans an unfinished row only after as many new bytes arrive, so a long quoted cell stays linear. Docs: the xlsx/ods 'Column Selection' sections in the Python, TypeScript and Rust docs now say columnNames names columns by position; the .skills/fairspec-validation resource/type row is updated; the conformance-development skill records the Polars workarounds. ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* accept file dialects with a custom format ([32659fe](https://github.com/datisthq/fairspec/commit/32659fe302e36aa0b488ba1272ca5ff79eaa160e))
* accept native Struct and List columns as object and array columns ([cd401d5](https://github.com/datisthq/fairspec/commit/cd401d5a520171040002061a8199f784bb0216de))
* add a conformance suite and fix the divergences it found ([#74](https://github.com/datisthq/fairspec/issues/74)) ([9e9ee2b](https://github.com/datisthq/fairspec/commit/9e9ee2b7be097e879197b38794a23eeb567086f6))
* all three twins read sample_bytes + 1 bytes. Only a sample that was cut short may end inside a character; a whole file that ends inside one is not textual. Before, Python and Rust rejected valid files that the sample cut mid-character, and TS accepted files that really ended truncated ([2a884f6](https://github.com/datisthq/fairspec/commit/2a884f6d7c485cd96aebb23be98f627ad4aad41e))
* bound MCP tool responses and surface tool errors ([45f6dd1](https://github.com/datisthq/fairspec/commit/45f6dd111805f06e4d9e5e0d092330fc14ce8d58))
* build each JSON Schema validator once instead of per cell ([7ac9d09](https://github.com/datisthq/fairspec/commit/7ac9d0920279fdcab644e5681a6603d6cd26e86a))
* check multipleOf exactly on the value as written ([cbdb9b5](https://github.com/datisthq/fairspec/commit/cbdb9b5fd6feb42e40d9a2953422ba625b4683b0))
* close the spreadsheet size guard's bypasses and halve its cost ([7a00362](https://github.com/datisthq/fairspec/commit/7a003629b2f10fdb4bcfcb9f16039a5f979cadde))
* collect the chunks, search only the new chunk for the newline, join once ([45a4d4f](https://github.com/datisthq/fairspec/commit/45a4d4fa249c01ef4f44862bc39da7059db31b35))
* compare keys as typed values, report every failing constraint in keyword order, sort nulls last ([86c1a56](https://github.com/datisthq/fairspec/commit/86c1a566ab334006299647b906f7da7918c075bf))
* compare numeric foreign keys by value, report other key type mismatches as metadata, check string constraints of row-parsed types and sort categoricals without withOrder by value ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* compare quantifier bounds as digit strings and saturate a backreference at u64::MAX, as the Rust crate does; TypeScript already compared them safely ([0922f38](https://github.com/datisthq/fairspec/commit/0922f3875a0da768941d2f2b8106cad3b514b579))
* copy the validated models and update only the path properties. TypeScript never re-validates there and was unaffected; Rust passes the same cases ([88c64ec](https://github.com/datisthq/fairspec/commit/88c64ecae5db44805f2589687028df3e5837f38d))
* count headerRows and commentRows on the file's own rows when commentPrefix also removes rows ([d053ede](https://github.com/datisthq/fairspec/commit/d053ede4eefaa39f66287655020ab295ee13919f))
* createLongRowError and its twins are worded so getIsLongRowError matches them, and validation reports the file as resource/type. decodeJsonBuffer skips lines made only of ' ', '\t' and '\r'. inspectFileDialect reports a metadata error at …/headerRows when a later row is smaller than the one before it. The CSV loaders read each part's own header rows as text, inserting missing columns because a first header row may be shorter. They compare that header with part 1's before the part is read, since joining header rows reads the part eagerly. The sheet loaders use getSheetHeader. The JSON loaders compare getHeaderLabelsFromRows across array-row parts before reading any row. Object-row parts are not compared and are concatenated diagonally and relaxed, so a column holding a number in one part and text in another loads as text, as it does from one file: how=diagonal_relaxed in Python, and in Rust a concat_diagonal_tables with to_supertypes, because its polars build has no diagonal_concat feature. ([7c167f5](https://github.com/datisthq/fairspec/commit/7c167f56106a5c16cb70c5b8e7db461fd39977a3))
* **desktop:** render the file/header error ([a9b6352](https://github.com/datisthq/fairspec/commit/a9b6352b7dba6544ca24767ea7bdfab968682059))
* **desktop:** show local date-times without shifting them into the viewer's time zone ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* each twin has one parse_default_temporal (parseDefaultOffsetTemporal/parseDefaultLocalTemporal in TS). It uppercases the text and matches the anchored ASCII pattern (table-schema.md#temporalformat: years 0000-9999, no 24:00, offsets ±23:59 with a colon). A second 60 is read as 59 and kept only when the value is 23:59 in UTC after the offset is applied. The list item casts reuse the per-type parsers. URL_REGEX (RFC 3986 URI, http/https in any case, non-empty host, ASCII; #url), EMAIL_REGEX (RFC 5321 Mailbox; #email), DURATION_REGEX (RFC 3339 Appendix A, letters in any case; #duration), HEX_REGEX (pairs; #hex) and BASE64_REGEX (#base64) are byte-identical strings in all three twins and use ASCII classes only. Inference reuses them. Appendix A is applied literally, so P1Y1D and PT1H1S are invalid. The now-unused isodate (Python) and temporal-polyfill (TypeScript) dependencies are dropped. Rust: the isodate port is removed. ([6c97483](https://github.com/datisthq/fairspec/commit/6c97483c0d88c03a00c00312d53db988c2bb8969))
* enforce maxLength 0 and reject bad required, null const/enum and invalid patterns in table schemas ([38bc5e9](https://github.com/datisthq/fairspec/commit/38bc5e9c2ccec53c2955a8086c92a95e3cb850d2))
* extend the testsuite and fix the divergences it found ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* fail on long JSON array rows, skip whitespace JSONL lines, reject unsorted headerRows and compare multipart headers under columnNames and in JSON array rows ([7c167f5](https://github.com/datisthq/fairspec/commit/7c167f56106a5c16cb70c5b8e7db461fd39977a3))
* FileDialect is BuiltinFileDialect | CustomFileDialect in all three twins. CustomFileDialect replaces UnknownFileDialect, requires format and refuses the built-in values, so a broken CSV dialect still fails as CSV. getSupportedFileDialect returns only built-in dialects and never infers a format over a declared custom one; the Rust MCP schemas are resynced. ([32659fe](https://github.com/datisthq/fairspec/commit/32659fe302e36aa0b488ba1272ca5ff79eaa160e))
* give columnNames' columns to an empty JSON array, with or without jsonPointer ([d053ede](https://github.com/datisthq/fairspec/commit/d053ede4eefaa39f66287655020ab295ee13919f))
* inspectForeignKeyTypes and its twins report a declared key column type mismatch as a metadata error at …/foreignKeys/&lt;i&gt;. Integer, number and decimal count as one family. Self-references are checked in inspectTableSchema. References to another resource are checked in the dataset descriptor validator, which reuses the table schemas its loop already loaded, so no path schema is fetched twice and no data is checked after the error. validateDatasetForeignKeys joins on cast key columns (Float64 when either side is a float, else Decimal(38,18); categoricals as String) and keeps the original values for the reported cells. A dtype mismatch it finds is reported the same way (inferred schemas, in-memory datasets), so the join never sees one. The text row scan computes the polars check expressions for enum/const/minLength/maxLength/pattern in its existing single collect and reports them only for cells that parse. Python and Rust load an integer categorical, or a string one with labels, without withOrder as an Enum in value order. TypeScript adds nodejs-polars as a direct dependency of @fairspec/library. ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* make the fraction an optional group, (?:\.[0-9]*)?, in both libraries; the accepted language is unchanged ([9661eb5](https://github.com/datisthq/fairspec/commit/9661eb52ebec5ee5e94367ad68d2daa454156ac3))
* multipleOf scales the cell text as written by 10^scale of multipleOf and checks it as a Decimal(38,0) integer, vectorised; scaled values over 37 digits keep float modulo, since a 38-digit product can overflow. TypeScript integer bounds below 2^63 go in as text cast to Int64 ([cbdb9b5](https://github.com/datisthq/fairspec/commit/cbdb9b5fd6feb42e40d9a2953422ba625b4683b0))
* number headerRows and commentRows before commentPrefix, keep columnNames on empty JSON and over header rows, read array JSONL rows ([d053ede](https://github.com/datisthq/fairspec/commit/d053ede4eefaa39f66287655020ab295ee13919f))
* parse WKT numbers in linear time ([9661eb5](https://github.com/datisthq/fairspec/commit/9661eb52ebec5ee5e94367ad68d2daa454156ac3))
* **python,typescript,rustlang:** block non-public hosts in remote fetches ([78f20cd](https://github.com/datisthq/fairspec/commit/78f20cd75c07525d62165d7e44883e74eb9aa565))
* **python:** accept a utc offset in a date-time without a temporalFormat ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** accept file dialects with a custom format ([32659fe](https://github.com/datisthq/fairspec/commit/32659fe302e36aa0b488ba1272ca5ff79eaa160e))
* **python:** accept native Struct and List columns as object and array columns ([cd401d5](https://github.com/datisthq/fairspec/commit/cd401d5a520171040002061a8199f784bb0216de))
* **python:** add the json pointer helpers the json table plugin imports ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** apply table-level missing values when validating ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** bind each step of the exact integer and full temporal readings as a struct field (the streaming engine has no CSE, so a step used twice was evaluated twice) ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))
* **python:** build each JSON Schema validator once instead of per cell ([7ac9d09](https://github.com/datisthq/fairspec/commit/7ac9d0920279fdcab644e5681a6603d6cd26e86a))
* **python:** check enum, const, minLength, maxLength and pattern on duration, wkt and wkb columns ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **python:** check multipleOf with exact decimal arithmetic ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** check self-referencing and unresolved foreign keys ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** check the hash shape before hashing, compare multipart headers, validate a data schema's $schema, $ref and $dynamicRef first, assert its known formats and point a false subschema at its member ([a9b6352](https://github.com/datisthq/fairspec/commit/a9b6352b7dba6544ca24767ea7bdfab968682059))
* **python:** compare integer, number and decimal foreign key columns by value and report any other key type mismatch as a metadata error at /foreignKeys/&lt;i&gt; ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **python:** drop blank lines, keep a quoted nullSequence as text, fail on long rows and repeated header names, consume every row up to the last header row, read past-EOF header rows and short columnNames as errors, skip a UTF-8 BOM and order JSON object columns first-seen ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **python:** drop the charset-normalizer fallback and dependency; textual is a strict UTF-8 decode ([2a884f6](https://github.com/datisthq/fairspec/commit/2a884f6d7c485cd96aebb23be98f627ad4aad41e))
* **python:** enforce maxLength 0 and check required, null values and patterns in validate_table_schema ([38bc5e9](https://github.com/datisthq/fairspec/commit/38bc5e9c2ccec53c2955a8086c92a95e3cb850d2))
* **python:** escape sqlite table and column identifiers ([347ec1b](https://github.com/datisthq/fairspec/commit/347ec1b8322bbf523e29004478e701fabf58ceca))
* **python:** exact integer reading uses Int128 arithmetic and runs only on cells with a point or an exponent; no zero-string padding ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))
* **python:** fail on a JSON array row longer than its header, skip whitespace-only JSONL lines, report headerRows out of ascending order, compare each multipart part's own header under columnNames and across JSON array-row parts ([7c167f5](https://github.com/datisthq/fairspec/commit/7c167f56106a5c16cb70c5b8e7db461fd39977a3))
* **python:** fail on a long row under several header rows, keep a JSON key first seen after the inferred rows, and fail on a past-EOF header row with columnNames ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **python:** find blank lines with C memmem through ctypes on a private mmap, one overlapping slice per core, GIL released. Falls back to mmap.find on Windows, below 16MB, or when the mapping cannot be writable ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))
* **python:** follow RFC 6901 in JSON pointers of schema errors ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** give a JSON file with a top-level object no table ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** hash table keys in polars before the batch callback ([f8b7793](https://github.com/datisthq/fairspec/commit/f8b7793fe9093b2d089ee561fce8105d43f7b26e))
* **python:** infer columns mixing integers and decimals as numbers ([2063df1](https://github.com/datisthq/fairspec/commit/2063df1cd0e3f04fa4eea683e6742a74d41692bb))
* **python:** infer data schemas in the TypeScript shape ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** infer zero-padded numbers as strings, so an inferred schema accepts its own data ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** load a withOrder categorical as a polars Enum in category order ([eb02068](https://github.com/datisthq/fairspec/commit/eb02068231043107f5846dbb2bac793ab21cbe53))
* **python:** load csv files with a crlf lineTerminator ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** load zenodo records whose license is an object ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **python:** make a list with an unparsable item a cell/type error ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** map the CSV writable only for the parallel blank-line search (review fix) ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))
* **python:** match trueValues/falseValues exactly, a declared list replacing its default; report a value in both lists as a table schema error ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** pass --sample-bytes on to infer-dialect, which dropped it ([599627a](https://github.com/datisthq/fairspec/commit/599627ad72309bd88dd20fe830f542c2ac05df8c))
* **python:** pass inference flags through the table commands ([9bf761c](https://github.com/datisthq/fairspec/commit/9bf761cf366c99b4ebf25d1ca5c25ae9c3194005))
* **python:** pass inference options from the dataset infer command ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** pass table schema options through dataset and resource inference ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** pass the resource, not its dump, to convert_resource_to_ckan ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **python:** read an integer categorical cell as an integer and check its minimum, maximum, exclusive bounds and multipleOf ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** read an integer column plain when its 100-row sample is plain; re-read all-exact (fresh key probes) only if a cell/type error cell is one the exact reading reads. Exact by construction: plain errors are a superset of exact ones and differ only on such cells, so a cut at max_errors without one is identical and the probes are complete only when no cut happens. The cost is a second full read for a file whose sample is plain but which later holds e.g. `5.0` ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))
* **python:** read csv header rows as the spec describes ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** read json tables as the spec describes ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** read lowercase t/z, leap seconds at 23:59 UTC and the RFC 3339 default forms, and check url, email, duration, hex and base64 by the spec grammars ([6c97483](https://github.com/datisthq/fairspec/commit/6c97483c0d88c03a00c00312d53db988c2bb8969))
* **python:** read number and integer cells by the JSON number grammar, integers exactly within int64 ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** read whole-second uppercase temporal defaults with the fast parser alone; the uppercase/leap-second reading runs only on the other cells ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))
* **python:** refuse a WKB byte order other than 0 or 1, and a WKT geometry type with trailing letters ([6834865](https://github.com/datisthq/fairspec/commit/683486572a49b5d68ebd2f7f2ad88bf387de5495))
* **python:** refuse an invalid inline dialect on load and report a row in both headerRows and commentRows at /commentRows ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **python:** reject a spreadsheet dialect that sets both sheetName and sheetNumber ([6de7a0d](https://github.com/datisthq/fairspec/commit/6de7a0dd559dc45aa8e6a7c054e169881dd3187b))
* **python:** render a whole-number bound without a trailing .0 ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** report a CSV/TSV row longer than the header as resource/type in validation ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **python:** report column errors before cell errors ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** report missing cells in duration, geometry and JSON columns ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** require every part of a textual resource to be utf-8 ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** require YYYY-MM-DD for dates and list items, reject %S 60 and PT/P1DT durations ([5fa5090](https://github.com/datisthq/fairspec/commit/5fa50906d489ead67074c16a1e84c80db1db5b9d))
* **python:** resolve the JSON Schema 2020-12 metaschema without the network ([037f133](https://github.com/datisthq/fairspec/commit/037f13386dbde45f54ff31514d56869c9458f1ac))
* **python:** return the descriptor url from save_dataset_to_ckan ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **python:** reuse the key-hash callback's all-false result ([ff7fc96](https://github.com/datisthq/fairspec/commit/ff7fc966dbfadbb8ed8bc81d8953a368f3704773))
* **python:** save the ckan descriptor as dataset.json so load finds it ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **python:** scan JSONL natively when its dialect is inferred ([86188f3](https://github.com/datisthq/fairspec/commit/86188f3428368ae2ed656984cbd02bab95701d29))
* **python:** skip the impossible type check of a column read as its source, but still project the column (long-row and UTF-8 reporting need it) ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))
* **python:** sort an integer or labelled string categorical column without withOrder by value ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **python:** split list cells literally: an empty item fails a typed list, an empty cell is an empty list ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** stop crashing on a regex quantifier or backreference of thousands of digits ([0922f38](https://github.com/datisthq/fairspec/commit/0922f3875a0da768941d2f2b8106cad3b514b579))
* **python:** the key hash probe reads the target column the check read already parses; the error bitmask keeps 0 as no error ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))
* **python:** the same limits, truncated flag and budget in the MCP server; pass every tool failure to the client with its message ([45f6dd1](https://github.com/datisthq/fairspec/commit/45f6dd111805f06e4d9e5e0d092330fc14ce8d58))
* **python:** transliterate non-ascii letters in inferred resource names ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** treat a column without a type as unknown ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** validate a dataset with schema paths outside the working directory ([88c64ec](https://github.com/datisthq/fairspec/commit/88c64ecae5db44805f2589687028df3e5837f38d))
* **python:** validate descriptors against their profile, then the core model ([b755f85](https://github.com/datisthq/fairspec/commit/b755f8514540a9b1f46c19fd4cfd3ce34963d185))
* **python:** validate empty JSON documents and report non-JSON data ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** validate_dataset loads a path through the dataset plugins, as TypeScript does ([462e5ed](https://github.com/datisthq/fairspec/commit/462e5ed867f1bccd3b58db9aee9730416f777927))
* read column values by the decided boolean, number, integer, categorical and list rules ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* read CSV, TSV, JSON and sheet rows by the decided dialect rules and refuse invalid dialects ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* read JSONL array rows when rowType is absent and the first item is an array ([d053ede](https://github.com/datisthq/fairspec/commit/d053ede4eefaa39f66287655020ab295ee13919f))
* read RFC 3339 temporal defaults and the decided duration, URL, email, hex and base64 grammars ([6c97483](https://github.com/datisthq/fairspec/commit/6c97483c0d88c03a00c00312d53db988c2bb8969))
* read the first JSONL line in linear time ([45a4d4f](https://github.com/datisthq/fairspec/commit/45a4d4fa249c01ef4f44862bc39da7059db31b35))
* refuse spreadsheets whose used range would exhaust memory ([4ea8d8a](https://github.com/datisthq/fairspec/commit/4ea8d8a49281cd9b0591bd69ddda0a08422d1231))
* refuse unknown WKB byte orders, partial WKT and GeoJSON features; allow position altitude ([6834865](https://github.com/datisthq/fairspec/commit/683486572a49b5d68ebd2f7f2ad88bf387de5495))
* reject loose dates, leap seconds and empty duration time parts ([5fa5090](https://github.com/datisthq/fairspec/commit/5fa50906d489ead67074c16a1e84c80db1db5b9d))
* report part headers, hash shape, data schema refs, dialects and formats as the dataset and data schema specs decide ([a9b6352](https://github.com/datisthq/fairspec/commit/a9b6352b7dba6544ca24767ea7bdfab968682059))
* report textual only for strict utf-8, allowing a cut character only at the end of a sample ([2a884f6](https://github.com/datisthq/fairspec/commit/2a884f6d7c485cd96aebb23be98f627ad4aad41e))
* **rust:** check enum, const, minLength, maxLength and pattern on duration, wkt and wkb columns ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **rust:** check the hash shape before hashing, compare multipart headers, validate a data schema's $schema, $ref and $dynamicRef first, and assert its known formats ([a9b6352](https://github.com/datisthq/fairspec/commit/a9b6352b7dba6544ca24767ea7bdfab968682059))
* **rust:** compare integer, number and decimal foreign key columns by value and report any other key type mismatch as a metadata error at /foreignKeys/&lt;i&gt; ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **rust:** drop blank lines, keep a quoted nullSequence as text, fail on long rows and repeated header names, consume every row up to the last header row, read past-EOF header rows and short columnNames as errors, skip a UTF-8 BOM and order JSON object columns first-seen ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **rust:** enforce maxLength 0 and check required, null values and patterns in validate_table_schema ([38bc5e9](https://github.com/datisthq/fairspec/commit/38bc5e9c2ccec53c2955a8086c92a95e3cb850d2))
* **rust:** fail on a JSON array row longer than its header, skip whitespace-only JSONL lines, report headerRows out of ascending order, compare each multipart part's own header under columnNames and across JSON array-row parts ([7c167f5](https://github.com/datisthq/fairspec/commit/7c167f56106a5c16cb70c5b8e7db461fd39977a3))
* **rust:** fail on a long row under several header rows, keep a JSONL key first seen after the inferred lines, and fail on a past-EOF header row with columnNames ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **rust:** infer zero-padded numbers as strings, so an inferred schema accepts its own data ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **rust:** load a withOrder categorical as a polars Enum in category order ([eb02068](https://github.com/datisthq/fairspec/commit/eb02068231043107f5846dbb2bac793ab21cbe53))
* **rust:** match trueValues/falseValues exactly, a declared list replacing its default; report a value in both lists as a table schema error ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **rust:** read an integer categorical cell as an integer and check its minimum, maximum, exclusive bounds and multipleOf ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **rust:** read number and integer cells by the JSON number grammar, integers exactly within int64 ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **rust:** refuse a WKB byte order other than 0 or 1 ([6834865](https://github.com/datisthq/fairspec/commit/683486572a49b5d68ebd2f7f2ad88bf387de5495))
* **rust:** refuse an invalid inline dialect on load and report a row in both headerRows and commentRows at /commentRows ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **rust:** report a CSV/TSV row longer than the header as resource/type in validation ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **rust:** require the format of a custom file dialect ([32659fe](https://github.com/datisthq/fairspec/commit/32659fe302e36aa0b488ba1272ca5ff79eaa160e))
* **rust:** require YYYY-MM-DD for dates and list items, reject %S 60 and PT/P1DT durations ([5fa5090](https://github.com/datisthq/fairspec/commit/5fa50906d489ead67074c16a1e84c80db1db5b9d))
* **rust:** sort an integer or labelled string categorical column without withOrder by value ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **rust:** split list cells literally: an empty item fails a typed list, an empty cell is an empty list ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **rust:** validate_dataset loads a path through the dataset plugins, as TypeScript does ([462e5ed](https://github.com/datisthq/fairspec/commit/462e5ed867f1bccd3b58db9aee9730416f777927))
* scan JSONL natively when its dialect is inferred ([86188f3](https://github.com/datisthq/fairspec/commit/86188f3428368ae2ed656984cbd02bab95701d29))
* sort a withOrder categorical column by category order ([eb02068](https://github.com/datisthq/fairspec/commit/eb02068231043107f5846dbb2bac793ab21cbe53))
* split CSV cells on a multi-byte delimiter ([50a77d9](https://github.com/datisthq/fairspec/commit/50a77d93268156af75052c6d48b0fa2273ce8fa6))
* **standard:** accept any value in a column without a type ([ea97c9f](https://github.com/datisthq/fairspec/commit/ea97c9f75ab7479ee103efe331487e770563f2b0))
* **standard:** allow and ignore unknown properties in every descriptor ([88c8ddd](https://github.com/datisthq/fairspec/commit/88c8ddd17724b96aa27267987b83b2000cb22795))
* **standard:** allow categories and withOrder only on categorical columns ([57f7228](https://github.com/datisthq/fairspec/commit/57f7228ce710a114f144d8167c0ffef3d093fa43))
* **standard:** define header carry-forward, object JSON files and multipart textual ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **standard:** enforce date-time, date, URI and JSON pointer shapes with patterns ([f32eb88](https://github.com/datisthq/fairspec/commit/f32eb88e8dd209730ecf31240513ce649bd9ba28))
* **standard:** forbid a data schema on multipart data ([de3aaef](https://github.com/datisthq/fairspec/commit/de3aaefdf8f95b9ca50016ad0e01eefc664cdd07))
* **standard:** give absent dialect and table schema properties defaults instead of inference ([148d1c8](https://github.com/datisthq/fairspec/commit/148d1c8b0f280888b1c9c6eae9daf1a7c5ca44fe))
* **standard:** improve standard pipeline ([#52](https://github.com/datisthq/fairspec/issues/52)) ([407d938](https://github.com/datisthq/fairspec/commit/407d9381adb1be365ac7243ece48410aca150ec7))
* **standard:** pin down table value checks, null foreign keys and error order ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **standard:** reject a scheme in an internal path and allow names starting with two dots ([c9b50e6](https://github.com/datisthq/fairspec/commit/c9b50e6499457e1c6af06604b4f2d0a37c274db6))
* **standard:** reject a spreadsheet dialect that sets both sheetName and sheetNumber ([6de7a0d](https://github.com/datisthq/fairspec/commit/6de7a0dd559dc45aa8e6a7c054e169881dd3187b))
* **standard:** require a file dialect's format ([c58361f](https://github.com/datisthq/fairspec/commit/c58361fb93873423bed4e9a3cce19a437f6339d2))
* **standard:** require a lowercase hex digest, and no integrity on inline data ([3a0af71](https://github.com/datisthq/fairspec/commit/3a0af7149e76774e8bcddc8bea2115be50bd8aea))
* **standard:** require table schema keys to name columns and primary key cells to be present ([01e5341](https://github.com/datisthq/fairspec/commit/01e53417711a305233041ee066882edc6978d9d5))
* **standard:** require unique resource names and literal missing value matching ([4d07c6d](https://github.com/datisthq/fairspec/commit/4d07c6da7ccc9bcd218bdf13b59d9f14acee9292))
* **standard:** strip only the prefix and suffix of a withText number ([3a92d39](https://github.com/datisthq/fairspec/commit/3a92d39fa1cabad9f3439bd7b089d6704874b769))
* **standard:** tighten single characters, sheet numbers, key columns and paths ([b87487d](https://github.com/datisthq/fairspec/commit/b87487dd84abcd71b325b1d8f3bc1294b56815e7))
* **standard:** validate descriptors against their profile, then the core model ([b755f85](https://github.com/datisthq/fairspec/commit/b755f8514540a9b1f46c19fd4cfd3ce34963d185))
* **standard:** write the decided table, dialect, dataset, data schema and catalog rules into the specs ([4a4f12c](https://github.com/datisthq/fairspec/commit/4a4f12c7e04d8b2b323cfb920a37739ae16d638e))
* the maxLength check now skips only when maxLength is absent. inspect_table_schema checks `required` against properties (metadata error at /required/&lt;i&gt;). It checks every pattern for ECMA-262 syntax (error at /properties/&lt;name&gt;/pattern) with one recognizer in all three twins. Rust uses jsonschema_regex::is_valid_ecma_regex; TS and Python use ports of it (isValidEcmaRegex, is_valid_ecma_regex). `re` and native RegExp disagree on named groups, \p{..}, (?i) and \-. New inspect_table_schema_nulls / inspectTableSchemaNulls reads the raw descriptor. It reports a null in const, default, enum or examples when the column type does not include null. Both validate_table_schema and dataset validation of inline schemas call it. Everything new is internal, not in the barrels. ([38bc5e9](https://github.com/datisthq/fairspec/commit/38bc5e9c2ccec53c2955a8086c92a95e3cb850d2))
* treat missing values as null in primary and unique key checks ([f8879dd](https://github.com/datisthq/fairspec/commit/f8879dda8116b39fbad29d740992c0e3495c003f))
* TS wkt.ts/wkb.ts now port Rust's wkt.rs/wkb.rs readers: every token parses fully, trailing tokens are refused and the byte order must be 0 or 1. wkx-ts is removed from typescript/table, and the lockfile only drops its subtree. Rust's reader refuses a byte order other than 0 or 1 at every nesting level. Python first runs `_skip_wkb_geometry`, which checks the byte order at every nesting level, and `_get_are_words_known`, which refuses a WKT word that is neither a number nor a keyword; then it calls shapely. geojson.json (one byte-identical copy per library): the oneOf is now geometry or GeometryCollection only, the feature defs are dropped, and a position is minItems 2 with numeric items (RFC 7946). ([6834865](https://github.com/datisthq/fairspec/commit/683486572a49b5d68ebd2f7f2ad88bf387de5495))
* **typescript:** a failed fatal TextDecoder decode now means not textual. The chardet fallback, which called overlong forms (c0 af) and CESU-8 surrogates (ed a0 80) UTF-8, is removed along with the inferTextual confidencePercent option and the chardet dependency. isBinaryFile is replaced by the twins' control-byte check (0-7, 14-31) ([2a884f6](https://github.com/datisthq/fairspec/commit/2a884f6d7c485cd96aebb23be98f627ad4aad41e))
* **typescript:** accept an empty cell in a column without a type ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** accept an integer column where the schema declares number ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** accept any value in a column without a type ([ea97c9f](https://github.com/datisthq/fairspec/commit/ea97c9f75ab7479ee103efe331487e770563f2b0))
* **typescript:** accept file dialects with a custom format ([32659fe](https://github.com/datisthq/fairspec/commit/32659fe302e36aa0b488ba1272ca5ff79eaa160e)), closes [#45](https://github.com/datisthq/fairspec/issues/45)
* **typescript:** accept file names starting with two dots and reject internal paths with a scheme ([c9b50e6](https://github.com/datisthq/fairspec/commit/c9b50e6499457e1c6af06604b4f2d0a37c274db6))
* **typescript:** accept native Struct and List columns as object and array columns ([cd401d5](https://github.com/datisthq/fairspec/commit/cd401d5a520171040002061a8199f784bb0216de))
* **typescript:** allow categories and withOrder only on categorical columns ([57f7228](https://github.com/datisthq/fairspec/commit/57f7228ce710a114f144d8167c0ffef3d093fa43))
* **typescript:** apply minimum and maximum only to numeric columns ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** apply table-level missingValues when validating ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** bundle the catalog and dataset profiles that allow unknown properties ([88c8ddd](https://github.com/datisthq/fairspec/commit/88c8ddd17724b96aa27267987b83b2000cb22795))
* **typescript:** bundle the file dialect profile that requires format ([c58361f](https://github.com/datisthq/fairspec/commit/c58361fb93873423bed4e9a3cce19a437f6339d2))
* **typescript:** cap queryTableTool/queryDatasetTool at limit records and validate tools at maxErrors (default 100, max 1000) within a 50k-character budget, with a truncated flag; move the MCP server from Mastra to @modelcontextprotocol/sdk so a failed tool returns its real message; throw the SQL error queryTables used to swallow ([45f6dd1](https://github.com/datisthq/fairspec/commit/45f6dd111805f06e4d9e5e0d092330fc14ce8d58)), closes [#71](https://github.com/datisthq/fairspec/issues/71)
* **typescript:** carry header labels forward across empty cells ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** check enum, const, minLength, maxLength and pattern on duration, wkt and wkb columns ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **typescript:** check key columns, foreign key lengths and null key cells ([01e5341](https://github.com/datisthq/fairspec/commit/01e53417711a305233041ee066882edc6978d9d5))
* **typescript:** check multipleOf with exact decimal arithmetic ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** check the hash shape before hashing, compare multipart headers, validate a data schema's $schema, $ref and $dynamicRef first, and assert its known formats ([a9b6352](https://github.com/datisthq/fairspec/commit/a9b6352b7dba6544ca24767ea7bdfab968682059))
* **typescript:** compare integer bounds past 2^53 exactly ([cbdb9b5](https://github.com/datisthq/fairspec/commit/cbdb9b5fd6feb42e40d9a2953422ba625b4683b0))
* **typescript:** compare integer, number and decimal foreign key columns by value and report any other key type mismatch as a metadata error at /foreignKeys/&lt;i&gt; ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **typescript:** drop blank lines, keep a quoted nullSequence as text, fail on long rows and repeated header names, consume every row up to the last header row, read past-EOF header rows and short columnNames as errors, skip a UTF-8 BOM and order JSON object columns first-seen ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **typescript:** drop the stray stdout log from inferTableSchemaTool ([9a76889](https://github.com/datisthq/fairspec/commit/9a768895fc18298474159ad5f704637f1e364f43)), closes [#48](https://github.com/datisthq/fairspec/issues/48)
* **typescript:** enforce maxLength 0 and check required, null values and patterns in validateTableSchema ([38bc5e9](https://github.com/datisthq/fairspec/commit/38bc5e9c2ccec53c2955a8086c92a95e3cb850d2))
* **typescript:** fail on a JSON array row longer than its header, skip whitespace-only JSONL lines, report headerRows out of ascending order, compare each multipart part's own header under columnNames and across JSON array-row parts ([7c167f5](https://github.com/datisthq/fairspec/commit/7c167f56106a5c16cb70c5b8e7db461fd39977a3))
* **typescript:** fail on a long row under several header rows, keep a JSON key first seen after the inferred rows, and fail on a past-EOF header row with columnNames ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **typescript:** give decimal columns a precision and scale so polars does not abort ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** infer columns mixing integers and decimals as numbers ([2063df1](https://github.com/datisthq/fairspec/commit/2063df1cd0e3f04fa4eea683e6742a74d41692bb))
* **typescript:** infer zero-padded numbers as strings, so an inferred schema accepts its own data ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **typescript:** keep declared textual and name resources by position in inference ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** let columnNames replace joined header rows instead of being joined to the second one ([d053ede](https://github.com/datisthq/fairspec/commit/d053ede4eefaa39f66287655020ab295ee13919f))
* **typescript:** load any JSON document as resource data ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** load CSV files with a crlf lineTerminator ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** load sqlite INTEGER columns as integers ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** make a list with an unparsable item a cell/type error ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** match integer categories against the text of a CSV cell ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** match trueValues/falseValues exactly, a declared list replacing its default; report a value in both lists as a table schema error ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **typescript:** parse dates, times and date-times exactly ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** pass inference flags through the table preview, describe, query and validate commands ([9bf761c](https://github.com/datisthq/fairspec/commit/9bf761cf366c99b4ebf25d1ca5c25ae9c3194005))
* **typescript:** read a declared dialect with defaults, sniffing only without one ([148d1c8](https://github.com/datisthq/fairspec/commit/148d1c8b0f280888b1c9c6eae9daf1a7c5ca44fe))
* **typescript:** read an integer categorical cell as an integer and check its minimum, maximum, exclusive bounds and multipleOf ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **typescript:** read jsonPointer as an RFC 6901 pointer ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** read lowercase t/z, leap seconds at 23:59 UTC and the RFC 3339 default forms, and check url, email, duration, hex and base64 by the spec grammars ([6c97483](https://github.com/datisthq/fairspec/commit/6c97483c0d88c03a00c00312d53db988c2bb8969))
* **typescript:** read number and integer cells by the JSON number grammar, integers exactly within int64 ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **typescript:** read the guarded archive from its central directory, as calamine does ([7a00362](https://github.com/datisthq/fairspec/commit/7a003629b2f10fdb4bcfcb9f16039a5f979cadde))
* **typescript:** read WKT and WKB with the GEOS-mirroring reader the Rust twin uses, dropping wkx-ts ([6834865](https://github.com/datisthq/fairspec/commit/683486572a49b5d68ebd2f7f2ad88bf387de5495))
* **typescript:** refuse a spreadsheet spanning more than 50M cells before calamine allocates it ([4ea8d8a](https://github.com/datisthq/fairspec/commit/4ea8d8a49281cd9b0591bd69ddda0a08422d1231))
* **typescript:** refuse an invalid inline dialect on load and report a row in both headerRows and commentRows at /commentRows ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **typescript:** reject a data schema on multipart data instead of checking only its first part ([de3aaef](https://github.com/datisthq/fairspec/commit/de3aaefdf8f95b9ca50016ad0e01eefc664cdd07))
* **typescript:** reject a spreadsheet dialect that sets both sheetName and sheetNumber ([6de7a0d](https://github.com/datisthq/fairspec/commit/6de7a0dd559dc45aa8e6a7c054e169881dd3187b))
* **typescript:** reject repeated resource names and check self foreign keys of unnamed resources ([4d07c6d](https://github.com/datisthq/fairspec/commit/4d07c6da7ccc9bcd218bdf13b59d9f14acee9292))
* **typescript:** reject unpadded list date items and a trailing duration time designator, and parse a date pattern with time fields ([5fa5090](https://github.com/datisthq/fairspec/commit/5fa50906d489ead67074c16a1e84c80db1db5b9d))
* **typescript:** report a column pattern the table engine cannot compile ([f32eb88](https://github.com/datisthq/fairspec/commit/f32eb88e8dd209730ecf31240513ce649bd9ba28))
* **typescript:** report a CSV/TSV row longer than the header as resource/type in validation ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **typescript:** report missing cells in duration, geometry and JSON columns ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** report table column errors before cell errors in schema order ([181ce13](https://github.com/datisthq/fairspec/commit/181ce13ed8819f1724570aa29eab0b02b664f872))
* **typescript:** require a lowercase hex digest, and no integrity on inline data ([3a0af71](https://github.com/datisthq/fairspec/commit/3a0af7149e76774e8bcddc8bea2115be50bd8aea))
* **typescript:** require every part of a textual resource to be utf-8 ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** return the descriptor url from saveDatasetToCkan ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **typescript:** save the ckan descriptor as dataset.json so load finds it ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **typescript:** scan JSONL natively when its dialect is inferred ([86188f3](https://github.com/datisthq/fairspec/commit/86188f3428368ae2ed656984cbd02bab95701d29))
* **typescript:** skip foreign keys with a null part ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** skip the type keyword the native kind check already holds ([7ac9d09](https://github.com/datisthq/fairspec/commit/7ac9d0920279fdcab644e5681a6603d6cd26e86a))
* **typescript:** split list cells literally: an empty item fails a typed list, an empty cell is an empty list ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **typescript:** strip only the text around a withText number ([3a92d39](https://github.com/datisthq/fairspec/commit/3a92d39fa1cabad9f3439bd7b089d6704874b769))
* **typescript:** tighten single characters, sheet numbers, key columns and paths ([b87487d](https://github.com/datisthq/fairspec/commit/b87487dd84abcd71b325b1d8f3bc1294b56815e7))
* **typescript:** validate a data schema declaring the JSON Schema 2020-12 dialect ([037f133](https://github.com/datisthq/fairspec/commit/037f13386dbde45f54ff31514d56869c9458f1ac))
* **typescript:** validate descriptors against their profile, then the core model ([b755f85](https://github.com/datisthq/fairspec/commit/b755f8514540a9b1f46c19fd4cfd3ce34963d185))
* **typescript:** write the zenodo license id, not its object, to rightsList ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))


### Performance Improvements

* check all columns of a table in a single scan ([181ce13](https://github.com/datisthq/fairspec/commit/181ce13ed8819f1724570aa29eab0b02b664f872))
* check table keys with a group-by probe ([fa1e2e0](https://github.com/datisthq/fairspec/commit/fa1e2e006c55d674f5ad6557a63774c21be43492))
* **python:** restore validation speed to the pre-regression level ([38b9bbe](https://github.com/datisthq/fairspec/commit/38b9bbe1b073be5e19e15ef94725106e321673ba))

## 0.2.0 (2026-09-10)

### Bug Fixes

- **metadata**: Resolve versioned profile paths and send a User-Agent ([`ddb5833`](https://github.com/datisthq/fairspec-python/commit/ddb583375c39a7a4809cfcc450ecd4a829450dde))

Two bugs on the same path, ported from the fairspec-typescript fix.

The registry keyed the five vendored profiles only on `https://fairspec.org/profiles/latest/<name>.json`, but every save_* action stamps a versioned $schema from FAIRSPEC_VERSION, and both lookups match the URL by exact string equality. So the library wrote a URL its own bundle could never match: every saved descriptor missed the cache and went remote.

That remote load then failed outright. _load_remote_descriptor passed a bare URL to urlopen, so requests carried the default Python-urllib agent, which fairspec.org (Cloudflare) answers with 403:

saved $schema: https://fairspec.org/profiles/0.5.0/dataset.json validate : HTTPError: HTTP Error 403: Forbidden

Bump FAIRSPEC_VERSION to 0.6.0 to match what the bundle already is -- the vendored dataset.json carries the object Integrity that shipped as spec 0.6.0 -- and derive the registry so each profile answers to both "latest" and the pinned version. Versions the bundle does not carry still go remote, which is correct: they have different rules.

Add a USER_AGENT setting and send it from both urlopen callers that target fairspec.org: the descriptor loader and resolve_basepath, which 403s the same way.

Collapse load_profile into load_json_schema, which already does registry-then-network plus caching, removing the duplicated lookup.

Every fixture and default used "latest", so nothing covered this. Add an offline save -> validate round-trip, registry resolution tests, and a User-Agent assertion; all fail without the fix.

- **table**: Emit tuple for nullable column types in sqlite plugin ([`717ede5`](https://github.com/datisthq/fairspec-python/commit/717ede5f8b5271cb3039a6cb8c50c5d12b9e62c9))

The sqlite table_schema conversion carried a copy of the shared _make_property_nullable helper with tuple changed to list, so it assigned ["string", "null"] to a field annotated as a tuple of literals. Pydantic models do not validate on assignment, so the bad value was stored silently and only surfaced later at model_dump() time as serializer warnings.

Extract the helper as a public set_property_nullable in fairspec_metadata and call it from both the shared polars inferrer and the sqlite plugin, so the two cannot diverge again. Add filterwarnings = ["error::UserWarning"] to fail the suite on any future occurrence.

Closes #2

- **table**: Tag cell and row errors instead of embedding a template ([`8bb9a49`](https://github.com/datisthq/fairspec-python/commit/8bb9a49942b2175579dff3a2f9b155ab13ba91ef))

A polars literal is broadcast to every row, so putting the JSON error template in the `then` branch materialized a full-length string column for each of the 14 cell checks. On a 500MB file that made one column's check cost ~2.7GB, and site B validates columns concurrently, so the cost was multiplied by the worker count and 500MB no longer fitted in 6GB.

Carry a UInt8 tag in the error column instead and resolve it back to the template in Python once the frame has been filtered to the reported errors. Row key checks own a single template each, so they only need a boolean flag and can read the template from the closure.

Isolated on the same file, 14 layers cost 2754MB with the JSON payload and 652MB with a short one; the layering itself is free. End to end, 500MB and 1GB validations that were OOM-killed under a 6GB cap now complete, and the JSON round-trip disappears from the hot path.

Output is unchanged: 1472 tests pass, and the invalid 500MB fixture still reports exactly its one error.

- **test**: Point fixture URLs at the current repo and path ([`7f09ffa`](https://github.com/datisthq/fairspec-python/commit/7f09ffad3b1fc8bfb3987e6e27d9dab0ca248c34))

Both specs fetch a fixture from the fairspec-typescript repo, still naming the fairspec org that was renamed to datisthq and a path that predates the -test folder layout. The csv spec ran against the live URL and had been failing with a 404.

VCR matches on URI, so the two prefetch cassettes are re-recorded.

### Chores

- Omit _test folders from coverage ([`ff3fff0`](https://github.com/datisthq/fairspec-python/commit/ff3fff015addc9b34f22c92ce76b0213230922fd))

The specs live inside the measured source packages, so until now they counted toward their own coverage. The _test convention makes them matchable by path for the first time.

- Rename test files to _unit.py to match typescript ([`fe4d303`](https://github.com/datisthq/fairspec-python/commit/fe4d30346752cb9b11c8ce8bc4877e0a2bc7c485))

The TypeScript repo names every test file &lt;module>.unit.ts. Rename the 139 Python specs from &lt;module>_spec.py to &lt;module>_unit.py, and the taskipy task from spec to unit so `uv run task unit` mirrors `pnpm unit`.

Python cannot use the literal .unit.py: a dot is not legal in a module name, and 132 of the specs import their subject relatively, so the package would be computed one level too deep and every `from ..load import` would break. AGENTS.md now records that alongside the existing note on why directories are _test and _shared rather than -test.

Also updates the gitleaks allowlist, which is keyed on the filename suffix and would otherwise have silently stopped exempting tests from the secret scan.

Test files are renames only, with no content change; all 1467 tests still pass and the 3 VCR cassettes are untouched, since their names key on the test class rather than the module.

- Update GitHub org to datisthq ([`9a868c7`](https://github.com/datisthq/fairspec-python/commit/9a868c7bf426a02e047344aa39633b9d0e4f7585))

- **deps**: Bump vcrpy in the uv group across 1 directory ([#18](https://github.com/datisthq/fairspec-python/pull/18), [`df3adeb`](https://github.com/datisthq/fairspec-python/commit/df3adeb4371e19ba53356f54c87a33ab4294bfd2))

Bumps the uv group with 1 update in the / directory: [vcrpy](https://github.com/kevin1024/vcrpy).

Updates `vcrpy` from 8.1.1 to 8.2.1
- [Release notes](https://github.com/kevin1024/vcrpy/releases)
- [Changelog](https://github.com/kevin1024/vcrpy/blob/master/docs/changelog.rst)
- [Commits](https://github.com/kevin1024/vcrpy/compare/v8.1.1...v8.2.1)

---
updated-dependencies:
- dependency-name: vcrpy dependency-version: 8.2.1

dependency-type: indirect

dependency-group: uv ...

Signed-off-by: dependabot[bot] &lt;support@github.com>

Co-authored-by: dependabot[bot] &lt;49699333+dependabot[bot]@users.noreply.github.com>

- **deps**: Migrate to polars 1.44 ([`dc69b6f`](https://github.com/datisthq/fairspec-python/commit/dc69b6f6867a0819f1a4ea6df26bb5476152e176))

polars 1.44 tightens the DataFrame constructor signature, so the decoded JSON buffer needs a cast at the call site. That is the only source change the upgrade needs — unlike nodejs-polars 0.26, the Python binding keeps the Expr.dt namespace unchanged.

Not a performance win. On the 1 GB validation benchmark it measured 31% slower than 1.38.1 (5.7s to 7.4s) for 18% less peak memory (2393MB to 1970MB), and it carries the scan_csv memory regression tracked in pola-rs/polars#27539. It also does not bring a usable streaming engine for this workload: CSV is not a streaming source.

- **metadata**: Track fairspec spec 0.6.1 ([`eb21f8c`](https://github.com/datisthq/fairspec-python/commit/eb21f8c58f27a03cdc79503209e1abb70add4925))

0.6.1 re-tags 0.6.0 with no rule changes -- the only difference is the version segment in dataset.json's internal $refs, which the bundled copy writes as "latest" and resolves through the registry either way. The vendored profiles are already content-identical to live 0.6.1, so this is the constant alone; the registry re-keys itself from it.

- **project**: Add a project package owning repo docs and the docs guard ([`6696bee`](https://github.com/datisthq/fairspec-python/commit/6696beed3c425512841a4fadf70ccc379a82ecf0))

The docs-consistency guard lived in fairspec/fairspec/_test/, inside the umbrella package whose entire source is __init__.py and main.py. It reads AGENTS.md, README.md and CONTRIBUTING.md and reaches up to the repo root, so it has nothing to do with the package hosting it.

Port the convention from fairspec-typescript: a directory that nothing depends on, holding the skills Claude Code reads through .claude/skills and the guard that checks this repo's prose. Deliberately not a uv workspace member and carrying no pyproject.toml -- it ships nothing, and a manifest would invite adding it to version_toml later.

skills/ carries a .gitkeep because git cannot commit an empty directory and the symlink would otherwise dangle on a fresh clone, failing the new assertion in CI while passing locally.

Also corrects a stale line: fairspec-typescript renamed agent to mcp-server, so the cross-repo note named a package that no longer exists.

### Documentation

- Port AGENTS.md structure from mycarro-next ([`b17675c`](https://github.com/datisthq/fairspec-python/commit/b17675c81f11a4f134c33fb23e1d3f0c53199789))

Rewrite AGENTS.md around the sections mycarro-next uses: Rules, Modules with the package DAG, Code structure including the new _test/_shared convention, Conventions, and a calibrated comment rule in place of a blanket ban.

Correct a claim that was never true: the Specs section asked for test_&lt;module>.py files, while pyproject.toml has always set python_files = "*_spec.py" and no file in the repo matched the documented pattern. The taskipy tasks for build, publish and coverage and the whole pnpm side — leaks, vulns, scan, deps, docs:start, docs:build — went undocumented.

CONTRIBUTING.md still cloned from github.com/yourusername, described tests as merely collocated, and gave a placeholder path as the single-test example.

- Update citation author email ([`526c8cd`](https://github.com/datisthq/fairspec-python/commit/526c8cd95031d1811478f56f23af06e5fc806e52))

- **project**: Move the documentation site into the project package ([`5bf0613`](https://github.com/datisthq/fairspec-python/commit/5bf061342efbadaa4cfb8cf5027e923ac2267cf2))

The livemark sources sat at the repo root alongside config. They belong with the package that owns documentation.

livemark.config.ts stays at the root -- livemark resolves its config from the cwd, and docs:build/docs:start run from there -- so only the include glob moves. README.md and CONTRIBUTING.md stay at the root too: the patches block keys off those filenames and GitHub resolves them there.

Every page carries an absolute path: in its frontmatter, so the published URLs are unchanged. The guard's package list drops "docs" for "project" now that docs/ is no longer a root directory.

### Features

- Run validation and inference in parallel ([`dd47e96`](https://github.com/datisthq/fairspec-python/commit/dd47e96b224a72531fa7a6c4eb2971b2aee76949))

The TypeScript codebase fans out concurrently in six places; the Python port turned every one into a sequential loop, and the workspace had no concurrency at all.

Add a shared bounded ThreadPoolExecutor in fairspec_dataset.helpers and restore all six sites: per-resource validate and infer, per-column and per-row-key checks, prefetch, and hash reads. A single `concurrency` option on InferTableSchemaOptions is inherited by LoadTableOptions and ValidateTableOptions, defaulting to os.cpu_count().

Report output is unchanged. Each task's results are collected separately and concatenated in input order, and the column and row-key loops are chunked so the existing max_errors early exit still applies. A 143KB report is byte-identical at concurrency 1, 4 and 12, and the whole suite passes with the default forced to 1.

Two deliberate differences from the TypeScript original:

- Nested fan-out runs inline. Submitting into a bounded pool from inside that pool deadlocks once every worker blocks on an inner future, so a thread-local marker makes inner calls run in the current worker. Total threads can never exceed the pool width; TypeScript instead applies its limit per call, allowing cpu^2 tasks in flight. - infer_hash parallelises the file read rather than only the stream open, since concat_file_streams reads them serially afterwards.

Threads rather than processes, so the remaining GIL-bound work scales for free once polars ships free-threaded wheels.

Closes #3

### Performance Improvements

- **table**: Cap column check concurrency at four ([`1e278d3`](https://github.com/datisthq/fairspec-python/commit/1e278d3b7a75347f56d3684ed6812574bc45ea8d))

Each column check scans the whole source, so peak memory grows with the number in flight while the speed stops improving well before it. On the NYC 311 sample, 1M rows across 41 columns, validation takes 6.8s in 384MB serially, 3.5s in 1GB at four checks, and 3.4s in 1.5GB at one per core: the last eight workers buy a tenth of a second for half a gigabyte.

Four is where the curve flattens. Wide tables are what expose this — the earlier 5 and 10 column benchmarks never had enough columns in flight to show it. Callers can still override with the concurrency option, and row key checks are left alone since a schema rarely has more than a couple.

- **table**: Collect checks with the streaming engine, restore concurrency ([`5cd6efc`](https://github.com/datisthq/fairspec-python/commit/5cd6efc65d70082affcff9cb3fca53e1f914970a))

The in-memory engine holds the whole source while it collects, so peak memory grew with the file and checks had to run one at a time. py-polars exposes the streaming engine on collect(), which keeps a bounded working set: the minimum memory a 1 GB validation survives stops tracking file size and sits at 384MB for 50MB, 500MB and 1GB alike, of which 128MB is just importing the library.

That makes per-core concurrency affordable again, so the two INSPECT_* limits added in fc463ea are gone and the checks run at cpu_count once more. Against that serial-and-in-memory baseline: 100MB 0.93s to 0.34s, 500MB 4.0s to 1.1s, 1GB 7.0s to 2.6s, all now fitting in a 1G cap where the pre-cap code needed more than 6G.

Applied to the collect sites that scan the whole source. The bounded head(sample_rows) and head(HEAD_ROWS) collects are left alone; they read a fixed number of rows and gain nothing.

Note this diverges from fairspec-typescript, which keeps its checks serial: nodejs-polars exposes streaming on collectSync only, so the same fix is not reachable there.

- **table**: Execute save plugins with the query engine ([`0aee3a5`](https://github.com/datisthq/fairspec-python/commit/0aee3a5367448e4eb7c335ce7235455b571d2fa3))

The parquet and arrow plugins already sink, so they only needed the engine passed. The csv plugin collected the whole frame and then wrote it, even though sink_csv accepts the same four options it was building — the local was already called sink_options — so it now sinks directly. Saving a 500MB CSV drops from a 1536MB floor to 384MB, the same bounded working set validation gets.

json, xlsx and sqlite have no polars sink: the first decodes the written text back to Python to reshape it, and the last two hand a materialized frame to openpyxl and sqlite3. They still collect, but with the streaming engine so the collect itself is bounded.

- **table**: Run column and row checks one at a time ([`fc463ea`](https://github.com/datisthq/fairspec-python/commit/fc463ead65bbb4f7d05a26cf170d757813ad02a9))

Every check builds its own LazyFrame over the source, and a LazyFrame caches nothing, so N checks means N independent scans of the same file. polars materializes a CSV in full however the query is collected, so peak memory was the file size times the number of checks in flight.

On a 1 GB file that was 12GB at one check per core, against 1.9GB serially. Time goes the other way, 5.0s to 7.0s, but the concurrency sweep flattened above two workers while memory kept climbing, so the parallelism was buying little of what it cost.

The two limits are separate settings: row-key checks are usually one or two tasks, columns are the many. Callers can still override with the concurrency option.

### Refactoring

- Complete the __init__.py package chain ([`73e2943`](https://github.com/datisthq/fairspec-python/commit/73e2943ae56d74d9621db6ec937133da3ab62879))

metadata carried exactly one __init__.py and relied on PEP 420 namespace packages; dataset was missing the markers under actions/. The other four packages already have them nearly everywhere.

pytest resolves a test module by walking up the __init__.py chain from the file, and stops at the first directory without one. With a gap, a spec one level below its subject resolves as a top-level module and a relative import of that subject reaches beyond the top-level package.

Prerequisite for moving specs into _test folders.

- Move unit tests into _test folders ([`683e171`](https://github.com/datisthq/fairspec-python/commit/683e171fce4b283c13d011039239ebc9d9f74d22))

Adopt the convention fairspec-typescript took from mycarro-next: unit tests and their fixtures move into a collocated _test/ directory, so the module surface reads without spec files interleaved.

TypeScript marks these folders with a leading -, which is illegal in a JS identifier and so can never be mistaken for an importable module. Python needs the marker to stay a valid identifier, because _shared is imported by its siblings, so it uses a single leading underscore instead.

Discovery is unaffected — pytest matches on the *_spec.py suffix, and both the fixture paths and the vcr_cassette_dir fixture derive from the test file's own directory, which now carries its fixtures with it.

dataset's conftest.py moves down from the package root to the one folder whose spec consumes it, taking the two cassettes with it.

plugins/xlsx/actions/table/test.py was production-located but read only by two specs; it moves into _test/ as test_data.py, named after its exports.

- **metadata**: Drop the dead format="" literal on the number base ([`f2f91a8`](https://github.com/datisthq/fairspec-python/commit/f2f91a8af3925d73e7b5b26f884e8a599666e34f))

BaseNumberColumnProperty was the only plain column base declaring a format, and it declared the empty-string literal that every other plain model had already migrated away from. NumberColumnProperty overrides it with Literal[None], so the field was shadowed and unreachable — the public surface already rejected format="".

Removing it makes the number base match the string, integer and object bases, which declare no format at all.

- **table**: Move action helpers into _shared folders ([`f332be3`](https://github.com/datisthq/fairspec-python/commit/f332be3fed6113d8497f8abce47f61ef0002905b))

Both helpers.py files sit inside action folders where every other file is a single action. _shared marks production code shared by those siblings without being an action itself, and can hold its own _test.

actions/table/_shared/helpers.py is used by normalize.py and denormalize.py. actions/column/_shared/helpers.py has no production caller here — the column types each implement their own inspection loop, unlike the TypeScript tree where seven of them import the equivalent helper.

- **table**: Name the polars engine setting QUERY_ENGINE ([`e508881`](https://github.com/datisthq/fairspec-python/commit/e5088818413e72e41bc7f0743e6f02d818001e97))

INSPECT_ENGINE was named for the one caller it started with, but the setting is already read by the foreign-key join and the dialect probe, and polars accepts engine= on fifteen methods rather than collect alone — including sink_parquet and sink_ipc, which the save plugins already call. QUERY_ENGINE names what it selects: how a query plan is executed, for collects and sinks alike.

### Testing

- **fairspec**: Guard docs against stale scripts and paths ([`4a035d0`](https://github.com/datisthq/fairspec-python/commit/4a035d04bc168bf615310c3fde8ca10daffe2f99))

Fail the build when AGENTS.md, README.md or CONTRIBUTING.md names a taskipy task, a pnpm script or a repo path that does not exist, or when .claude/CLAUDE.md stops resolving to AGENTS.md.

The docs had drifted to a test-file convention the repo never used; this is what stops that happening again.

- **table**: Record the remote csv fixture request ([`c8f400a`](https://github.com/datisthq/fairspec-python/commit/c8f400a7f465de204f7328c0a470a0077425fe7e))

The spec fetched the fixture over the network on every run, so it broke whenever the fixture moved — it had been failing with a 404 until the URL was corrected. It now replays a cassette like the prefetch spec.

The vcr_cassette_dir fixture moves to a root conftest: the cassette directory rule is repo-wide, and a second copy would have been the only alternative.

---

**Detailed Changes**: [v0.1.7...v0.2.0](https://github.com/datisthq/fairspec-python/compare/v0.1.7...v0.2.0)

## 0.1.7 (2026-06-15)

### Bug Fixes

- **terminal**: Set non-zero exit code on validation failure ([`a566051`](https://github.com/fairspec/fairspec-python/commit/a5660510359259fb531906fc5a68120fecf63e80))

Validate commands (table, data, file, dataset, tableSchema, dataSchema) route through Session.render_report_result, which printed the report but never signalled failure, so CI relying on $? passed on invalid data. Raise typer.Exit(1) when the report is invalid, including silent mode.

Mirrors fairspec-typescript 9fe1ded.

### Testing

- Write validate fixture as LF to avoid CRLF on windows ([`b6abbb0`](https://github.com/fairspec/fairspec-python/commit/b6abbb0c91587bd445b90d3569f0ea949b01e005))

open("w") translates \n to os.linesep, so on Windows the CRLF made a valid table parse as invalid and the exit-code assertion failed. Write the CSV with newline="" so it is LF on every platform.

---

**Detailed Changes**: [v0.1.6...v0.1.7](https://github.com/fairspec/fairspec-python/compare/v0.1.6...v0.1.7)

## 0.1.6 (2026-06-15)

### Bug Fixes

- **dataset**: Make dataset copy persist to a new folder ([`2bbe158`](https://github.com/fairspec/fairspec-python/commit/2bbe158d0ea323696bb944a9190716cde89e6f09))

The folder plugin only claimed targets that already existed as a directory, while the save action requires a vacant path, so a folder copy was a silent no-op. Detect folder-like targets without requiring existence, pass a Dataset model, and fail loudly if no plugin saves.

Found via smoke testing 0.1.5.

- **terminal**: Coerce loaded descriptor to Dataset for resource selection ([`d8834da`](https://github.com/fairspec/fairspec-python/commit/d8834dab6efbae320f72d55aa5e908a60b3ff0e2))

select_resource and dataset script accessed .resources on the dict that load_dataset returns, so --dataset/--resource lookups always failed and the script REPL exposed a dict. Coerce to a Dataset model first.

Found via smoke testing 0.1.5; same root cause as #11.

### Documentation

- **terminal**: Correct --schema and --dataset/--resource flag names ([`b7ea93d`](https://github.com/fairspec/fairspec-python/commit/b7ea93d4a133851232efadfaef2ab054d2d11765))

Docs referenced --table-schema, --from-dataset and --from-resource, but the CLI exposes --schema, --dataset (-d) and --resource (-r).

---

**Detailed Changes**: [v0.1.5...v0.1.6](https://github.com/fairspec/fairspec-python/compare/v0.1.5...v0.1.6)

## 0.1.5 (2026-06-15)

### Bug Fixes

- **library**: Accept dict descriptor in validate_dataset ([`aa40290`](https://github.com/fairspec/fairspec-python/commit/aa40290380aa252751d9c26888188d1d0756d567))

A dict argument fell through to normalize_dataset and raised AttributeError; coerce it and return the structured report instead.

Fixes #15

- **library**: Infer data schema from top-level JSON arrays ([`67f813c`](https://github.com/fairspec/fairspec-python/commit/67f813cd30244685ee3b22ac65f56dfb8295d67c))

Load JSON data via json.loads instead of the descriptor parser, which rejected any non-object top-level value.

Fixes #12

- **metadata**: Accept object integrity in dataset profile ([`236591b`](https://github.com/fairspec/fairspec-python/commit/236591b513146a1f447e86a037ebeb616d32df80))

The dataset profile declared integrity as a string while the model and every emitter use the object form, so inferred datasets never validated.

Fixes #13

- **table**: Infer file dialect for json and jsonl ([`7c0b9fb`](https://github.com/fairspec/fairspec-python/commit/7c0b9fb1e74f2e6c4b1c1f7247bf02244d471466))

JsonPlugin was missing an infer_file_dialect override, so data infer-dialect could not detect .json/.jsonl files.

Fixes #14

- **terminal**: List dataset resources via model coercion ([`1754eed`](https://github.com/fairspec/fairspec-python/commit/1754eedd27a2124734a872f8bb72e79238ca114f))

load_dataset returns a descriptor dict, so getattr(dataset, "resources") was always None. Coerce to a Dataset model before listing resource names.

Fixes #11

### Chores

- Ignore .worktrees directory ([`69b91a9`](https://github.com/fairspec/fairspec-python/commit/69b91a9bf2ad17f7373b8f821de4a1a78c974336))

### Documentation

- Point remote dataset example at a fairspec descriptor ([`16856c0`](https://github.com/fairspec/fairspec-python/commit/16856c0ef161cc05d491229cfc33b47962839d90))

The previous URL was a Frictionless data package incompatible with the fairspec profile; also wrap load_dataset output in Dataset.model_validate.

Fixes #16

### Testing

- Use ascii fixture in dataset round-trip spec ([`7c266d0`](https://github.com/fairspec/fairspec-python/commit/7c266d0edcfdd68a0252f174607fad692f84cee4))

write_text used the platform default encoding, so the non-ascii content failed on Windows (cp1252). The round-trip test does not need unicode.

Fixes #13

- Write round-trip fixture as bytes to avoid CRLF on windows ([`8012e1d`](https://github.com/fairspec/fairspec-python/commit/8012e1db90edee84424b77f593a85654e4aaec3d))

write_text translates \n to os.linesep, so on Windows the CRLF corrupted type inference (name -> [string, null]) and failed profile validation. write_bytes writes identical LF content on every platform.

Fixes #13

---

**Detailed Changes**: [v0.1.4...v0.1.5](https://github.com/fairspec/fairspec-python/compare/v0.1.4...v0.1.5)

## 0.1.4 (2026-05-26)

### Bug Fixes

- Move ty ignore to assignment line after format ([`2d1db70`](https://github.com/fairspec/fairspec-python/commit/2d1db7064cb199c7e868dce09f5215ae293a0d87))

### Chores

- Bump ruff line-length to 90 ([`cf7681a`](https://github.com/fairspec/fairspec-python/commit/cf7681ac12cd1fc523ddc25d33450f14106f9125))

- **deps**: Bump idna in the uv group across 1 directory ([#10](https://github.com/fairspec/fairspec-python/pull/10), [`1ec4fb0`](https://github.com/fairspec/fairspec-python/commit/1ec4fb0c44a4e74964ab1920561a7897602ccbe2))

Bumps the uv group with 1 update in the / directory: [idna](https://github.com/kjd/idna).

Updates `idna` from 3.11 to 3.15
- [Release notes](https://github.com/kjd/idna/releases)
- [Changelog](https://github.com/kjd/idna/blob/master/HISTORY.md)
- [Commits](https://github.com/kjd/idna/compare/v3.11...v3.15)

---
updated-dependencies:
- dependency-name: idna dependency-version: '3.15'

dependency-type: indirect

dependency-group: uv ...

Signed-off-by: dependabot[bot] &lt;support@github.com>

Co-authored-by: dependabot[bot] &lt;49699333+dependabot[bot]@users.noreply.github.com>

---

**Detailed Changes**: [v0.1.3...v0.1.4](https://github.com/fairspec/fairspec-python/compare/v0.1.3...v0.1.4)

## 0.1.3 (2026-05-16)

### Bug Fixes

- Correct keywords in nested pyproject.toml files ([`d9d9c4e`](https://github.com/fairspec/fairspec-python/commit/d9d9c4e41b78ad116e2e88c4a31c3c89b692e0e0))

### Chores

- Add CITATION.cff ([`025a6a4`](https://github.com/fairspec/fairspec-python/commit/025a6a4c4d79c8cb0fa18393865b5354626d048b))

- Add format:staged pre-commit hook ([`da85f95`](https://github.com/fairspec/fairspec-python/commit/da85f951e3ecbcfe95edb0cd12d04225120c88cf))

- Added cursor/claude ([`874a105`](https://github.com/fairspec/fairspec-python/commit/874a105fd7be224338f17929eb9f550a1462e94c))

- Added json coverate report ([`3727d61`](https://github.com/fairspec/fairspec-python/commit/3727d618cd2046aa4f30cd7508d98c4364b93a84))

- Added pytest-cov ([`35bce34`](https://github.com/fairspec/fairspec-python/commit/35bce34f6913c31e976cdbd64bc87fd1862815be))

- Align docs navigation with fairspec-typescript ([`c440c4f`](https://github.com/fairspec/fairspec-python/commit/c440c4f95eb1b566b0abae3722825b53b79d0971))

Bump livemark to 0.18.0 and consolidate header links into sections (type: "custom" with icons), matching the fairspec-typescript layout. Reorder Terminal before Python and replace renamed lucide icons (terminal-square -> square-terminal, file-type-2 -> file-code) so they render under lucide-react 0.577.0.

Co-Authored-By: Claude Opus 4.7 (1M context) &lt;noreply@anthropic.com>

- Bump livemark ([`e4e9cb6`](https://github.com/fairspec/fairspec-python/commit/e4e9cb617899e200a5a88fd6a561815a18d78dd9))

- Bump pnpm to 11 ([`3d1ee27`](https://github.com/fairspec/fairspec-python/commit/3d1ee274bb324c4d4198c9c74596cfd970d35537))

- Collapse uv/pnpm allowlist in workflows ([`474865e`](https://github.com/fairspec/fairspec-python/commit/474865e85189c93b8ff35bff34ee4fb1478b5d7e))

- Fixed gitattributes ([`3a14f57`](https://github.com/fairspec/fairspec-python/commit/3a14f57d2cf7f237303c953d02ebd9e4a09d8022))

- Fixed github actions ([`23387cd`](https://github.com/fairspec/fairspec-python/commit/23387cdd9660a61e0ed841302df583ab1b7485cb))

- Fixed navigation link ([`8c851e4`](https://github.com/fairspec/fairspec-python/commit/8c851e42d82048ffba777e33b2a73d88970c37bb))

- Improved github workflows ([`21b7d10`](https://github.com/fairspec/fairspec-python/commit/21b7d10d06f5e5dcea84e7839f5b7cbd516f6653))

Adds the same GitHub Actions plumbing that fairspec-standard and fairspec-typescript run: weekly + on-PR semgrep + gitleaks scans (`scan.yaml`), the `@claude` mention bot (`mention.yaml`), and the OWASP-themed automated PR review (`review.yaml`), along with the matching `.gitleaks.toml` allowlist. `package.json` gains `leaks` / `vulns` scripts (semgrep targets `p/python` here) plus a `scan` alias, along with `engines` and `packageManager` so the scan workflow can resolve a pinned Node and pnpm. The allowedTools list in mention/ review is adjusted from pnpm scripts to the `uv run task ...` commands this repo uses, and the pre-commit `format` job is scoped to `*.py` since `ruff format` only handles Python.

Co-Authored-By: Claude Opus 4.7 (1M context) &lt;noreply@anthropic.com>

- Migrate docs from Astro/Starlight to livemark ([`b2d16f2`](https://github.com/fairspec/fairspec-python/commit/b2d16f201a2302626e4b10985652ca3b0589da87))

Replace the website/ workspace package with livemark at the project root. Move docs from website/content/docs/ to docs/, public assets to .livemark/public/, and route the README as the landing page. Drop the overview/contributing.md duplicate and use livemark patches instead.

Co-Authored-By: Claude Opus 4.7 (1M context) &lt;noreply@anthropic.com>

- Migrate to lefthook ([`a5ccd03`](https://github.com/fairspec/fairspec-python/commit/a5ccd03ff3dc6d25aba56f055c14baf6475a61e6))

- Pin actions by SHA in workflows ([`2f81605`](https://github.com/fairspec/fairspec-python/commit/2f8160577be29f6373be0238c6bfc3655dae2ed2))

- Rebase on allowBuilds ([`e308968`](https://github.com/fairspec/fairspec-python/commit/e308968142f5ecf14b346993c90b58308445919e))

- Rename mention to comment workflow and add @claude/review trigger ([`ecd92c6`](https://github.com/fairspec/fairspec-python/commit/ecd92c6c29ebd03d1b743dffac57ce882300e95d))

- Restrict mention workflow to users with repo association ([`428548b`](https://github.com/fairspec/fairspec-python/commit/428548b9001740ee92523d09c2c7b4ddf2d81c19))

- Update comment/review workflow job names ([`ac41795`](https://github.com/fairspec/fairspec-python/commit/ac41795aea4cd26a1570adce93e013a5b63acb12))

- Update navigation ([`e0b698e`](https://github.com/fairspec/fairspec-python/commit/e0b698ed7de84444e8cc8a43ae6f5784f67c7d77))

- Update navigation ([`da79a5c`](https://github.com/fairspec/fairspec-python/commit/da79a5c3ada1dab7f37b04ae2712dbba9f02ab8d))

- Updated links ([`ea8e1aa`](https://github.com/fairspec/fairspec-python/commit/ea8e1aaa9d4a03173f7c5a10a1f7431b24281416))

- **deps**: Bump gitpython from 3.1.46 to 3.1.50 ([#6](https://github.com/fairspec/fairspec-python/pull/6), [`a08141f`](https://github.com/fairspec/fairspec-python/commit/a08141f4a1d932355ff0e88240126d0ac9b5b23a))

Bumps [gitpython](https://github.com/gitpython-developers/GitPython) from 3.1.46 to 3.1.50.
- [Release notes](https://github.com/gitpython-developers/GitPython/releases)
- [Changelog](https://github.com/gitpython-developers/GitPython/blob/main/CHANGES)
- [Commits](https://github.com/gitpython-developers/GitPython/compare/3.1.46...3.1.50)

---
updated-dependencies:
- dependency-name: gitpython dependency-version: 3.1.50

dependency-type: indirect ...

Signed-off-by: dependabot[bot] &lt;support@github.com>

Co-authored-by: dependabot[bot] &lt;49699333+dependabot[bot]@users.noreply.github.com>

- **deps**: Bump mermaid in the npm_and_yarn group across 1 directory ([#9](https://github.com/fairspec/fairspec-python/pull/9), [`a894aaf`](https://github.com/fairspec/fairspec-python/commit/a894aaf3a8ff51eb0b409922a97e799b93c739c0))

Bumps the npm_and_yarn group with 1 update in the / directory: [mermaid](https://github.com/mermaid-js/mermaid).

Updates `mermaid` from 11.14.0 to 11.15.0
- [Release notes](https://github.com/mermaid-js/mermaid/releases)
- [Commits](https://github.com/mermaid-js/mermaid/compare/mermaid@11.14.0...mermaid@11.15.0)

---
updated-dependencies:
- dependency-name: mermaid dependency-version: 11.15.0

dependency-type: indirect

dependency-group: npm_and_yarn ...

Signed-off-by: dependabot[bot] &lt;support@github.com>

Co-authored-by: dependabot[bot] &lt;49699333+dependabot[bot]@users.noreply.github.com>

- **deps**: Bump pygments from 2.19.2 to 2.20.0 ([#7](https://github.com/fairspec/fairspec-python/pull/7), [`abde099`](https://github.com/fairspec/fairspec-python/commit/abde099e628d22270078ebd4d34f0c1c88dafaa2))

Bumps [pygments](https://github.com/pygments/pygments) from 2.19.2 to 2.20.0.
- [Release notes](https://github.com/pygments/pygments/releases)
- [Changelog](https://github.com/pygments/pygments/blob/master/CHANGES)
- [Commits](https://github.com/pygments/pygments/compare/2.19.2...2.20.0)

---
updated-dependencies:
- dependency-name: pygments dependency-version: 2.20.0

dependency-type: indirect ...

Signed-off-by: dependabot[bot] &lt;support@github.com>

Co-authored-by: dependabot[bot] &lt;49699333+dependabot[bot]@users.noreply.github.com>

- **deps**: Bump the uv group across 1 directory with 2 updates ([#8](https://github.com/fairspec/fairspec-python/pull/8), [`7e66204`](https://github.com/fairspec/fairspec-python/commit/7e662045b3440cea06ad7d0d6581bc0bb6909a97))

Bumps the uv group with 2 updates in the / directory: [requests](https://github.com/psf/requests) and [urllib3](https://github.com/urllib3/urllib3).

Updates `requests` from 2.32.5 to 2.33.0 - [Release notes](https://github.com/psf/requests/releases) - [Changelog](https://github.com/psf/requests/blob/main/HISTORY.md) - [Commits](https://github.com/psf/requests/compare/v2.32.5...v2.33.0)

Updates `urllib3` from 2.6.3 to 2.7.0 - [Release notes](https://github.com/urllib3/urllib3/releases) - [Changelog](https://github.com/urllib3/urllib3/blob/main/CHANGES.rst) - [Commits](https://github.com/urllib3/urllib3/compare/2.6.3...2.7.0)

--- updated-dependencies: - dependency-name: requests dependency-version: 2.33.0

dependency-type: indirect

dependency-group: uv

- dependency-name: urllib3 dependency-version: 2.7.0

dependency-group: uv ...

Signed-off-by: dependabot[bot] &lt;support@github.com>

Co-authored-by: dependabot[bot] &lt;49699333+dependabot[bot]@users.noreply.github.com>

### Documentation

- Add high-level Python guides for dataset, table, data, file ([`bca330c`](https://github.com/fairspec/fairspec-python/commit/bca330c9e1781bfcb3c5c1901f2a7daf80a6ac6b))

Add four conceptual API guides modelled on `docs/terminal/{dataset,table, data,file}.md`, using the programming API (load_dataset, validate_table, infer_data_schema, copy_file, …) instead of CLI subcommands. Sit above the format-specific pages in the sidebar (orders 1-4); the existing format guides are renumbered to 10-20. docs/python/table.md is rewritten in place to become the high-level Table guide while keeping the existing Polars-Table-type content (normalize/denormalize/inspect).

Co-Authored-By: Claude Opus 4.7 (1M context) &lt;noreply@anthropic.com>

---

**Detailed Changes**: [v0.1.2...v0.1.3](https://github.com/fairspec/fairspec-python/compare/v0.1.2...v0.1.3)

## 0.1.2 (2026-02-12)

### Bug Fixes

- Fixed semantic release ([`ca8bb11`](https://github.com/fairspec/fairspec-python/commit/ca8bb11ad997f04ca391cc639de56801317e9465))

---

**Detailed Changes**: [v0.1.1...v0.1.2](https://github.com/fairspec/fairspec-python/compare/v0.1.1...v0.1.2)
