---
name: benchmark-development
description: Use when working in benchmark/ — speed and memory of both Fairspec libraries against other validators on generated data. Covers commands, profiles and faults, the correctness gate, the rival adapters and their traps, and publishing the report.
---

# Benchmark

`benchmark/` measures table validation in both Fairspec libraries against eleven other validators
(frictionless-py, qsv, pandera on polars and on DuckDB, dataframely, pointblank, Great
Expectations, patito, Soda Core, datacontract-cli, CSV Validator). It ships nothing
and is not a release component: its commits are `chore(benchmark)`. `benchmark/README.md` is the
user reference; this skill is how to change it without making the numbers lie.

## Commands

- `pnpm -C benchmark test` — types and the unit tests in `-test/`; the root `check:types`,
  `test:unit` and `test` chains include it. Nothing here runs a real tool, and the section has
  no CI job by decision: its numbers need a fixed, idle machine, not a shared runner
- `pnpm -C benchmark setup:rivals` — `uv sync --frozen` in `rivals/` and the pinned qsv binary
  into .cache/bin/. The Fairspec adapters (`fairspec/validate.{py,ts}`) need `pnpm -C python setup` and `pnpm -C typescript build`; the Rust one (`fairspec/rust/`, a member of the `rust/` workspace so it shares its polars patch, profile and CPU flags) is built by its tool with `pnpm -C rust build:dist` before each run, so it never goes stale; that needs the pinned nightly toolchain (`rustup toolchain install nightly-2026-09-01`). Rust runs on the streaming engine like Python; on the in-memory engine it held 2.8GB per GB
- `pnpm -C benchmark measure` — narrowed with `--case`, `--size`, `--tool`; set `--runs 1 --warmup 0`
  and a `--timeout` for multi-gigabyte sizes, and `--memory-max` below what the machine has free
- `pnpm -C benchmark show:report` — merges `.cache/results/`, latest result per case, size and tool
- After a Fairspec change, re-measure only the tools that changed
  (`--tool fairspec-python --tool fairspec-typescript --tool fairspec-rust`); unchanged rivals carry over in the
  report. Rerunning them burns an hour at large sizes and only adds noise
- `pnpm -C benchmark generate:data` — a dataset alone; at ~55 MB/s, 10GB takes about three minutes

## Layout

- `@*.ts` are the scripts' entry points; `generate/` the seeded generator, `profiles/` the data
  shapes, `cases/` what `measure` runs, `harness/` running and the gate, `tools/` one `Tool`
  per validator, `report/` the HTML, `rivals/` the Python adapters and their uv project
- Everything generated lives in `.cache/` (gitignored repo-wide): data, results, the report, qsv. A manifest's
  stored paths are recomputed on reuse, so the cache can move (or be symlinked into a worktree)
  without regenerating 10GB

## Faults and the gate

A tool's numbers count only when it reports exactly the bad cells it can detect, so the faults
must be unambiguous:

- Each fault breaks **exactly one** check. A `type` fault goes in a **nullable** column: in a
  required one, a tool without a type check still catches the null its cast leaves, and passes
  by accident
- Fault values never collide with generated values (`-test/profiles.unit.ts` checks), and never
  use a token pandas reads as missing (`NA`, `N/A`, `null`, `NaN`)
- Profiles generate no duplicate keys: tools count duplicates differently (the second row, or
  both), so keys are checked for cost, not counted
- Faults are a fixed count, not a rate: the TypeScript library stops at 1000 errors per column
  and frictionless limits errors by default
- A check a tool cannot express goes in its `unsupported`, so its faults are not expected; any
  other difference goes in `caveats`, which the report prints. Never adjust a tool's count to
  pass the gate; filter only output that is provably not an error (pandera, below)

## Adding a tool

Only tools that take a declarative, JSON-Schema-like schema qualify (polars schema libraries
included); libraries of hand-written checks (Deequ, cuallee, plain SQL) are out of scope.

Write the adapter (`rivals/validate_<name>.py` printing `{"errorCount": n}`, or a `Tool` that
builds a command, as `tools/qsv.ts` does), list it in `TOOLS` in `tools/index.ts`, and run
`measure --size 10000 --size 100MB --tool <name>` until it reports the expected counts on both
profiles. Read the CSV as text and let the tool do the typing — reading typed columns hides type
errors. Pin new Python packages in `rivals/pyproject.toml`; `exclude-newer` enforces the
repo's seven-day quarantine, and a downloaded binary is pinned to a release at least that old.
A tool whose exit code for invalid data is not 1 sets `okExitCodes` on its `Command`; anything
else counts as a crash. A tool that reports only counts, never the failing rows or cells
(Soda Core, datacontract-cli, patito), is not a full validator and sets `isPartial`: the report lists it after every validator, labels it,
and leaves it out of the validator count and the fastest-rival tile.

