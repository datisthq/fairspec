---
title: Working with Arrow in Rust
label: Arrow
path: /rust/library/arrow/
order: 16
---

Apache Arrow IPC file handling with high-performance columnar data processing.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

The Arrow plugin provides:

- `load_arrow_table` - Load Arrow IPC files into tables
- `save_arrow_table` - Save tables to Arrow IPC files
- `ArrowPlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_arrow_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "table.arrow"}))?;
let table = load_arrow_table(&resource, &LoadTableOptions::default())?;
// High-performance columnar format
```

## Basic Usage

### Loading Arrow Files

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_arrow_table};
use serde_json::json;

let options = LoadTableOptions::default();

// Load from local file
let resource = serde_json::from_value(json!({"data": "data.arrow"}))?;
let table = load_arrow_table(&resource, &options)?;

// Load from remote URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.arrow"}))?;
let table = load_arrow_table(&resource, &options)?;

// Load multiple files (concatenated)
let resource = serde_json::from_value(json!({"data": ["file1.arrow", "file2.arrow"]}))?;
let table = load_arrow_table(&resource, &options)?;
```

### Saving Arrow Files

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_arrow_table};

// Save with default options
save_arrow_table(table, &SaveTableOptions {
    path: "output.arrow".into(),
    ..Default::default()
})?;
```

## Advanced Features

### Remote File Loading

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_arrow_table};
use serde_json::json;

// Load from URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.arrow"}))?;
let table = load_arrow_table(&resource, &LoadTableOptions::default())?;

// Load multiple remote files
let resource = serde_json::from_value(json!({
    "data": [
        "https://api.example.com/data-2023.arrow",
        "https://api.example.com/data-2024.arrow",
    ],
}))?;
let table = load_arrow_table(&resource, &LoadTableOptions::default())?;
```
