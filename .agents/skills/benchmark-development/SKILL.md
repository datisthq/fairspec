---
name: benchmark-development
description: Use when working in benchmark/ — speed and memory of both Fairspec CLIs against other validators on generated data. Covers commands, profiles and faults, the correctness gate, the rival adapters and their traps, and publishing the report.
---

# Benchmark

`benchmark/` measures `table validate` in both Fairspec CLIs against six other validators
(frictionless-py, qsv, pandera, dataframely, pointblank, Great Expectations). It ships nothing
and is not a release component: its commits are `chore(benchmark)`. `benchmark/README.md` is the
user reference; this skill is how to change it without making the numbers lie.

## Commands

- `pnpm -C benchmark test` — types and the unit tests in `-test/`; the root `check:types`,
  `test:unit` and `test` chains include it. Nothing here runs a real tool, and the section has
  no CI job by decision: its numbers need a fixed, idle machine, not a shared runner
- `pnpm -C benchmark setup:rivals` — `uv sync --frozen` in `rivals/` and the pinned qsv binary
  into `.cache/bin/`. The Fairspec CLIs need `pnpm -C python setup` and `pnpm -C typescript build`
- `pnpm -C benchmark measure` — narrowed with `--case`, `--size`, `--tool`; set `--runs 1 --warmup 0`
  and a `--timeout` for multi-gigabyte sizes, and `--memory-max` below what the machine has free
- `pnpm -C benchmark show:report` — merges `.cache/results/`, latest result per case, size and tool
- After a Fairspec change, re-measure only the tools that changed
  (`--tool fairspec-python --tool fairspec-typescript`); unchanged rivals carry over in the
  report. Rerunning them burns an hour at large sizes and only adds noise
- `pnpm -C benchmark generate:data` — a dataset alone; at ~55 MB/s, 16GB takes about five minutes

## Layout

- `@*.ts` are the scripts' entry points; `generate/` the seeded generator, `profiles/` the data
  shapes, `cases/` what `measure` runs, `harness/` running and the gate, `tools/` one `Tool`
  per validator, `report/` the HTML, `rivals/` the Python adapters and their uv project
- Everything generated lives in `.cache/` (gitignored repo-wide): data, results, the report, qsv

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

Write the adapter (`rivals/validate_<name>.py` printing `{"errorCount": n}`, or a `Tool` that
builds a command, as `tools/qsv.ts` does), list it in `TOOLS` in `tools/index.ts`, and run
`measure --size 10000 --size 100MB --tool <name>` until it reports the expected counts on both
profiles. Read the CSV as text and let the tool do the typing — reading typed columns hides type
errors. Pin new Python packages in `rivals/pyproject.toml`; `exclude-newer` enforces the
repo's seven-day quarantine, and a downloaded binary is pinned to a release at least that old.

## Traps

- The Python CLI rejects an absolute `--schema` path, so every command runs from the dataset's
  directory with basenames
- frictionless imports every module on `sys.path` named `frictionless_*` as a plugin, which ran
  an adapter twice; adapters are `validate_<name>.py` behind `if __name__ == "__main__"`
- polars cannot cast text to `Datetime` or `Boolean`, so the polars rivals match those by regex
- pandera and pointblank run each check as its own query, so a `scan_csv` LazyFrame re-reads the
  file per check, 10-20x slower; their adapters `read_csv` once, as both document. dataframely
  plans one query and stays lazy. Give a new rival the input its docs show, then check both
- pandera coerces the whole frame at once: one bad cell leaves every column as text and its
  value checks never run (a real miss, reported as such). It also reports a null input as a
  failed cast, which the adapter drops
- qsv copies every valid row to `<input>.valid` — 16GB more disk at 16GB — so the harness links
  that path to `/dev/null`; it reports an empty required enum cell twice, so its errors are
  counted per (row, field)
- GNU `time` prefixes "Command exited with non-zero status" when a tool exits 1 for invalid data,
  or "Command terminated by signal N" (exiting 128 + N) when the kernel kills it; the figures are
  on the last line
- Under `--memory-max`, an out-of-memory kill can take the whole systemd scope with SIGTERM
  (exit 143), `time` included, so the run has no `time` figures at all
- Peak RSS lies for mmap readers: polars and qsv map the input, once per concurrent scan, so a
  16GB file showed 52GB RSS inside a 12GB cap. The reported memory is peak `RssAnon` summed over
  the tool's process tree, sampled every 50 ms; `peakRssMb` stays in the results only for
  reference

## Publishing

The published comparison is a `.humans/` page: `pnpm -C benchmark show:report --out
../.humans/benchmark-nyc-311.html`. Its numbers carry the date and machine. Regenerate it from a
full run of the case on an otherwise idle machine; a partial or noisy run is not published.