## Traps

- Every command runs from the dataset's directory with basenames (the Python CLI rejected an
  absolute `--schema` path)
- Time excludes imports: script adapters start the clock after them (`read_arguments` in
  `rivals/common.py`) and print `elapsedMs`, which `Tool.readElapsedMs` reads in place of wall
  time. Benchmark Fairspec through its library API, never its CLI: a CLI pays start-up the
  library rivals do not. Only true CLIs (qsv, CSV Validator) are timed whole
- frictionless imports every module on `sys.path` named `frictionless_*` as a plugin, which ran
  an adapter twice; adapters are `validate_<name>.py` behind `if __name__ == "__main__"`
- polars cannot cast text to `Datetime` or `Boolean`, so the polars rivals match those by regex
- pandera and pointblank run each check as its own query, so a `scan_csv` LazyFrame re-reads the
  file per check, 10-20x slower; their adapters `read_csv` once, as both document. dataframely
  plans one query and stays lazy. Give a new rival the input its docs show, then check both
- pandera coerces the whole frame at once: one bad cell leaves every column as text and its
  value checks never run (a real miss, reported as such). It also reports a null input as a
  failed cast, which the adapter drops
- qsv copies every valid row to `<input>.valid` — 10GB more disk at 10GB — so the harness links
  that path to `/dev/null`; it reports an empty required enum cell twice, so its errors are
  counted per (row, field)
- DuckDB-backed rivals re-read a `read_csv` view once per query: pandera on Ibis runs one query
  per check, 5x slower than loading an on-disk DuckDB table first (about 0.2GB per GB of CSV in
  the temp dir); Soda v4 batches every check into one query, so its view stays
- datacontract-cli's local CSV server sniffs types and aborts the run on one unparsable cell,
  and skips type checks for CSV; its adapter points a `duckdb` server at text views instead.
  Soda and datacontract compare a text column to a numeric literal, which DuckDB refuses, so
  ranges run on a `TRY_CAST`
- patito's enum check reports the bad values, not the rows: enums go in `constraints`
- CSV Validator exits 3 for invalid data, prints one line per failing rule (a bad number fails
  its type regex and its range, so count per (row, column)), checks UTF-8 in a second pass,
  and its `csv-validator-cmd` script forces `-Xmx1024m`, so the adapter runs `java -jar`
- GNU `time` prefixes "Command exited with non-zero status" when a tool exits 1 for invalid data,
  or "Command terminated by signal N" (exiting 128 + N) when the kernel kills it; the figures are
  on the last line
- Under `--memory-max`, an out-of-memory kill can take the whole systemd scope, `time`
  included: the run exits 143 or with no code at all, and has no `time` figures. The runner
  counts both as `oom` under a cap; only a process that never started is a `crash`
- Peak RSS lies for mmap readers: polars and qsv map the input, once per concurrent scan, so a
  16GB file showed 52GB RSS inside a 12GB cap. The reported memory is peak `RssAnon` summed over
  the tool's process tree, sampled every 50 ms; `peakRssMb` stays in the results only for
  reference
- Fairspec TypeScript's memory is bimodal (2.2–7.7GB at 10GB, Python a steady 2.3GB): the
  nodejs-polars binary uses glibc malloc where py-polars ships jemalloc, and glibc fragments
  under the streaming engine. `LD_PRELOAD` of jemalloc or mimalloc brings it to Python's level,
  time unchanged. One high reading is that, not a measuring fault; don't rerun until it is low.
  In-process polars knobs don't help (morsel/chunk size, prefetch) except `POLARS_MAX_THREADS=6`,
  at 15–30% more time. A source build of nodejs-polars with jemalloc and py-polars' x86-64-v3
  `RUSTFLAGS` gave 19s/2.4GB against npm's 21s/2.4–8GB; adding `cse`/`nightly` gained nothing
- A partial tool's bars are a lighter gray (`--partial`) than the full rivals'

## Publishing

The published comparison is a `.humans/` page: `pnpm -C benchmark show:report --out
../.humans/benchmark-nyc-311.html`. Its numbers carry the date and machine. Regenerate it from a
full run of the case on an otherwise idle machine; a partial or noisy run is not published.
A bare `show:report` merges every file in the results cache, so stale sizes, renamed tools and
partial reruns resurface on the page: pass the result files explicitly, the rivals' files the
current page was built from plus the new Fairspec runs. Re-run a Fairspec number that moves
far from its last review before publishing it; a single cold 10GB run under load is noise

The landing page copies the 10GB numbers by hand into `.livemark/routes/index.tsx`: the hero's
`benchmark` card and the Performance stripe's `measures`. Refresh both with the page, and name a
rival by its backend (`pandera (polars)` and `pandera (DuckDB)` are different results).
