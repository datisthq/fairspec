export const NUMBER_COLUMN_NAME = "fairspec:number"
export const ERROR_COLUMN_NAME = "fairspec:error"
export const COUNT_COLUMN_NAME = "fairspec:count"

// nodejs-polars takes an engine on collectSync only, and polars ignores the
// `streaming` option of async collect, but it reads this variable on every query.
// Without it a 1GB validation takes 24s in 8GB instead of 7s in 3GB; "0" opts out.
process.env.POLARS_FORCE_STREAMING ??= "1"

// A key probe holds every distinct key in memory (3GB for 10M rows), so they stay serial.
export const INSPECT_ROW_CONCURRENCY = 1
