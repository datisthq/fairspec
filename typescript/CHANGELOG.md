# Changelog

## [0.13.1](https://github.com/datisthq/fairspec/compare/typescript-v0.12.4...typescript-v0.13.1) (2026-10-28)


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
* **python:** check the JSON type of object and array cells in Polars ([2cc0e6a](https://github.com/datisthq/fairspec/commit/2cc0e6a530d3042257bdb0c83e6fddf362f16f60))
* **python:** print every command's input and output schema with fairspec --llms ([599627a](https://github.com/datisthq/fairspec/commit/599627ad72309bd88dd20fe830f542c2ac05df8c))
* **python:** read date-time and time values with an offset as UTC and add the local types ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* **python:** read JSON arrays of objects with the native Polars reader ([c5441c4](https://github.com/datisthq/fairspec/commit/c5441c403f836c0e409edf9e53fc08a1690f62d1))
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
* **typescript:** build the CLI with fireargs ([bacef9d](https://github.com/datisthq/fairspec/commit/bacef9d53ffb028686ad10fe2d843a3230f2e1ef))
* **typescript:** bundle the CLI for a faster start ([a951e9f](https://github.com/datisthq/fairspec/commit/a951e9f2d3425056bcd049c49ae500a2cd5b02e2))
* **typescript:** check the JSON type of object and array cells in Polars ([2cc0e6a](https://github.com/datisthq/fairspec/commit/2cc0e6a530d3042257bdb0c83e6fddf362f16f60))
* **typescript:** mark removed datasets in a catalog with del ([9837583](https://github.com/datisthq/fairspec/commit/9837583ba1e5b4e0a073e093501dd5a2a2dfa3ad))
* **typescript:** parse default dates and times exactly ([259218a](https://github.com/datisthq/fairspec/commit/259218a2d129823d398223fffc501b57f91b39fb))
* **typescript:** print every command's input and output schema with fairspec --llms ([bacef9d](https://github.com/datisthq/fairspec/commit/bacef9d53ffb028686ad10fe2d843a3230f2e1ef))
* **typescript:** read date-time and time values with an offset as UTC and add the local types ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* **typescript:** read JSON arrays of objects with the native Polars reader ([c5441c4](https://github.com/datisthq/fairspec/commit/c5441c403f836c0e409edf9e53fc08a1690f62d1))
* **typescript:** read xlsx and ods with calamine-node and write them with wasm-xlsxwriter, dropping SheetJS ([82539a3](https://github.com/datisthq/fairspec/commit/82539a3a0435bf712544ea0bad0bcbf3e92a6fae))
* **typescript:** run polars queries on the streaming engine ([#62](https://github.com/datisthq/fairspec/issues/62)) ([04c8d2c](https://github.com/datisthq/fairspec/commit/04c8d2cc4d451416962c6f9f16378604e61a98aa))
* **typescript:** take a command's input as JSON with --json '&lt;value&gt;' ([bacef9d](https://github.com/datisthq/fairspec/commit/bacef9d53ffb028686ad10fe2d843a3230f2e1ef))
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
* **python:** build each JSON Schema validator once instead of per cell ([7ac9d09](https://github.com/datisthq/fairspec/commit/7ac9d0920279fdcab644e5681a6603d6cd26e86a))
* **python:** check enum, const, minLength, maxLength and pattern on duration, wkt and wkb columns ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **python:** check multipleOf with exact decimal arithmetic ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** check self-referencing and unresolved foreign keys ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** check the hash shape before hashing, compare multipart headers, validate a data schema's $schema, $ref and $dynamicRef first, assert its known formats and point a false subschema at its member ([a9b6352](https://github.com/datisthq/fairspec/commit/a9b6352b7dba6544ca24767ea7bdfab968682059))
* **python:** compare integer, number and decimal foreign key columns by value and report any other key type mismatch as a metadata error at /foreignKeys/&lt;i&gt; ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **python:** drop blank lines, keep a quoted nullSequence as text, fail on long rows and repeated header names, consume every row up to the last header row, read past-EOF header rows and short columnNames as errors, skip a UTF-8 BOM and order JSON object columns first-seen ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **python:** drop the charset-normalizer fallback and dependency; textual is a strict UTF-8 decode ([2a884f6](https://github.com/datisthq/fairspec/commit/2a884f6d7c485cd96aebb23be98f627ad4aad41e))
* **python:** enforce maxLength 0 and check required, null values and patterns in validate_table_schema ([38bc5e9](https://github.com/datisthq/fairspec/commit/38bc5e9c2ccec53c2955a8086c92a95e3cb850d2))
* **python:** fail on a JSON array row longer than its header, skip whitespace-only JSONL lines, report headerRows out of ascending order, compare each multipart part's own header under columnNames and across JSON array-row parts ([7c167f5](https://github.com/datisthq/fairspec/commit/7c167f56106a5c16cb70c5b8e7db461fd39977a3))
* **python:** fail on a long row under several header rows, keep a JSON key first seen after the inferred rows, and fail on a past-EOF header row with columnNames ([8bf2264](https://github.com/datisthq/fairspec/commit/8bf226473c5c6cd12eb01856e37d2a87379a1c35))
* **python:** follow RFC 6901 in JSON pointers of schema errors ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** give a JSON file with a top-level object no table ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** infer columns mixing integers and decimals as numbers ([2063df1](https://github.com/datisthq/fairspec/commit/2063df1cd0e3f04fa4eea683e6742a74d41692bb))
* **python:** infer data schemas in the TypeScript shape ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** infer zero-padded numbers as strings, so an inferred schema accepts its own data ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** load a withOrder categorical as a polars Enum in category order ([eb02068](https://github.com/datisthq/fairspec/commit/eb02068231043107f5846dbb2bac793ab21cbe53))
* **python:** load csv files with a crlf lineTerminator ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** load zenodo records whose license is an object ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **python:** make a list with an unparsable item a cell/type error ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** match trueValues/falseValues exactly, a declared list replacing its default; report a value in both lists as a table schema error ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** pass --sample-bytes on to infer-dialect, which dropped it ([599627a](https://github.com/datisthq/fairspec/commit/599627ad72309bd88dd20fe830f542c2ac05df8c))
* **python:** pass inference flags through the table commands ([9bf761c](https://github.com/datisthq/fairspec/commit/9bf761cf366c99b4ebf25d1ca5c25ae9c3194005))
* **python:** pass inference options from the dataset infer command ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** pass table schema options through dataset and resource inference ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** pass the resource, not its dump, to convert_resource_to_ckan ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **python:** read an integer categorical cell as an integer and check its minimum, maximum, exclusive bounds and multipleOf ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** read csv header rows as the spec describes ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** read json tables as the spec describes ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** read lowercase t/z, leap seconds at 23:59 UTC and the RFC 3339 default forms, and check url, email, duration, hex and base64 by the spec grammars ([6c97483](https://github.com/datisthq/fairspec/commit/6c97483c0d88c03a00c00312d53db988c2bb8969))
* **python:** read number and integer cells by the JSON number grammar, integers exactly within int64 ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
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
* **python:** save the ckan descriptor as dataset.json so load finds it ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **python:** scan JSONL natively when its dialect is inferred ([86188f3](https://github.com/datisthq/fairspec/commit/86188f3428368ae2ed656984cbd02bab95701d29))
* **python:** sort an integer or labelled string categorical column without withOrder by value ([e22976e](https://github.com/datisthq/fairspec/commit/e22976e8502529f421622576a8f401e1ab1ac8db))
* **python:** split list cells literally: an empty item fails a typed list, an empty cell is an empty list ([d7fdb10](https://github.com/datisthq/fairspec/commit/d7fdb10c245e323a268c87ff0c1dbbd7fd670e93))
* **python:** the same limits, truncated flag and budget in the MCP server; pass every tool failure to the client with its message ([45f6dd1](https://github.com/datisthq/fairspec/commit/45f6dd111805f06e4d9e5e0d092330fc14ce8d58))
* **python:** transliterate non-ascii letters in inferred resource names ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** treat a column without a type as unknown ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
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
* **typescript:** reuse only the bundled 2020-12 metaschema, never a schema's own $id ([a9a9f8c](https://github.com/datisthq/fairspec/commit/a9a9f8c74d222bee19e8a0044da1bd0fe8d2816b))
* **typescript:** save the ckan descriptor as dataset.json so load finds it ([3bcf801](https://github.com/datisthq/fairspec/commit/3bcf80195eb33155425174e224e2eaa9a45e8200))
* **typescript:** scan JSONL natively when its dialect is inferred ([86188f3](https://github.com/datisthq/fairspec/commit/86188f3428368ae2ed656984cbd02bab95701d29))
* **typescript:** scan JSONL with a default dialect natively ([8d5fa84](https://github.com/datisthq/fairspec/commit/8d5fa841a5e0542f2946ec9c5441ef2431e50fdf))
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
* **typescript:** keep 0 as no error in the check read's mask (2.39 -&gt; 2.29 s), and drop the type check of a column read as its source, which stays in the read (2.29 -&gt; 1.85 s) ([27a4d79](https://github.com/datisthq/fairspec/commit/27a4d793b0e6a114c9274fda9d906c26b85cddd7))
* **typescript:** make table validation about a third faster ([b2a90dc](https://github.com/datisthq/fairspec/commit/b2a90dcccf26acb3543d24c05f7a387222425f4d))
* **typescript:** read an integer column plain when its sample is, and again exactly only if a cell needs it (1GB 6.5 -&gt; 2.6 s) ([27a4d79](https://github.com/datisthq/fairspec/commit/27a4d793b0e6a114c9274fda9d906c26b85cddd7))
* **typescript:** read plain integers first, bind exact and temporal steps as struct fields, keep 0 in the error mask, drop impossible type checks and search blank lines with concurrent reads ([27a4d79](https://github.com/datisthq/fairspec/commit/27a4d793b0e6a114c9274fda9d906c26b85cddd7))
* **typescript:** read whole-second uppercase temporal defaults with the fast parser alone, each step a struct field (2.63 -&gt; 2.39 s; createFieldExpr builds pl.field through pl.Expr.deserialize, since nodejs-polars has none) ([27a4d79](https://github.com/datisthq/fairspec/commit/27a4d793b0e6a114c9274fda9d906c26b85cddd7))
* **typescript:** search for blank lines with four concurrent reads into fixed buffers instead of a concatenating stream (allstr schema 1.83 -&gt; 1.2 s at 1GB; scan 0.54 -&gt; 0.10 s) ([27a4d79](https://github.com/datisthq/fairspec/commit/27a4d793b0e6a114c9274fda9d906c26b85cddd7))
* **typescript:** send only cells with a point or an exponent to the exact integer reading, each step a struct field (4 columns read exactly at 1GB: 6.6 -&gt; 1.1 s; the exact path still builds a string, since nodejs-polars has no Int128 and no per-row pow) ([27a4d79](https://github.com/datisthq/fairspec/commit/27a4d793b0e6a114c9274fda9d906c26b85cddd7))

## [0.12.4](https://github.com/fairspec/fairspec-typescript/compare/v0.12.3...v0.12.4) (2026-06-15)

### Bug Fixes

- **dataset:** make dataset copy persist to a new folder ([cf0beef](https://github.com/fairspec/fairspec-typescript/commit/cf0beefab8cc77057145a4bfc9e1a2ab750c7cba))

## [0.12.3](https://github.com/fairspec/fairspec-typescript/compare/v0.12.2...v0.12.3) (2026-06-15)

### Bug Fixes

- **metadata:** accept object integrity in dataset profile ([5230677](https://github.com/fairspec/fairspec-typescript/commit/5230677694f0ced07876961d002548f4065872a0)), closes [fairspec-standard#35](https://github.com/fairspec/fairspec-standard/issues/35)

## [0.12.2](https://github.com/fairspec/fairspec-typescript/compare/v0.12.1...v0.12.2) (2026-06-15)

### Bug Fixes

- reference xlsx via pnpm catalog to satisfy peer dep spec ([cd28fa1](https://github.com/fairspec/fairspec-typescript/commit/cd28fa18f4560bd93e5310eaf222fec5aefca043))
- **terminal:** set non-zero exit code on validation failure ([9fe1ded](https://github.com/fairspec/fairspec-typescript/commit/9fe1ded7e757c38e654d0fd344149ab8d0607c72))

## [0.12.1](https://github.com/fairspec/fairspec-typescript/compare/v0.12.0...v0.12.1) (2026-05-15)

### Bug Fixes

- move xlsx to peerDependencies ([7c5e740](https://github.com/fairspec/fairspec-typescript/commit/7c5e740f474d4d0b66769832d47ccaf6fceaaa9e))

## [0.12.0](https://github.com/fairspec/fairspec-typescript/compare/v0.11.3...v0.12.0) (2026-02-11)

### Features

- Export metadata submodels ([cf8f8a7](https://github.com/fairspec/fairspec-typescript/commit/cf8f8a73ac71e458c3ab4f80c45b0e14a424e4a6))

## [0.11.3](https://github.com/fairspec/fairspec-typescript/compare/v0.11.2...v0.11.3) (2026-02-10)

### Bug Fixes

- File dialect profile url ([679eda1](https://github.com/fairspec/fairspec-typescript/commit/679eda12fcea9883900dd76d688abbd640fb4362))

## [0.11.2](https://github.com/fairspec/fairspec-typescript/compare/v0.11.1...v0.11.2) (2026-02-10)

### Bug Fixes

- Fixed fairspec version setting ([51590ed](https://github.com/fairspec/fairspec-typescript/commit/51590ed32e41ba7941a54010722dd0ff0509e090))

## [0.11.1](https://github.com/fairspec/fairspec-typescript/compare/v0.11.0...v0.11.1) (2026-02-10)

### Bug Fixes

- Improve infer table schema ([#33](https://github.com/fairspec/fairspec-typescript/issues/33)) ([0cd04ec](https://github.com/fairspec/fairspec-typescript/commit/0cd04ecc06dc08b893ce303a30c5fcb59aa12cca))

## [0.11.0](https://github.com/fairspec/fairspec-typescript/compare/v0.10.0...v0.11.0) (2026-02-10)

### Features

- Support nullable properties and all required ([#32](https://github.com/fairspec/fairspec-typescript/issues/32)) ([1f737ab](https://github.com/fairspec/fairspec-typescript/commit/1f737ab5b49d8e39eb005126a38c75e1d96497b0))

## [0.10.0](https://github.com/fairspec/fairspec-typescript/compare/v0.9.3...v0.10.0) (2026-02-09)

### Features

- Rebase on File Dialect spec ([#31](https://github.com/fairspec/fairspec-typescript/issues/31)) ([97b2fd9](https://github.com/fairspec/fairspec-typescript/commit/97b2fd9134b25967076f12520371e5d821a4d421))

## [0.9.3](https://github.com/fairspec/fairspec-typescript/compare/v0.9.2...v0.9.3) (2026-02-09)

### Bug Fixes

- Fix row number tracking when data contains 'number' column ([#30](https://github.com/fairspec/fairspec-typescript/issues/30)) ([031a8eb](https://github.com/fairspec/fairspec-typescript/commit/031a8ebc57c524017bbbf94e610e7b9a5bce8eed)), closes [#29](https://github.com/fairspec/fairspec-typescript/issues/29) [#29](https://github.com/fairspec/fairspec-typescript/issues/29)

## [0.9.2](https://github.com/fairspec/fairspec-typescript/compare/v0.9.1...v0.9.2) (2026-02-06)

### Bug Fixes

- Fix MCP server ([#28](https://github.com/fairspec/fairspec-typescript/issues/28)) ([77f4ca1](https://github.com/fairspec/fairspec-typescript/commit/77f4ca128f2378ecc37cf78fc207fbd010788e55))

## [0.9.1](https://github.com/fairspec/fairspec-typescript/compare/v0.9.0...v0.9.1) (2026-02-06)

### Bug Fixes

- Fixed agent release ([5ee1b08](https://github.com/fairspec/fairspec-typescript/commit/5ee1b0840d0fc9588fcfb1b0b650d2ba6a1809cd))

## [0.8.0](https://github.com/fairspec/fairspec-typescript/compare/v0.7.0...v0.8.0) (2026-02-05)

### Features

- Improved dialect inference ([#26](https://github.com/fairspec/fairspec-typescript/issues/26)) ([829ce34](https://github.com/fairspec/fairspec-typescript/commit/829ce3494dd7b038ce99a533c25dbf20459e9a15))

### Bug Fixes

- Fixed dialect usage in loadTable ([94fe2b3](https://github.com/fairspec/fairspec-typescript/commit/94fe2b384bde8b41ea3f27fd7f7732c15ed6db9e))

## [0.7.0](https://github.com/fairspec/fairspec-typescript/compare/v0.6.1...v0.7.0) (2026-02-04)

### Features

- Improved dialect support ([#25](https://github.com/fairspec/fairspec-typescript/issues/25)) ([658b184](https://github.com/fairspec/fairspec-typescript/commit/658b1840cdb3799c4a000e0f75580553f315961c))

## [0.6.1](https://github.com/fairspec/fairspec-typescript/compare/v0.6.0...v0.6.1) (2026-02-04)

### Bug Fixes

- Limit preview to 100 rows ([337fdc6](https://github.com/fairspec/fairspec-typescript/commit/337fdc6f23883647f679da1c6ea264cd6c4e2f6a))

## [0.6.0](https://github.com/fairspec/fairspec-typescript/compare/v0.5.2...v0.6.0) (2026-02-04)

### Features

- Preview table command ([#24](https://github.com/fairspec/fairspec-typescript/issues/24)) ([6301bde](https://github.com/fairspec/fairspec-typescript/commit/6301bde92c826c459a9da721c26b57a69d7cb4ff))

## [0.5.2](https://github.com/fairspec/fairspec-typescript/compare/v0.5.1...v0.5.2) (2026-01-27)

### Bug Fixes

- Fixed assertion messages ([f6768fa](https://github.com/fairspec/fairspec-typescript/commit/f6768fa528f8a01367bc8cee0b844a57ffa2444e))

## [0.5.1](https://github.com/fairspec/fairspec-typescript/compare/v0.5.0...v0.5.1) (2026-01-27)

### Bug Fixes

- Fixed resource errors ([5b954e9](https://github.com/fairspec/fairspec-typescript/commit/5b954e919882da766222daa996ce8daae4c1bf00))

## [0.5.0](https://github.com/fairspec/fairspec-typescript/compare/v0.4.0...v0.5.0) (2026-01-27)

### Features

- Improve error hierarchy ([#23](https://github.com/fairspec/fairspec-typescript/issues/23)) ([af8266c](https://github.com/fairspec/fairspec-typescript/commit/af8266cdb4388b82a6c3990e23f267c7cd989ef6))

## [0.4.0](https://github.com/fairspec/fairspec-typescript/compare/v0.3.0...v0.4.0) (2026-01-22)

### Features

- Migrated to fairspec@0.3.0 ([#22](https://github.com/fairspec/fairspec-typescript/issues/22)) ([de6eaa6](https://github.com/fairspec/fairspec-typescript/commit/de6eaa628a7f52e4eb1d6da38b98a21f1a68b3d9))

## [0.3.0](https://github.com/fairspec/fairspec-typescript/compare/v0.2.2...v0.3.0) (2026-01-22)

### Features

- Migrate to fairspec@0.2.2 ([#21](https://github.com/fairspec/fairspec-typescript/issues/21)) ([b2f1914](https://github.com/fairspec/fairspec-typescript/commit/b2f191421893c32977488cede8cd06a4dade84a8))

## [0.2.2](https://github.com/fairspec/fairspec-typescript/compare/v0.2.1...v0.2.2) (2026-01-19)

### Bug Fixes

- Fixed tableSchema rendering ([fc163f7](https://github.com/fairspec/fairspec-typescript/commit/fc163f7beeab6966e4e52b6c1a8e8019d0c2ca05))

## [0.2.1](https://github.com/fairspec/fairspec-typescript/compare/v0.2.0...v0.2.1) (2026-01-19)

### Bug Fixes

- Updated table profile ([5382bec](https://github.com/fairspec/fairspec-typescript/commit/5382beca5f116ebdd3badcd0bfe2ac67c2d495f8))
