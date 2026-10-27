---
title: Working with JSONL tables in Rust
label: JSONL
path: /rust/library/jsonl/
order: 13
---

JSONL (JSON Lines) file handling with automatic format detection and high-performance data operations.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

JSONL is handled by the JSON plugin, which provides:

- `load_json_table` - Load JSONL files into tables
- `save_json_table` - Save tables to JSONL files
- `JsonPlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "table.jsonl"}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;
// Newline-delimited JSON objects
```

## Basic Usage

### Loading JSONL Files

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

let options = LoadTableOptions::default();

// Load from local file
let resource = serde_json::from_value(json!({"data": "data.jsonl"}))?;
let table = load_json_table(&resource, &options)?;

// Load with explicit format
let resource = serde_json::from_value(json!({
    "data": "data.jsonl",
    "fileDialect": {"format": "jsonl"},
}))?;
let table = load_json_table(&resource, &options)?;

// Load multiple files (concatenated)
let resource = serde_json::from_value(json!({"data": ["file1.jsonl", "file2.jsonl"]}))?;
let table = load_json_table(&resource, &options)?;
```

### Saving JSONL Files

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_json_table};
use serde_json::json;

// Save as JSONL
save_json_table(table, &SaveTableOptions {
    path: "output.jsonl".into(),
    file_dialect: Some(serde_json::from_value(json!({"format": "jsonl"}))?),
    ..Default::default()
})?;
```

## Standard Format

JSONL uses newline-delimited JSON objects:

```jsonl
{"id": 1, "name": "Alice", "age": 30}
{"id": 2, "name": "Bob", "age": 25}
{"id": 3, "name": "Charlie", "age": 35}
```

A blank line, or one holding only spaces, tabs and a carriage return, is not a row and takes no row number.

## Advanced Features

### Column Selection

Select specific columns using `columnNames`:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

// Only load specific columns
let resource = serde_json::from_value(json!({
    "data": "data.jsonl",
    "fileDialect": {"format": "jsonl", "columnNames": ["name", "age"]},
}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;
```

### Array Format Handling

Handle CSV-style array data with `rowType: "array"`:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

// Input JSONL with arrays:
// ["id", "name"]
// [1, "Alice"]
// [2, "Bob"]

let resource = serde_json::from_value(json!({
    "data": "data.jsonl",
    "fileDialect": {"format": "jsonl", "rowType": "array"},
}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;
```

### Remote File Loading

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

// Load from URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.jsonl"}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;

// Load multiple remote files
let resource = serde_json::from_value(json!({
    "data": [
        "https://api.example.com/logs-2023.jsonl",
        "https://api.example.com/logs-2024.jsonl",
    ],
}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;
```
