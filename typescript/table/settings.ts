export const NUMBER_COLUMN_NAME = "fairspec:number"
export const ERROR_COLUMN_NAME = "fairspec:error"

// nodejs-polars takes an engine on collectSync only, and polars ignores the
// `streaming` option of async collect, but it reads this variable on every query.
// Without it a 1GB validation takes 24s in 8GB instead of 7s in 3GB; "0" opts out.
process.env.POLARS_FORCE_STREAMING ??= "1"

// Each check scans the whole source, so peak memory grows with the number in flight.
// At four, validating 10 columns fits 768MB for a 1GB file and 1GB for a 3GB one.
// A key check holds every key in memory (3GB for 10M rows), so they stay serial.
export const INSPECT_COLUMN_CONCURRENCY = 4
export const INSPECT_ROW_CONCURRENCY = 1
