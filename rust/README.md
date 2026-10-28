# Fairspec Rust

This guide will help you get started with Fairspec Rust. If you are new to the core framework's technologies, please take a look at the [Fairspec standard](https://fairspec.org/) and [Polars DataFrames](https://pola.rs/) documentation.

## Platforms

Fairspec Rust is a single `fairspec` binary — the [terminal](/rust/terminal/) and the [MCP server](/mcp-server/) — and a library of crates. Prebuilt binaries are published for:

- **Linux** x64 and arm64
- **macOS** x64 and arm64
- **Windows** x64

Building from source requires **Rust 1.88+**.

## Installation

Install the binary for your platform from the [GitHub releases](https://github.com/datisthq/fairspec/releases?q=rust-v), checking its checksum:

```bash
curl -fsSL https://fairspec.org/install.sh | sh -s -- terminal
```

Or build it from crates.io:

```bash
cargo install fairspec
```

A crates.io build runs on the Polars that crates.io carries, which is older than the one the prebuilt binaries are built with, so it validates large tables more slowly. To use the library in a project:

```bash
cargo add fairspec
cargo add serde_json
```

## Examples

The examples are bodies of a function returning `Result<(), Box<dyn std::error::Error>>`; see [the library overview](/rust/library/) for the conventions.

Loading a Dataset from Zenodo merging system Zenodo metadata into a user dataset:

```rust
use fairspec::load_dataset;

let dataset = load_dataset("https://zenodo.org/records/10053903")?;

println!("{dataset:?}");
// Some({
//   "id": "https://doi.org/10.5281/zenodo.10053903",
//   ...
// })
```

Validating an in-memory dataset descriptor:

```rust
use fairspec::fairspec_metadata::Descriptor;
use fairspec::{ValidateTableOptions, validate_dataset};
use serde_json::json;

let descriptor: Descriptor = serde_json::from_value(json!({"resources": "bad"}))?;
let report = validate_dataset(descriptor, &ValidateTableOptions::default())?;

println!("{}", report.valid);
// false
println!("{}", serde_json::to_string_pretty(&report.errors)?);
// [
//   {
//     "type": "metadata",
//     "message": "\"bad\" is not of type \"array\"",
//     "jsonPointer": "/resources"
//   }
// ]
```

Loading a dataset from a remote descriptor, saving it locally as a zip archive, and then using it as a local dataset:

```rust
use fairspec::fairspec_dataset::{get_temp_file_path, load_dataset_from_zip, save_dataset_to_zip};
use fairspec::fairspec_metadata::Dataset;
use fairspec::load_dataset;

let archive_path = get_temp_file_path(false, None, Some("zip"))?;
let descriptor = load_dataset(
    "https://raw.githubusercontent.com/fairspec/fairspec-typescript/refs/heads/main/metadata/actions/dataset/fixtures/dataset.json",
)?
.ok_or("unsupported source")?;
let source_dataset: Dataset = serde_json::from_value(descriptor.into())?;

save_dataset_to_zip(&source_dataset, &archive_path, false)?;
let target_dataset = load_dataset_from_zip(&archive_path)?;
println!("{target_dataset:?}");
```

Reading a CSV table:

```rust
use fairspec::fairspec_table::LoadTableOptions;
use fairspec::load_table;
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let table = load_table(&resource, &LoadTableOptions::default())?;

// Load with custom format
let resource = serde_json::from_value(json!({
    "data": "data.csv",
    "fileDialect": {"format": "csv", "delimiter": ";", "headerRows": [1]},
}))?;
let table = load_table(&resource, &LoadTableOptions::default())?;
```

## Reference

Note that the `fairspec` crate re-exports `fairspec-library`, which re-exports the `fairspec-metadata`, `fairspec-dataset` and `fairspec-table` crates as modules.
