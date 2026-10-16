---
title: Working with ODS in Python
label: ODS
path: /python/ods/
order: 15
---

OpenDocument Spreadsheet (ODS) file handling with sheet selection, advanced header processing, and high-performance data operations.

## Installation

```bash
pip install fairspec
```

## Getting Started

ODS format is handled by the XLSX plugin, which provides:

- `load_xlsx_table` - Load ODS files into tables
- `save_xlsx_table` - Save tables to ODS files
- `XlsxPlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_xlsx_table, Resource

table = load_xlsx_table(Resource(data="table.ods"))
# the column types will be automatically inferred
```

## Basic Usage

### Loading ODS Files

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# Load a simple ODS file
table = load_xlsx_table(Resource(data="data.ods"))

# Load with custom format (specify sheet)
table = load_xlsx_table(Resource(
    data="data.ods",
    fileDialect=XlsxFileDialect(format="ods", sheetName="Sheet2"),
))

# Load multiple ODS files (concatenated)
table = load_xlsx_table(Resource(data=["part1.ods", "part2.ods", "part3.ods"]))
```

### Saving ODS Files

```python
from fairspec import save_xlsx_table
from fairspec_metadata import XlsxFileDialect

# Save with default options
save_xlsx_table(table, path="output.ods", fileDialect=XlsxFileDialect(format="ods"))

# Save with custom sheet name
save_xlsx_table(table, path="output.ods", fileDialect=XlsxFileDialect(
    format="ods",
    sheetName="Data",
))
```

## Advanced Features

### Sheet Selection

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# Select by sheet number (1-indexed)
table = load_xlsx_table(Resource(
    data="workbook.ods",
    fileDialect=XlsxFileDialect(format="ods", sheetNumber=2),
))

# Select by sheet name
table = load_xlsx_table(Resource(
    data="workbook.ods",
    fileDialect=XlsxFileDialect(format="ods", sheetName="Sales Data"),
))
```

### Multi-Header Row Processing

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# ODS with multiple header rows
table = load_xlsx_table(Resource(
    data="multi-header.ods",
    fileDialect=XlsxFileDialect(
        format="ods",
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
    data="with-comments.ods",
    fileDialect=XlsxFileDialect(
        format="ods",
        commentRows=[1, 2],
        headerRows=[3],
    ),
))

# Skip rows with comment prefix
table = load_xlsx_table(Resource(
    data="data.ods",
    fileDialect=XlsxFileDialect(
        format="ods",
        commentPrefix="#",
        headerRows=[1],
    ),
))
```

### Remote File Loading

```python
from fairspec import load_xlsx_table, Resource

# Load from URL
table = load_xlsx_table(Resource(data="https://example.com/data.ods"))

# Load multiple remote files
table = load_xlsx_table(Resource(data=[
    "https://api.example.com/data-2023.ods",
    "https://api.example.com/data-2024.ods",
]))
```

### Column Selection

```python
from fairspec import load_xlsx_table, Resource
from fairspec_metadata import XlsxFileDialect

# Select specific columns
table = load_xlsx_table(Resource(
    data="data.ods",
    fileDialect=XlsxFileDialect(format="ods", columnNames=["name", "age", "city"]),
))
```
