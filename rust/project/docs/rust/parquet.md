---
title: Working with Parquet in Rust
label: Parquet
path: /rust/library/parquet/
order: 17
---

Apache Parquet file handling with high-performance columnar data processing and compression.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

The Parquet plugin provides:

- `load_parquet_table` - Load Parquet files into tables
- `save_parquet_table` - Save tables to Parquet files
- `ParquetPlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_parquet_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "table.parquet"}))?;
let table = load_parquet_table(&resource, &LoadTableOptions::default())?;
// Efficient columnar format with compression
```

## Basic Usage

### Loading Parquet Files

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_parquet_table};
use serde_json::json;

let options = LoadTableOptions::default();

// Load from local file
let resource = serde_json::from_value(json!({"data": "data.parquet"}))?;
let table = load_parquet_table(&resource, &options)?;

// Load from remote URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.parquet"}))?;
let table = load_parquet_table(&resource, &options)?;

// Load multiple files (concatenated)
let resource = serde_json::from_value(json!({"data": ["file1.parquet", "file2.parquet"]}))?;
let table = load_parquet_table(&resource, &options)?;
```

### Saving Parquet Files

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_parquet_table};

// Save with default options
save_parquet_table(table, &SaveTableOptions {
    path: "output.parquet".into(),
    ..Default::default()
})?;
```

## Advanced Features

### Remote File Loading

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_parquet_table};
use serde_json::json;

// Load from URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.parquet"}))?;
let table = load_parquet_table(&resource, &LoadTableOptions::default())?;

// Load multiple remote files
let resource = serde_json::from_value(json!({
    "data": [
        "https://api.example.com/data-2023.parquet",
        "https://api.example.com/data-2024.parquet",
    ],
}))?;
let table = load_parquet_table(&resource, &LoadTableOptions::default())?;
```
