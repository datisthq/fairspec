# Benchmark

How fast, and in how much memory, Fairspec validates a large CSV table — its Python and
TypeScript CLIs side by side with other validators, all checking the same generated data
against the same Table Schema:

| tool                | how it runs                                                      |
| ------------------- | ---------------------------------------------------------------- |
| Fairspec Python     | `fairspec table validate` from the `python/` workspace           |
| Fairspec TypeScript | `fairspec table validate` from `typescript/terminal/build/`      |
| frictionless-py     | `frictionless.validate` with the schema translated               |
| qsv                 | `qsv validate` with the schema translated to JSON Schema         |
| pandera (polars)    | a `pandera.polars` schema with coercion, over a read frame       |
| dataframely         | a generated `dataframely` schema, `filter(cast=True)`            |
| pointblank (polars) | a `pointblank.Validate` plan of column checks, over a read frame |
| Great Expectations  | an ephemeral GX context over a pandas DataFrame                  |

The other tools live in `rivals/`, a uv project of its own, so their dependencies never reach
the Fairspec packages. Linux only: memory is the peak anonymous memory of the tool's processes,
sampled from `/proc` (memory-mapped pages of the input are excluded, so polars and qsv compare
fairly), and the cap comes from `systemd-run`.

## Usage

```bash
pnpm -C python setup && pnpm -C typescript build   # the two Fairspec CLIs
pnpm -C benchmark setup:rivals                     # uv sync rivals/, download qsv
pnpm -C benchmark measure --case nyc-311 --size 1GB
pnpm -C benchmark show:report                      # .cache/report.html
```

`measure` runs every size of every case unless narrowed with `--case`, `--size` or `--tool`
(each repeatable). `--runs` (3) and `--warmup` (1) set the series per tool, `--timeout` its
limit in seconds (3600), and `--memory-max` caps each tool, for example `12G`. Results land in
`.cache/results/`, one file per invocation; `show:report` keeps the latest result per case,
size and tool, and `--out` writes the page elsewhere.

`generate:data` writes a dataset without measuring anything:

```bash
pnpm -C benchmark generate:data --profile nyc-311 --size 16GB
```

## Data

Every dataset is generated from a seed, so each tool reads identical bytes, and cached in
`.cache/data/` until its profile, size, seed or fault count changes. A size is a row count or
a byte target such as `16GB`. A profile in `profiles/` names its columns, how to draw each
value, its Table Schema and its faults:

- `simple` — ten columns of mixed types, one of each common constraint
- `nyc-311` — the 41 columns of NYC 311 Service Requests with plausible value spreads, about
  31 million rows at 16GB

A case in `cases/` picks a profile, its sizes and its fault count.

## Correctness

Each file carries a fixed number of bad cells, spread over the whole file, each breaking exactly
one check: a wrong type, a missing required value, an out-of-range number, a value outside an
enum, a pattern mismatch. A tool's result counts only when it reports exactly the bad cells it
is able to detect; a tool that cannot express a check (pointblank and Great Expectations have no
per-cell type check over text) is not expected to find that check's faults. Everything a tool
checks differently is listed in the report.
