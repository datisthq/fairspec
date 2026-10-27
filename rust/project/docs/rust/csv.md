---
title: Working with CSV in Rust
label: CSV
path: /rust/library/csv/
order: 10
---

Comprehensive CSV file handling with automatic format detection, advanced header processing, and high-performance data operations.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

The CSV plugin provides these capabilities:

- `load_csv_table` - Load CSV/TSV files into tables
- `save_csv_table` - Save tables to CSV/TSV files
- `CsvPlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "table.csv"}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
// the column types will be automatically inferred
// or you can provide a Table Schema
```

## Basic Usage

### Loading CSV Files

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

let options = LoadTableOptions::default();

// Load a simple CSV file
let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let table = load_csv_table(&resource, &options)?;

// Load with custom format
let resource = serde_json::from_value(json!({
    "data": "data.csv",
    "fileDialect": {"format": "csv", "delimiter": ";", "headerRows": [1]},
}))?;
let table = load_csv_table(&resource, &options)?;

// Load multiple CSV files (concatenated)
let resource = serde_json::from_value(json!({
    "data": ["part1.csv", "part2.csv", "part3.csv"],
}))?;
let table = load_csv_table(&resource, &options)?;
```

### Saving CSV Files

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_csv_table};
use serde_json::json;

// Save with default options
save_csv_table(table.clone(), &SaveTableOptions {
    path: "output.csv".into(),
    ..Default::default()
})?;

// Save with custom format
save_csv_table(table.clone(), &SaveTableOptions {
    path: "output.csv".into(),
    file_dialect: Some(serde_json::from_value(json!({
        "format": "csv",
        "delimiter": "\t",
        "quoteChar": "'",
    }))?),
    overwrite: Some(true),
    ..Default::default()
})?;

// Save as TSV
save_csv_table(table, &SaveTableOptions {
    path: "output.tsv".into(),
    file_dialect: Some(serde_json::from_value(json!({"format": "tsv"}))?),
    ..Default::default()
})?;
```

A save takes the table by value: `Table` is a Polars `LazyFrame`, so `clone()` it to save one table more than once.

### Automatic Format Detection

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

// Format is automatically detected when not specified
let resource = serde_json::from_value(json!({"data": "unknown-dialect.csv"}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
// The CSV plugin will automatically infer delimiter, quote characters, etc.

// You can also explicitly specify the format if detection isn't accurate
let resource = serde_json::from_value(json!({
    "data": "data.csv",
    "fileDialect": {"format": "csv", "delimiter": ",", "quoteChar": "\"", "headerRows": [1]},
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
```

## Advanced Features

### Multi-Header Row Processing

```rust
// CSV with multiple header rows:
// Year,2023,2023,2024,2024
// Quarter,Q1,Q2,Q1,Q2
// Revenue,100,120,110,130

use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": "multi-header.csv",
    "fileDialect": {"format": "csv", "headerRows": [1, 2], "headerJoin": "_"},
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
// Resulting columns: ["Year_Quarter", "2023_Q1", "2023_Q2", "2024_Q1", "2024_Q2"]
```

### Comment Row Handling

```rust
// CSV with comment rows:
// # This is a comment
// # Generated on 2024-01-01
// Name,Age,City
// John,25,NYC

use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": "with-comments.csv",
    "fileDialect": {"format": "csv", "commentRows": [1, 2], "headerRows": [3]},
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;

// Or use commentPrefix to skip lines starting with a specific character
let resource = serde_json::from_value(json!({
    "data": "with-comments.csv",
    "fileDialect": {"format": "csv", "commentPrefix": "#", "headerRows": [1]},
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
```

### Remote File Loading

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

// Load from URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.csv"}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;

// Load multiple remote files
let resource = serde_json::from_value(json!({
    "data": [
        "https://api.example.com/data-2023.csv",
        "https://api.example.com/data-2024.csv",
    ],
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
```
