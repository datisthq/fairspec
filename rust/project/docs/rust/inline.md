---
title: Working with Inline Data tables in Rust
label: Inline Data
path: /rust/library/inline/
order: 19
---

Inline data handling for tables embedded directly in resource definitions.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

The inline plugin provides:

- `load_inline_table` - Load tables from inline data
- `InlinePlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_inline_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"},
    ],
}))?;
let table = load_inline_table(&resource, &LoadTableOptions::default())?;
```

## Basic Usage

### Object Format Data

The most common format is an array of objects:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_inline_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": [
        {"id": 1, "name": "english", "native": "English"},
        {"id": 2, "name": "chinese", "native": "中文"},
        {"id": 3, "name": "spanish", "native": "Español"},
    ],
}))?;
let table = load_inline_table(&resource, &LoadTableOptions::default())?;
```

### Array Format Data

You can also use array-of-arrays format with the first row as headers:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_inline_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": [
        ["id", "name", "native"],
        [1, "english", "English"],
        [2, "chinese", "中文"],
        [3, "spanish", "Español"],
    ],
}))?;
let table = load_inline_table(&resource, &LoadTableOptions::default())?;
```

## Advanced Features

### With Table Schema

Provide a Table Schema for type validation and conversion:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_inline_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": [
        {"id": 1, "name": "english", "active": true},
        {"id": 2, "name": "chinese", "active": false},
    ],
    "tableSchema": {
        "properties": {
            "id": {"type": "integer"},
            "name": {"type": "string"},
            "active": {"type": "boolean"},
        },
    },
}))?;
let table = load_inline_table(&resource, &LoadTableOptions::default())?;
```

### Mixed with File Data

Inline data can be used alongside file-based resources in datasets:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_csv_table, load_inline_table};
use serde_json::json;

let options = LoadTableOptions::default();

// Load inline reference data
let resource = serde_json::from_value(json!({
    "name": "languages",
    "data": [
        {"id": 1, "name": "english"},
        {"id": 2, "name": "chinese"},
    ],
}))?;
let languages = load_inline_table(&resource, &options)?;

// Load main data from file
let resource = serde_json::from_value(json!({"name": "users", "data": "users.csv"}))?;
let users = load_csv_table(&resource, &options)?;
```

### Resource Metadata

You can include metadata with inline data resources:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_inline_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "name": "countries",
    "titles": [{"title": "Country Reference Data"}],
    "descriptions": [{"description": "ISO country codes and names", "descriptionType": "Abstract"}],
    "data": [
        {"code": "US", "name": "United States"},
        {"code": "CN", "name": "China"},
        {"code": "ES", "name": "Spain"},
    ],
    "tableSchema": {
        "properties": {
            "code": {"type": "string"},
            "name": {"type": "string"},
        },
    },
}))?;
let table = load_inline_table(&resource, &LoadTableOptions::default())?;
```
