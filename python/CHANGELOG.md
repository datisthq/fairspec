# Changelog

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

The TypeScript repo names every test file <module>.unit.ts. Rename the 139 Python specs from <module>_spec.py to <module>_unit.py, and the taskipy task from spec to unit so `uv run task unit` mirrors `pnpm unit`.

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

Signed-off-by: dependabot[bot] <support@github.com>

Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

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

Correct a claim that was never true: the Specs section asked for test_<module>.py files, while pyproject.toml has always set python_files = "*_spec.py" and no file in the repo matched the documented pattern. The taskipy tasks for build, publish and coverage and the whole pnpm side — leaks, vulns, scan, deps, docs:start, docs:build — went undocumented.

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

Signed-off-by: dependabot[bot] <support@github.com>

Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

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

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

- Bump livemark ([`e4e9cb6`](https://github.com/fairspec/fairspec-python/commit/e4e9cb617899e200a5a88fd6a561815a18d78dd9))

- Bump pnpm to 11 ([`3d1ee27`](https://github.com/fairspec/fairspec-python/commit/3d1ee274bb324c4d4198c9c74596cfd970d35537))

- Collapse uv/pnpm allowlist in workflows ([`474865e`](https://github.com/fairspec/fairspec-python/commit/474865e85189c93b8ff35bff34ee4fb1478b5d7e))

- Fixed gitattributes ([`3a14f57`](https://github.com/fairspec/fairspec-python/commit/3a14f57d2cf7f237303c953d02ebd9e4a09d8022))

- Fixed github actions ([`23387cd`](https://github.com/fairspec/fairspec-python/commit/23387cdd9660a61e0ed841302df583ab1b7485cb))

- Fixed navigation link ([`8c851e4`](https://github.com/fairspec/fairspec-python/commit/8c851e42d82048ffba777e33b2a73d88970c37bb))

- Improved github workflows ([`21b7d10`](https://github.com/fairspec/fairspec-python/commit/21b7d10d06f5e5dcea84e7839f5b7cbd516f6653))

Adds the same GitHub Actions plumbing that fairspec-standard and fairspec-typescript run: weekly + on-PR semgrep + gitleaks scans (`scan.yaml`), the `@claude` mention bot (`mention.yaml`), and the OWASP-themed automated PR review (`review.yaml`), along with the matching `.gitleaks.toml` allowlist. `package.json` gains `leaks` / `vulns` scripts (semgrep targets `p/python` here) plus a `scan` alias, along with `engines` and `packageManager` so the scan workflow can resolve a pinned Node and pnpm. The allowedTools list in mention/ review is adjusted from pnpm scripts to the `uv run task ...` commands this repo uses, and the pre-commit `format` job is scoped to `*.py` since `ruff format` only handles Python.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

- Migrate docs from Astro/Starlight to livemark ([`b2d16f2`](https://github.com/fairspec/fairspec-python/commit/b2d16f201a2302626e4b10985652ca3b0589da87))

Replace the website/ workspace package with livemark at the project root. Move docs from website/content/docs/ to docs/, public assets to .livemark/public/, and route the README as the landing page. Drop the overview/contributing.md duplicate and use livemark patches instead.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

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

Signed-off-by: dependabot[bot] <support@github.com>

Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

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

Signed-off-by: dependabot[bot] <support@github.com>

Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

- **deps**: Bump pygments from 2.19.2 to 2.20.0 ([#7](https://github.com/fairspec/fairspec-python/pull/7), [`abde099`](https://github.com/fairspec/fairspec-python/commit/abde099e628d22270078ebd4d34f0c1c88dafaa2))

Bumps [pygments](https://github.com/pygments/pygments) from 2.19.2 to 2.20.0.
- [Release notes](https://github.com/pygments/pygments/releases)
- [Changelog](https://github.com/pygments/pygments/blob/master/CHANGES)
- [Commits](https://github.com/pygments/pygments/compare/2.19.2...2.20.0)

---
updated-dependencies:
- dependency-name: pygments dependency-version: 2.20.0

dependency-type: indirect ...

Signed-off-by: dependabot[bot] <support@github.com>

Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

- **deps**: Bump the uv group across 1 directory with 2 updates ([#8](https://github.com/fairspec/fairspec-python/pull/8), [`7e66204`](https://github.com/fairspec/fairspec-python/commit/7e662045b3440cea06ad7d0d6581bc0bb6909a97))

Bumps the uv group with 2 updates in the / directory: [requests](https://github.com/psf/requests) and [urllib3](https://github.com/urllib3/urllib3).

Updates `requests` from 2.32.5 to 2.33.0 - [Release notes](https://github.com/psf/requests/releases) - [Changelog](https://github.com/psf/requests/blob/main/HISTORY.md) - [Commits](https://github.com/psf/requests/compare/v2.32.5...v2.33.0)

Updates `urllib3` from 2.6.3 to 2.7.0 - [Release notes](https://github.com/urllib3/urllib3/releases) - [Changelog](https://github.com/urllib3/urllib3/blob/main/CHANGES.rst) - [Commits](https://github.com/urllib3/urllib3/compare/2.6.3...2.7.0)

--- updated-dependencies: - dependency-name: requests dependency-version: 2.33.0

dependency-type: indirect

dependency-group: uv

- dependency-name: urllib3 dependency-version: 2.7.0

dependency-group: uv ...

Signed-off-by: dependabot[bot] <support@github.com>

Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>

### Documentation

- Add high-level Python guides for dataset, table, data, file ([`bca330c`](https://github.com/fairspec/fairspec-python/commit/bca330c9e1781bfcb3c5c1901f2a7daf80a6ac6b))

Add four conceptual API guides modelled on docs/terminal/{dataset,table, data,file}.md, using the programming API (load_dataset, validate_table, infer_data_schema, copy_file, …) instead of CLI subcommands. Sit above the format-specific pages in the sidebar (orders 1-4); the existing format guides are renumbered to 10-20. docs/python/table.md is rewritten in place to become the high-level Table guide while keeping the existing Polars-Table-type content (normalize/denormalize/inspect).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

---

**Detailed Changes**: [v0.1.2...v0.1.3](https://github.com/fairspec/fairspec-python/compare/v0.1.2...v0.1.3)

## 0.1.2 (2026-02-12)

### Bug Fixes

- Fixed semantic release ([`ca8bb11`](https://github.com/fairspec/fairspec-python/commit/ca8bb11ad997f04ca391cc639de56801317e9465))

---

**Detailed Changes**: [v0.1.1...v0.1.2](https://github.com/fairspec/fairspec-python/compare/v0.1.1...v0.1.2)
