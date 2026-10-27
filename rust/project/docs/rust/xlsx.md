---
title: Working with XLSX in Rust
label: XLSX
path: /rust/library/xlsx/
order: 14
---

Excel (.xlsx) file handling with sheet selection, advanced header processing, and high-performance data operations.

## Installation

```bash
cargo add --git https://github.com/datisthq/fairspec fairspec
cargo add serde_json
```

## Getting Started

The XLSX plugin provides:

- `load_xlsx_table` - Load Excel files into tables
- `save_xlsx_table` - Save tables to Excel files
- `XlsxPlugin` - Plugin for framework integration

For example:

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_xlsx_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "table.xlsx"}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;
// the column types will be automatically inferred
```

## Basic Usage

### Loading XLSX Files

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_xlsx_table};
use serde_json::json;

let options = LoadTableOptions::default();

// Load a simple XLSX file
let resource = serde_json::from_value(json!({"data": "data.xlsx"}))?;
let table = load_xlsx_table(&resource, &options)?;

// Load with custom format (specify sheet)
let resource = serde_json::from_value(json!({
    "data": "data.xlsx",
    "fileDialect": {"format": "xlsx", "sheetName": "Sheet2"},
}))?;
let table = load_xlsx_table(&resource, &options)?;

// Load multiple XLSX files (concatenated)
let resource = serde_json::from_value(json!({"data": ["file1.xlsx", "file2.xlsx"]}))?;
let table = load_xlsx_table(&resource, &options)?;
```

### Saving XLSX Files

```rust
use fairspec::fairspec_table::{SaveTableOptions, save_xlsx_table};
use serde_json::json;

// Save with default options
save_xlsx_table(table.clone(), &SaveTableOptions {
    path: "output.xlsx".into(),
    ..Default::default()
})?;

// Save with custom sheet name
save_xlsx_table(table, &SaveTableOptions {
    path: "output.xlsx".into(),
    file_dialect: Some(serde_json::from_value(json!({
        "format": "xlsx",
        "sheetName": "Data",
    }))?),
    overwrite: Some(true),
    ..Default::default()
})?;
```

## Advanced Features

### Sheet Selection

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_xlsx_table};
use serde_json::json;

// Select by sheet number (1-indexed)
let resource = serde_json::from_value(json!({
    "data": "workbook.xlsx",
    "fileDialect": {"format": "xlsx", "sheetNumber": 2},
}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;

// Select by sheet name
let resource = serde_json::from_value(json!({
    "data": "workbook.xlsx",
    "fileDialect": {"format": "xlsx", "sheetName": "Sales Data"},
}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;
```

### Multi-Header Row Processing

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_xlsx_table};
use serde_json::json;

// XLSX with multiple header rows
let resource = serde_json::from_value(json!({
    "data": "multi-header.xlsx",
    "fileDialect": {"format": "xlsx", "headerRows": [1, 2], "headerJoin": "_"},
}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;
// Resulting columns: ["Year_Quarter", "2023_Q1", "2023_Q2", "2024_Q1", "2024_Q2"]
```

### Comment Row Handling

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_xlsx_table};
use serde_json::json;

// Skip specific comment rows
let resource = serde_json::from_value(json!({
    "data": "with-comments.xlsx",
    "fileDialect": {"format": "xlsx", "commentRows": [1, 2], "headerRows": [3]},
}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;

// Skip rows with comment prefix
let resource = serde_json::from_value(json!({
    "data": "data.xlsx",
    "fileDialect": {"format": "xlsx", "commentPrefix": "#", "headerRows": [1]},
}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;
```

### Remote File Loading

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_xlsx_table};
use serde_json::json;

// Load from URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.xlsx"}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;

// Load multiple remote files
let resource = serde_json::from_value(json!({
    "data": [
        "https://api.example.com/data-2023.xlsx",
        "https://api.example.com/data-2024.xlsx",
    ],
}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;
```

### Column Names

The n-th name names the n-th column. A sheet with more columns than names fails to
load; names past its last column add columns that are empty in every row.

```rust
use fairspec::fairspec_table::{LoadTableOptions, load_xlsx_table};
use serde_json::json;

// Name the three columns of the sheet
let resource = serde_json::from_value(json!({
    "data": "data.xlsx",
    "fileDialect": {"format": "xlsx", "columnNames": ["name", "age", "city"]},
}))?;
let table = load_xlsx_table(&resource, &LoadTableOptions::default())?;
```
