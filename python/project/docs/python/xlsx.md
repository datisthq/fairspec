---
title: Working with XLSX in Python
label: XLSX
path: /python/xlsx/
order: 14
---

Excel (.xlsx) file handling with sheet selection, advanced header processing, and high-performance data operations.

## Installation

```bash
pip install fairspec
```

## Getting Started

The XLSX plugin provides:

- `load_xlsx_table` - Load Excel files into tables
- `save_xlsx_table` - Save tables to Excel files
- `XlsxPlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_xlsx_table, Resource

table = load_xlsx_table(Resource(data="table.xlsx"))
# the column types will be automatically inferred
```

## Basic Usage

### Loading XLSX Files

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# Load a simple XLSX file
table = load_xlsx_table(Resource(data="data.xlsx"))

# Load with custom format (specify sheet)
table = load_xlsx_table(Resource(
    data="data.xlsx",
    fileDialect=XlsxFileDialect(sheetName="Sheet2"),
))

# Load multiple XLSX files (concatenated)
table = load_xlsx_table(Resource(data=["part1.xlsx", "part2.xlsx", "part3.xlsx"]))
```

### Saving XLSX Files

```python
from fairspec import save_xlsx_table
from fairspec_metadata import XlsxFileDialect

# Save with default options
save_xlsx_table(table, path="output.xlsx")

# Save with custom sheet name
save_xlsx_table(table, path="output.xlsx", fileDialect=XlsxFileDialect(sheetName="Data"))
```

## Advanced Features

### Sheet Selection

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# Select by sheet number (1-indexed)
table = load_xlsx_table(Resource(
    data="workbook.xlsx",
    fileDialect=XlsxFileDialect(sheetNumber=2),
))

# Select by sheet name
table = load_xlsx_table(Resource(
    data="workbook.xlsx",
    fileDialect=XlsxFileDialect(sheetName="Sales Data"),
))
```

### Multi-Header Row Processing

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# XLSX with multiple header rows
table = load_xlsx_table(Resource(
    data="multi-header.xlsx",
    fileDialect=XlsxFileDialect(
        headerRows=[1, 2],
        headerJoin="_",
    ),
))
# Resulting columns: ["Year_Quarter", "2023_Q1", "2023_Q2", "2024_Q1", "2024_Q2"]
```

### Comment Row Handling

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# Skip specific comment rows
table = load_xlsx_table(Resource(
    data="with-comments.xlsx",
    fileDialect=XlsxFileDialect(
        commentRows=[1, 2],
        headerRows=[3],
    ),
))

# Skip rows with comment prefix
table = load_xlsx_table(Resource(
    data="data.xlsx",
    fileDialect=XlsxFileDialect(
        commentPrefix="#",
        headerRows=[1],
    ),
))
```

### Remote File Loading

```python
from fairspec import load_xlsx_table, Resource

# Load from URL
table = load_xlsx_table(Resource(data="https://example.com/data.xlsx"))

# Load multiple remote files
table = load_xlsx_table(Resource(data=[
    "https://api.example.com/data-2023.xlsx",
    "https://api.example.com/data-2024.xlsx",
]))
```

### Column Selection

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# Select specific columns
table = load_xlsx_table(Resource(
    data="data.xlsx",
    fileDialect=XlsxFileDialect(columnNames=["name", "age", "city"]),
))
```
