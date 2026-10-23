# Benchmark

How fast, and in how much memory, Fairspec validates a large CSV table — its Python,
TypeScript and Rust libraries side by side with other validators, all checking the same generated data
against the same Table Schema:

| tool                | how it runs                                                                       |
| ------------------- | --------------------------------------------------------------------------------- |
| Fairspec Python     | `validate_table` from `fairspec/validate.py`, in the `python/` venv               |
| Fairspec TypeScript | `validateTable` from `fairspec/validate.ts`, on the workspace build               |
| Fairspec Rust       | `validate_table` from `fairspec/rustlang/`, a release build of `rustlang/library` |
| frictionless-py     | `frictionless.validate` with the schema translated                                |
| qsv                 | `qsv validate` with the schema translated to JSON Schema                          |
| pandera (polars)    | a `pandera.polars` schema with coercion, over a read frame                        |
| dataframely         | a generated `dataframely` schema, `filter(cast=True)`                             |
| pointblank (polars) | a `pointblank.Validate` plan of column checks, over a read frame                  |
| Great Expectations  | an ephemeral GX context over a pandas DataFrame                                   |
| pandera (DuckDB)    | a `pandera.ibis` schema over the CSV loaded into DuckDB                           |
| patito (polars)     | a generated `patito.Model`, over a read frame                                     |
| Soda Core (DuckDB)  | a Soda v4 contract over a DuckDB view of the CSV                                  |
| datacontract-cli    | an ODCS data contract tested against a DuckDB view of the CSV                     |
| CSV Validator       | the UK National Archives validator with a CSV Schema 1.1 file                     |

Every rival takes a declarative schema, as Fairspec does; libraries of hand-written checks
(Deequ, cuallee, plain SQL) are out of scope. The Python ones live in `rivals/`, a uv project of
its own, so their dependencies never reach the Fairspec packages; qsv and CSV Validator (Java)
are downloaded by `setup:rivals`, which needs `java` on the PATH. Linux only: memory is the peak
anonymous memory of the tool's processes, sampled from `/proc` (memory-mapped pages of the input
are excluded, so polars and qsv compare fairly), and the cap comes from `systemd-run`.

Time leaves out interpreter start and imports: every script that calls a library (both Fairspec
adapters and every rival in `rivals/`) starts its clock after its imports and prints
`{"errorCount", "elapsedMs"}`. qsv and CSV Validator are CLIs and are timed as whole processes.

## Usage

```bash
pnpm -C python setup && pnpm -C typescript build   # the two Fairspec libraries
pnpm -C benchmark setup:rivals                     # uv sync rivals/, download qsv and CSV Validator
pnpm -C benchmark measure --case nyc-311 --size 1GB
pnpm -C benchmark show:report                      # .cache/report.html
```

`measure` runs every size of every case unless narrowed with `--case`, `--size` or `--tool`
(each repeatable). `--runs` (3) and `--warmup` (1) set the series per tool, `--timeout` its
limit in seconds (3600), and `--memory-max` caps each tool, for example `10G`. Results land in
`.cache/results/`, one file per invocation; `show:report` keeps the latest result per case,
size and tool, and `--out` writes the page elsewhere.

`generate:data` writes a dataset without measuring anything:

```bash
pnpm -C benchmark generate:data --profile nyc-311 --size 10GB
```

## Data

Every dataset is generated from a seed, so each tool reads identical bytes, and cached in
`.cache/data/` until its profile, size, seed or fault count changes. A size is a row count or
a byte target such as `10GB`. A profile in `profiles/` names its columns, how to draw each
value, its Table Schema and its faults:

- `simple` — ten columns of mixed types, one of each common constraint
- `nyc-311` — the 41 columns of NYC 311 Service Requests with plausible value spreads, about
  19 million rows at 10GB

A case in `cases/` picks a profile, its sizes and its fault count.

## Correctness

Each file carries a fixed number of bad cells, spread over the whole file, each breaking exactly
one check: a wrong type, a missing required value, an out-of-range number, a value outside an
enum, a pattern mismatch. A tool's result counts only when it reports exactly the bad cells it
is able to detect; a tool that cannot express a check (pointblank, Great Expectations, patito and
pandera on DuckDB have no per-cell type check over text) is not expected to find that check's
faults. Everything a tool
checks differently is listed in the report.
