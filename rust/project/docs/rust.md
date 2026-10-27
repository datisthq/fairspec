---
title: Rust
label: Rust
path: /rust/library/
order: 20
icon: file-code
---

The Rust crates provide a fast data management framework built on the Fairspec standard and Polars DataFrames. They support a wide range of tabular formats — CSV, TSV, JSON, JSONL, XLSX, ODS, Arrow, Parquet, SQLite — plus inline data. Function names are the Python library's, so a Python example translates call by call.

The pages in this section show how to load, validate, and save tables in each format.

## Conventions

The `fairspec` crate exports the high-level API (`load_table`, `validate_dataset`, …) and re-exports the lower crates as modules: `fairspec::fairspec_metadata` (descriptors and models), `fairspec::fairspec_dataset` (files and dataset plugins) and `fairspec::fairspec_table` (Polars tables and format plugins).

Examples in this section are bodies of a function returning `Result<(), Box<dyn std::error::Error>>`, so `?` works on every call. Descriptors are built from JSON with `serde_json`, using the same camelCase keys as a `dataset.json`:

```rust
use fairspec::fairspec_metadata::Resource;
use fairspec::{ValidateTableOptions, validate_table};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource: Resource = serde_json::from_value(json!({
        "data": "data.csv",
        "fileDialect": {"format": "csv", "delimiter": ";"},
    }))?;

    let report = validate_table(&resource, &ValidateTableOptions::default())?;
    println!("{}", report.valid);
    Ok(())
}
```

Keyword arguments of the Python library are positional `Option` parameters or an options struct with a `Default`, so `..Default::default()` leaves the rest unset. The API is synchronous.

## Remote URLs

Loading functions fetch any `http(s)` URL they are given: descriptors, schemas and their `$ref`s, data files, and CKAN, Zenodo and GitHub datasets. A service handling untrusted URLs should refuse hosts that resolve to a loopback, private, link-local (cloud metadata at `169.254.169.254` among them) or other non-public address:

```rust
use fairspec::fairspec_metadata::{HostPolicy, set_host_policy};

set_host_policy(Some(HostPolicy::Public)); // None falls back to FAIRSPEC_HOST_POLICY
```

The check runs on the address actually connected to and again on every redirect, and proxy settings are ignored under it. The policy is process-wide. Without a call, the `FAIRSPEC_HOST_POLICY` environment variable (`any` or `public`) decides, and the default is `any`. The MCP server starts with `public`.
