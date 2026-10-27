---
title: Working with SQLite in Rust
label: SQLite
path: /rust/library/sqlite/
order: 18
---

SQLite database file handling with table loading and saving capabilities.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

The SQLite plugin provides:

- `load_sqlite_table` - Load tables from SQLite databases
- `save_sqlite_table` - Save tables to SQLite databases
- `SqlitePlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_sqlite_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": "database.db",
    "fileDialect": {"format": "sqlite", "tableName": "users"},
}))?;
let table = load_sqlite_table(&resource, &LoadTableOptions::default())?;
// column types will be automatically inferred from database schema
```

## Basic Usage

### Loading SQLite Tables

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_sqlite_table};
use serde_json::json;

// Load a table from SQLite database
let resource = serde_json::from_value(json!({
    "data": "data.db",
    "fileDialect": {"format": "sqlite", "tableName": "products"},
}))?;
let table = load_sqlite_table(&resource, &LoadTableOptions::default())?;

// Load from a specific path
let resource = serde_json::from_value(json!({
    "data": "/path/to/database.db",
    "fileDialect": {"format": "sqlite", "tableName": "orders"},
}))?;
let table = load_sqlite_table(&resource, &LoadTableOptions::default())?;
```

### Saving SQLite Tables

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_sqlite_table};
use serde_json::json;

// Save table to SQLite database
save_sqlite_table(table.clone(), &SaveTableOptions {
    path: "output.db".into(),
    file_dialect: Some(serde_json::from_value(json!({
        "format": "sqlite",
        "tableName": "results",
    }))?),
    ..Default::default()
})?;

// Overwrite existing table
save_sqlite_table(table, &SaveTableOptions {
    path: "output.db".into(),
    file_dialect: Some(serde_json::from_value(json!({
        "format": "sqlite",
        "tableName": "results",
    }))?),
    overwrite: Some(true),
    ..Default::default()
})?;
```

## Advanced Features

### Schema Inference

Table schemas are automatically inferred from SQLite table definitions:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_sqlite_table};
use serde_json::json;

// Field types are automatically detected from database schema
let resource = serde_json::from_value(json!({
    "data": "shop.db",
    "fileDialect": {"format": "sqlite", "tableName": "products"},
}))?;
let table = load_sqlite_table(&resource, &LoadTableOptions::default())?;
// Types like INTEGER, TEXT, REAL are mapped to appropriate Table Schema types
```

### Creating New Tables

When saving, the plugin automatically creates the table structure:

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_sqlite_table};
use serde_json::json;

// Creates a new database file with the specified table
save_sqlite_table(table, &SaveTableOptions {
    path: "new-database.db".into(),
    file_dialect: Some(serde_json::from_value(json!({
        "format": "sqlite",
        "tableName": "my_data",
    }))?),
    ..Default::default()
})?;
```

### Working with Table Schema

You can provide a custom Table Schema when saving:

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_sqlite_table};
use serde_json::json;

save_sqlite_table(table, &SaveTableOptions {
    path: "output.db".into(),
    file_dialect: Some(serde_json::from_value(json!({
        "format": "sqlite",
        "tableName": "customers",
    }))?),
    table_schema: Some(serde_json::from_value(json!({
        "properties": {
            "id": {"type": "integer"},
            "name": {"type": "string"},
            "email": {"type": "string"},
        },
    }))?),
    ..Default::default()
})?;
```
