---
title: Working with XLSX in TypeScript
label: XLSX
path: /typescript/library/xlsx/
order: 9
---

Excel (.xlsx) file handling with sheet selection and advanced header processing.

## Installation

```bash
npm install fairspec
```

## Getting Started

The XLSX plugin provides:

- `loadXlsxTable` - Load Excel files into tables
- `saveXlsxTable` - Save tables to Excel files
- `XlsxPlugin` - Plugin for framework integration

For example:

```typescript
import { loadXlsxTable } from "fairspec"

const table = await loadXlsxTable({ data: "table.xlsx" })
// the column types will be automatically inferred
```

Loaders return a nodejs-polars `LazyDataFrame`: call `.collect()` to get a `DataFrame`. Savers take any `LazyDataFrame`, so pass `df.lazy()` to save a `DataFrame`.

## Basic Usage

### Loading XLSX Files

```typescript
import { loadXlsxTable } from "fairspec"

// Load a simple XLSX file
const local = await loadXlsxTable({ data: "data.xlsx" })

// Load with custom format (specify sheet)
const fromSheet = await loadXlsxTable({
  data: "data.xlsx",
  fileDialect: {
    format: "xlsx",
    sheetName: "Sheet2",
  },
})

// Load multiple XLSX files (concatenated)
const combined = await loadXlsxTable({
  data: ["part1.xlsx", "part2.xlsx", "part3.xlsx"],
})
```

### Saving XLSX Files

```typescript
import { saveXlsxTable } from "fairspec"

// Save with default options
await saveXlsxTable(table, { path: "output.xlsx" })

// Save with custom sheet name
await saveXlsxTable(table, {
  path: "output.xlsx",
  fileDialect: {
    format: "xlsx",
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
  data: "workbook.xlsx",
  fileDialect: {
    format: "xlsx",
    sheetNumber: 2, // Load second sheet
  },
})

// Select by sheet name
const bySheetName = await loadXlsxTable({
  data: "workbook.xlsx",
  fileDialect: {
    format: "xlsx",
    sheetName: "Sales Data",
  },
})
```

### Multi-Header Row Processing

```typescript
import { loadXlsxTable } from "fairspec"

// XLSX with multiple header rows
const table = await loadXlsxTable({
  data: "multi-header.xlsx",
  fileDialect: {
    format: "xlsx",
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
  data: "with-comments.xlsx",
  fileDialect: {
    format: "xlsx",
    commentRows: [1, 2],
    headerRows: [3],
  },
})

// Skip rows with comment prefix
const byCommentPrefix = await loadXlsxTable({
  data: "data.xlsx",
  fileDialect: {
    format: "xlsx",
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
  data: "https://example.com/data.xlsx",
})

// Load multiple remote files
const remoteParts = await loadXlsxTable({
  data: [
    "https://api.example.com/data-2023.xlsx",
    "https://api.example.com/data-2024.xlsx",
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
  data: "data.xlsx",
  fileDialect: {
    format: "xlsx",
    columnNames: ["name", "age", "city"],
  },
})
```
