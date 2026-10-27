---
title: Working with JSON tables in Rust
label: JSON
path: /rust/library/json/
order: 12
---

JSON file handling with automatic format detection and high-performance data operations.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

The JSON plugin provides:

- `load_json_table` - Load JSON files into tables
- `save_json_table` - Save tables to JSON files
- `JsonPlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "table.json"}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;
// Standard JSON array of objects format
```

Unlike the other loaders, `load_json_table` returns `Option<Table>`: `None` when the data under the dialect's `jsonPointer` is not a table.

## Basic Usage

### Loading JSON Files

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

let options = LoadTableOptions::default();

// Load from local file
let resource = serde_json::from_value(json!({"data": "data.json"}))?;
let table = load_json_table(&resource, &options)?;

// Load from remote URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.json"}))?;
let table = load_json_table(&resource, &options)?;

// Load multiple files (concatenated)
let resource = serde_json::from_value(json!({"data": ["file1.json", "file2.json"]}))?;
let table = load_json_table(&resource, &options)?;
```

### Saving JSON Files

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_json_table};
use serde_json::json;

// Save with default options
save_json_table(table.clone(), &SaveTableOptions {
    path: "output.json".into(),
    ..Default::default()
})?;

// Save with explicit format
save_json_table(table, &SaveTableOptions {
    path: "output.json".into(),
    file_dialect: Some(serde_json::from_value(json!({"format": "json"}))?),
    overwrite: Some(true),
    ..Default::default()
})?;
```

## Standard Format

JSON tables use an array of objects format:

```json
[
  { "id": 1, "name": "Alice", "age": 30 },
  { "id": 2, "name": "Bob", "age": 25 }
]
```

## Advanced Features

### JSON Pointer Extraction

Extract data from nested objects using `jsonPointer`:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

// Input: {"users": [{"id": 1, "name": "Alice"}]}
let resource = serde_json::from_value(json!({
    "data": "data.json",
    "fileDialect": {"format": "json", "jsonPointer": "/users"},
}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;
```

### Column Selection

Select specific columns using `columnNames`:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

// Only load specific columns
let resource = serde_json::from_value(json!({
    "data": "data.json",
    "fileDialect": {"format": "json", "columnNames": ["name", "age"]},
}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;
```

### Array Format Handling

Handle CSV-style array data with `rowType: "array"`:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_json_table};
use serde_json::json;

// Input: [["id", "name"], [1, "Alice"], [2, "Bob"]]
let resource = serde_json::from_value(json!({
    "data": "data.json",
    "fileDialect": {"format": "json", "rowType": "array"},
}))?;
let table = load_json_table(&resource, &LoadTableOptions::default())?;
```

### Saving with JSON Pointer

Wrap data in a nested structure when saving:

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_json_table};
use serde_json::json;

// Output: {"users": [{"id": 1, "name": "Alice"}]}
save_json_table(table, &SaveTableOptions {
    path: "output.json".into(),
    file_dialect: Some(serde_json::from_value(json!({
        "format": "json",
        "jsonPointer": "/users",
    }))?),
    ..Default::default()
})?;
```
