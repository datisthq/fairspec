---
title: Working with Parquet in TypeScript
label: Parquet
path: /typescript/library/parquet/
order: 12
---

Apache Parquet file handling for compressed columnar data.

## Installation

```bash
npm install fairspec
```

## Getting Started

The Parquet plugin provides:

- `loadParquetTable` - Load Parquet files into tables
- `saveParquetTable` - Save tables to Parquet files
- `ParquetPlugin` - Plugin for framework integration

For example:

```typescript
import { loadParquetTable } from "fairspec"

const table = await loadParquetTable({ data: "table.parquet" })
// Efficient columnar format with compression
```

Loaders return a nodejs-polars `LazyDataFrame`: call `.collect()` to get a `DataFrame`. Savers take any `LazyDataFrame`, so pass `df.lazy()` to save a `DataFrame`.

## Basic Usage

### Loading Parquet Files

```typescript
import { loadParquetTable } from "fairspec"

// Load from local file
const table = await loadParquetTable({ data: "data.parquet" })

// Load from remote URL
const table = await loadParquetTable({
  data: "https://example.com/data.parquet",
})

// Load multiple files (concatenated)
const table = await loadParquetTable({
  data: ["file1.parquet", "file2.parquet"],
})
```

### Saving Parquet Files

```typescript
import { saveParquetTable } from "fairspec"

// Save with default options
await saveParquetTable(table, { path: "output.parquet" })

// Save with explicit format
await saveParquetTable(table, {
  path: "output.parquet",
  fileDialect: { format: "parquet" },
})
```

## Advanced Features

### Remote File Loading

```typescript
import { loadParquetTable } from "fairspec"

// Load from URL
const table = await loadParquetTable({
  data: "https://example.com/data.parquet",
})

// Load multiple remote files
const table = await loadParquetTable({
  data: [
    "https://api.example.com/data-2023.parquet",
    "https://api.example.com/data-2024.parquet",
  ],
})
```
