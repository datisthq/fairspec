---
title: Working with TSV in Rust
label: TSV
path: /rust/library/tsv/
order: 11
---

Tab-separated values (TSV) file handling with automatic format detection and high-performance data operations.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

TSV is handled by the CSV plugin, which provides:

- `load_csv_table` - Load TSV files into tables
- `save_csv_table` - Save tables to TSV files
- `CsvPlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "table.tsv"}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
// the column types will be automatically inferred
```

## Basic Usage

### Loading TSV Files

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

let options = LoadTableOptions::default();

// Load a simple TSV file
let resource = serde_json::from_value(json!({"data": "data.tsv"}))?;
let table = load_csv_table(&resource, &options)?;

// Load with explicit format
let resource = serde_json::from_value(json!({
    "data": "data.tsv",
    "fileDialect": {"format": "tsv", "headerRows": [1]},
}))?;
let table = load_csv_table(&resource, &options)?;

// Load multiple TSV files (concatenated)
let resource = serde_json::from_value(json!({
    "data": ["part1.tsv", "part2.tsv", "part3.tsv"],
}))?;
let table = load_csv_table(&resource, &options)?;
```

### Saving TSV Files

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_csv_table};
use serde_json::json;

// Save with default options
save_csv_table(table.clone(), &SaveTableOptions {
    path: "output.tsv".into(),
    file_dialect: Some(serde_json::from_value(json!({"format": "tsv"}))?),
    ..Default::default()
})?;

// Save with line terminator option
save_csv_table(table, &SaveTableOptions {
    path: "output.tsv".into(),
    file_dialect: Some(serde_json::from_value(json!({
        "format": "tsv",
        "lineTerminator": "\r\n",
    }))?),
    overwrite: Some(true),
    ..Default::default()
})?;
```

## Advanced Features

### Multi-Header Row Processing

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

// TSV with multiple header rows
let resource = serde_json::from_value(json!({
    "data": "multi-header.tsv",
    "fileDialect": {"format": "tsv", "headerRows": [1, 2], "headerJoin": "_"},
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
```

### Comment Handling

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

// TSV with comment lines
let resource = serde_json::from_value(json!({
    "data": "with-comments.tsv",
    "fileDialect": {"format": "tsv", "commentPrefix": "#", "headerRows": [1]},
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;

// Or specify specific comment row numbers
let resource = serde_json::from_value(json!({
    "data": "with-comments.tsv",
    "fileDialect": {"format": "tsv", "commentRows": [1, 2], "headerRows": [3]},
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
```

### Remote File Loading

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table};
use serde_json::json;

// Load from URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.tsv"}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;

// Load multiple remote files
let resource = serde_json::from_value(json!({
    "data": [
        "https://api.example.com/data-2023.tsv",
        "https://api.example.com/data-2024.tsv",
    ],
}))?;
let table = load_csv_table(&resource, &LoadTableOptions::default())?;
```
