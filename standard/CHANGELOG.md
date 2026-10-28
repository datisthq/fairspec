# Changelog

## [0.7.0](https://github.com/datisthq/fairspec/compare/standard-v0.6.1...standard-v0.7.0) (2026-10-28)


### Features

* add local-date-time and local-time, and require a UTC offset in date-time and time ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* **desktop:** show column/extra errors ([4fe5d2f](https://github.com/datisthq/fairspec/commit/4fe5d2f038d775e16a09d257f57e7414e9c2ed1b))
* **python:** accept a JSON Schema 2020-12 subset as a table schema ([4fe5d2f](https://github.com/datisthq/fairspec/commit/4fe5d2f038d775e16a09d257f57e7414e9c2ed1b))
* **python:** read date-time and time values with an offset as UTC and add the local types ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* **standard:** accept a JSON Schema 2020-12 subset as a table schema ([4fe5d2f](https://github.com/datisthq/fairspec/commit/4fe5d2f038d775e16a09d257f57e7414e9c2ed1b))
* **standard:** define the temporalFormat specifiers and the offset rule per temporal type ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4)), closes [#42](https://github.com/datisthq/fairspec/issues/42)
* **standard:** mark removed datasets in a catalog with del ([9837583](https://github.com/datisthq/fairspec/commit/9837583ba1e5b4e0a073e093501dd5a2a2dfa3ad))
* **typescript:** accept a JSON Schema 2020-12 subset as a table schema ([4fe5d2f](https://github.com/datisthq/fairspec/commit/4fe5d2f038d775e16a09d257f57e7414e9c2ed1b))
* **typescript:** mark removed datasets in a catalog with del ([9837583](https://github.com/datisthq/fairspec/commit/9837583ba1e5b4e0a073e093501dd5a2a2dfa3ad))
* **typescript:** read date-time and time values with an offset as UTC and add the local types ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))


### Bug Fixes

* **desktop:** show local date-times without shifting them into the viewer's time zone ([cac2830](https://github.com/datisthq/fairspec/commit/cac2830c45c83f3ef8478738678d39bd528d65e4))
* extend the testsuite and fix the divergences it found ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** accept a utc offset in a date-time without a temporalFormat ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** add the json pointer helpers the json table plugin imports ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** apply table-level missing values when validating ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** check multipleOf with exact decimal arithmetic ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** check self-referencing and unresolved foreign keys ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** follow RFC 6901 in JSON pointers of schema errors ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** give a JSON file with a top-level object no table ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** infer data schemas in the TypeScript shape ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** load csv files with a crlf lineTerminator ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** make a list with an unparsable item a cell/type error ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** pass inference options from the dataset infer command ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** pass table schema options through dataset and resource inference ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** read csv header rows as the spec describes ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** read json tables as the spec describes ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** reject a spreadsheet dialect that sets both sheetName and sheetNumber ([6de7a0d](https://github.com/datisthq/fairspec/commit/6de7a0dd559dc45aa8e6a7c054e169881dd3187b))
* **python:** render a whole-number bound without a trailing .0 ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** report column errors before cell errors ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** report missing cells in duration, geometry and JSON columns ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** require every part of a textual resource to be utf-8 ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** transliterate non-ascii letters in inferred resource names ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** treat a column without a type as unknown ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **python:** validate descriptors against their profile, then the core model ([b755f85](https://github.com/datisthq/fairspec/commit/b755f8514540a9b1f46c19fd4cfd3ce34963d185))
* **python:** validate empty JSON documents and report non-JSON data ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **standard:** accept any value in a column without a type ([ea97c9f](https://github.com/datisthq/fairspec/commit/ea97c9f75ab7479ee103efe331487e770563f2b0))
* **standard:** allow and ignore unknown properties in every descriptor ([88c8ddd](https://github.com/datisthq/fairspec/commit/88c8ddd17724b96aa27267987b83b2000cb22795))
* **standard:** allow categories and withOrder only on categorical columns ([57f7228](https://github.com/datisthq/fairspec/commit/57f7228ce710a114f144d8167c0ffef3d093fa43))
* **standard:** decide foreign key column types, long JSON array rows, whitespace JSONL lines, header row order and multipart header checks ([2e1df43](https://github.com/datisthq/fairspec/commit/2e1df437cdb7f36da68ca1edee18d1a62d4dc113))
* **standard:** define header carry-forward, object JSON files and multipart textual ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **standard:** enforce date-time, date, URI and JSON pointer shapes with patterns ([f32eb88](https://github.com/datisthq/fairspec/commit/f32eb88e8dd209730ecf31240513ce649bd9ba28))
* **standard:** forbid a data schema on multipart data ([de3aaef](https://github.com/datisthq/fairspec/commit/de3aaefdf8f95b9ca50016ad0e01eefc664cdd07))
* **standard:** give absent dialect and table schema properties defaults instead of inference ([148d1c8](https://github.com/datisthq/fairspec/commit/148d1c8b0f280888b1c9c6eae9daf1a7c5ca44fe))
* **standard:** improve standard pipeline ([#52](https://github.com/datisthq/fairspec/issues/52)) ([407d938](https://github.com/datisthq/fairspec/commit/407d9381adb1be365ac7243ece48410aca150ec7))
* **standard:** number header and comment rows by parsed row, not by line ([7cca9d3](https://github.com/datisthq/fairspec/commit/7cca9d3a10cf744c8e9ee80da16f6cfd8b0835bc))
* **standard:** pin down table value checks, null foreign keys and error order ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **standard:** read a quoted empty csv cell as an empty string ([791326f](https://github.com/datisthq/fairspec/commit/791326f849af8b2cae52c31316617c3bfbb7a5bd))
* **standard:** reject a scheme in an internal path and allow names starting with two dots ([c9b50e6](https://github.com/datisthq/fairspec/commit/c9b50e6499457e1c6af06604b4f2d0a37c274db6))
* **standard:** reject a spreadsheet dialect that sets both sheetName and sheetNumber ([6de7a0d](https://github.com/datisthq/fairspec/commit/6de7a0dd559dc45aa8e6a7c054e169881dd3187b))
* **standard:** require a file dialect's format ([c58361f](https://github.com/datisthq/fairspec/commit/c58361fb93873423bed4e9a3cce19a437f6339d2))
* **standard:** require a lowercase hex digest, and no integrity on inline data ([3a0af71](https://github.com/datisthq/fairspec/commit/3a0af7149e76774e8bcddc8bea2115be50bd8aea))
* **standard:** require table schema keys to name columns and primary key cells to be present ([01e5341](https://github.com/datisthq/fairspec/commit/01e53417711a305233041ee066882edc6978d9d5))
* **standard:** require types, not resourceType, in the DataCite and GREI extensions ([a713d91](https://github.com/datisthq/fairspec/commit/a713d91a8499a663963d56508b6feea57c17e359))
* **standard:** require unique resource names and literal missing value matching ([4d07c6d](https://github.com/datisthq/fairspec/commit/4d07c6da7ccc9bcd218bdf13b59d9f14acee9292))
* **standard:** state that decimal bounds are JSON numbers ([8a3c31c](https://github.com/datisthq/fairspec/commit/8a3c31c44e19ee05f95e753cdfcb83fdd3e50060))
* **standard:** state what a resource without data and commentRows on json objects mean ([955790c](https://github.com/datisthq/fairspec/commit/955790c2202a3b9e21219128a9f525b45d5c0d8e))
* **standard:** strip only the prefix and suffix of a withText number ([3a92d39](https://github.com/datisthq/fairspec/commit/3a92d39fa1cabad9f3439bd7b089d6704874b769))
* **standard:** sync a catalog with a high-water mark, rereading ties ([93af4f1](https://github.com/datisthq/fairspec/commit/93af4f17d724c6840687f865ba86c8c49c84f67e))
* **standard:** tighten single characters, sheet numbers, key columns and paths ([b87487d](https://github.com/datisthq/fairspec/commit/b87487dd84abcd71b325b1d8f3bc1294b56815e7))
* **standard:** validate descriptors against their profile, then the core model ([b755f85](https://github.com/datisthq/fairspec/commit/b755f8514540a9b1f46c19fd4cfd3ce34963d185))
* **standard:** write the decided table, dialect, dataset, data schema and catalog rules into the specs ([4a4f12c](https://github.com/datisthq/fairspec/commit/4a4f12c7e04d8b2b323cfb920a37739ae16d638e))
* **typescript:** accept an empty cell in a column without a type ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** accept an integer column where the schema declares number ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** accept any value in a column without a type ([ea97c9f](https://github.com/datisthq/fairspec/commit/ea97c9f75ab7479ee103efe331487e770563f2b0))
* **typescript:** accept file names starting with two dots and reject internal paths with a scheme ([c9b50e6](https://github.com/datisthq/fairspec/commit/c9b50e6499457e1c6af06604b4f2d0a37c274db6))
* **typescript:** allow categories and withOrder only on categorical columns ([57f7228](https://github.com/datisthq/fairspec/commit/57f7228ce710a114f144d8167c0ffef3d093fa43))
* **typescript:** apply minimum and maximum only to numeric columns ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** apply table-level missingValues when validating ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** bundle the catalog and dataset profiles that allow unknown properties ([88c8ddd](https://github.com/datisthq/fairspec/commit/88c8ddd17724b96aa27267987b83b2000cb22795))
* **typescript:** bundle the file dialect profile that requires format ([c58361f](https://github.com/datisthq/fairspec/commit/c58361fb93873423bed4e9a3cce19a437f6339d2))
* **typescript:** carry header labels forward across empty cells ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** check key columns, foreign key lengths and null key cells ([01e5341](https://github.com/datisthq/fairspec/commit/01e53417711a305233041ee066882edc6978d9d5))
* **typescript:** check multipleOf with exact decimal arithmetic ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** give decimal columns a precision and scale so polars does not abort ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** keep declared textual and name resources by position in inference ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** load any JSON document as resource data ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** load CSV files with a crlf lineTerminator ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** load sqlite INTEGER columns as integers ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** make a list with an unparsable item a cell/type error ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** match integer categories against the text of a CSV cell ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** parse dates, times and date-times exactly ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** read a declared dialect with defaults, sniffing only without one ([148d1c8](https://github.com/datisthq/fairspec/commit/148d1c8b0f280888b1c9c6eae9daf1a7c5ca44fe))
* **typescript:** read jsonPointer as an RFC 6901 pointer ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** reject a data schema on multipart data instead of checking only its first part ([de3aaef](https://github.com/datisthq/fairspec/commit/de3aaefdf8f95b9ca50016ad0e01eefc664cdd07))
* **typescript:** reject a spreadsheet dialect that sets both sheetName and sheetNumber ([6de7a0d](https://github.com/datisthq/fairspec/commit/6de7a0dd559dc45aa8e6a7c054e169881dd3187b))
* **typescript:** reject repeated resource names and check self foreign keys of unnamed resources ([4d07c6d](https://github.com/datisthq/fairspec/commit/4d07c6da7ccc9bcd218bdf13b59d9f14acee9292))
* **typescript:** report a column pattern the table engine cannot compile ([f32eb88](https://github.com/datisthq/fairspec/commit/f32eb88e8dd209730ecf31240513ce649bd9ba28))
* **typescript:** report missing cells in duration, geometry and JSON columns ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** require a lowercase hex digest, and no integrity on inline data ([3a0af71](https://github.com/datisthq/fairspec/commit/3a0af7149e76774e8bcddc8bea2115be50bd8aea))
* **typescript:** require every part of a textual resource to be utf-8 ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** skip foreign keys with a null part ([548b4da](https://github.com/datisthq/fairspec/commit/548b4da8683742b3f3d6f6d86aa691d9a03b889c))
* **typescript:** strip only the text around a withText number ([3a92d39](https://github.com/datisthq/fairspec/commit/3a92d39fa1cabad9f3439bd7b089d6704874b769))
* **typescript:** tighten single characters, sheet numbers, key columns and paths ([b87487d](https://github.com/datisthq/fairspec/commit/b87487dd84abcd71b325b1d8f3bc1294b56815e7))
* **typescript:** validate descriptors against their profile, then the core model ([b755f85](https://github.com/datisthq/fairspec/commit/b755f8514540a9b1f46c19fd4cfd3ce34963d185))

## [0.6.1](https://github.com/datisthq/fairspec/compare/v0.6.0...v0.6.1) (2026-09-05)

### Bug Fixes

* **dataset:** correct the multipart resource and integrity wording ([7edf233](https://github.com/datisthq/fairspec/commit/7edf233416aba8a4ee4c6933762d20ddfa93f6cc))

## [0.6.0](https://github.com/datisthq/fairspec/compare/v0.5.0...v0.6.0) (2026-09-04)

### Features

* **landing:** replace stack section with Datist attribution ([14eb3ba](https://github.com/datisthq/fairspec/commit/14eb3bae44d88b92609fc24a8e31247d99b6f51d))

### Bug Fixes

* **dataset:** make Integrity a JSON object in the profile ([79c3982](https://github.com/datisthq/fairspec/commit/79c3982b0ffd95cfa0270b0519fc45ec44824577)), closes [#35](https://github.com/datisthq/fairspec/issues/35)

## [0.5.0](https://github.com/fairspec/fairspec/compare/v0.4.0...v0.5.0) (2026-02-09)

### Features

* Table schema all required property ([#11](https://github.com/fairspec/fairspec/issues/11)) ([55dc452](https://github.com/fairspec/fairspec/commit/55dc4525e1cdfe3b32959bff74c0a0a684b815ea))

## [0.4.0](https://github.com/fairspec/fairspec/compare/v0.3.0...v0.4.0) (2026-02-06)

### Features

* Rebase on File Dialect ([#10](https://github.com/fairspec/fairspec/issues/10)) ([bb15fcd](https://github.com/fairspec/fairspec/commit/bb15fcd80952368646ad7eeba3837cde2d0c9b27))

## [0.3.0](https://github.com/fairspec/fairspec/compare/v0.2.3...v0.3.0) (2026-01-22)

### Features

* Added Data Schema spec ([#4](https://github.com/fairspec/fairspec/issues/4)) ([376cdab](https://github.com/fairspec/fairspec/commit/376cdab3f7786423b01b2f0475a3767116f86e22))

## [0.2.3](https://github.com/fairspec/fairspec/compare/v0.2.2...v0.2.3) (2026-01-22)

### Bug Fixes

* Fixed dialect spec ([f76101b](https://github.com/fairspec/fairspec/commit/f76101b37795a81bfdd89472617d838755799478))

## [0.2.2](https://github.com/fairspec/fairspec/compare/v0.2.1...v0.2.2) (2026-01-22)

### Bug Fixes

* Fixed dialect profile ([175d3b4](https://github.com/fairspec/fairspec/commit/175d3b44c97b8b88adfd106004ed431c4302ecbe))

## [0.2.1](https://github.com/fairspec/fairspec/compare/v0.2.0...v0.2.1) (2026-01-22)

### Bug Fixes

* Fixed dialect profile ([3465c2d](https://github.com/fairspec/fairspec/commit/3465c2d4c9c66afb8212a367702233ca413bebc6))

## [0.2.0](https://github.com/fairspec/fairspec/compare/v0.1.0...v0.2.0) (2026-01-22)

### Features

* Updated specifications ([#3](https://github.com/fairspec/fairspec/issues/3)) ([0d8cc13](https://github.com/fairspec/fairspec/commit/0d8cc13b5bb1a5c227a0a37ac660b23592d17104))
