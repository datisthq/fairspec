---
title: Working with ODS in TypeScript
label: ODS
path: /typescript/library/ods/
order: 10
---

OpenDocument Spreadsheet (ODS) file handling with sheet selection and advanced header processing.

## Installation

```bash
npm install fairspec
```

## Getting Started

ODS format is handled by the XLSX plugin, which provides:

- `loadXlsxTable` - Load ODS files into tables
- `saveXlsxTable` - Save tables to ODS files
- `XlsxPlugin` - Plugin for framework integration

For example:

```typescript
import { loadXlsxTable } from "fairspec"

const table = await loadXlsxTable({ data: "table.ods" })
// the column types will be automatically inferred
```

Loaders return a nodejs-polars `LazyDataFrame`: call `.collect()` to get a `DataFrame`. Savers take any `LazyDataFrame`, so pass `df.lazy()` to save a `DataFrame`.

## Basic Usage

### Loading ODS Files

```typescript
import { loadXlsxTable } from "fairspec"

// Load a simple ODS file
const local = await loadXlsxTable({ data: "data.ods" })

// Load with custom format (specify sheet)
const fromSheet = await loadXlsxTable({
  data: "data.ods",
  fileDialect: {
    format: "ods",
    sheetName: "Sheet2",
  },
})

// Load multiple ODS files (concatenated)
const combined = await loadXlsxTable({
  data: ["part1.ods", "part2.ods", "part3.ods"],
})
```

### Saving ODS Files

```typescript
import { saveXlsxTable } from "fairspec"

// Save with default options
await saveXlsxTable(table, {
  path: "output.ods",
  fileDialect: { format: "ods" },
})

// Save with custom sheet name
await saveXlsxTable(table, {
  path: "output.ods",
  fileDialect: {
    format: "ods",
    sheetName: "Data",
  },
})
```

## Advanced Features

### Sheet Selection

```typescript
import { loadXlsxTable } from "fairspec"

// Select by sheet number (1-indexed)
const bySheetNumber = await loadXlsxTable({
  data: "workbook.ods",
  fileDialect: {
    format: "ods",
    sheetNumber: 2, // Load second sheet
  },
})

// Select by sheet name
const bySheetName = await loadXlsxTable({
  data: "workbook.ods",
  fileDialect: {
    format: "ods",
    sheetName: "Sales Data",
  },
})
```

### Multi-Header Row Processing

```typescript
import { loadXlsxTable } from "fairspec"

// ODS with multiple header rows
const table = await loadXlsxTable({
  data: "multi-header.ods",
  fileDialect: {
    format: "ods",
    headerRows: [1, 2],
    headerJoin: "_",
  },
})
// Resulting columns: ["Year_Quarter", "2023_Q1", "2023_Q2", "2024_Q1", "2024_Q2"]
```

### Comment Row Handling

```typescript
import { loadXlsxTable } from "fairspec"

// Skip specific comment rows
const byCommentRows = await loadXlsxTable({
  data: "with-comments.ods",
  fileDialect: {
    format: "ods",
    commentRows: [1, 2],
    headerRows: [3],
  },
})

// Skip rows with comment prefix
const byCommentPrefix = await loadXlsxTable({
  data: "data.ods",
  fileDialect: {
    format: "ods",
    commentPrefix: "#",
    headerRows: [1],
  },
})
```

### Remote File Loading

```typescript
import { loadXlsxTable } from "fairspec"

// Load from URL
const remote = await loadXlsxTable({
  data: "https://example.com/data.ods",
})

// Load multiple remote files
const remoteParts = await loadXlsxTable({
  data: [
    "https://api.example.com/data-2023.ods",
    "https://api.example.com/data-2024.ods",
  ],
})
```

### Column Names

The n-th name names the n-th column. A sheet with more columns than names fails to
load; names past its last column add columns that are empty in every row.

```typescript
import { loadXlsxTable } from "fairspec"

// Name the three columns of the sheet
const table = await loadXlsxTable({
  data: "data.ods",
  fileDialect: {
    format: "ods",
    columnNames: ["name", "age", "city"],
  },
})
```
