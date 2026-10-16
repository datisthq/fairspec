---
title: Working with CSV in Python
label: CSV
path: /python/csv/
order: 10
---
Comprehensive CSV file handling with automatic format detection, advanced header processing, and high-performance data operations.

## Installation

```bash
pip install fairspec
```

## Getting Started

The CSV plugin provides these capabilities:

- `load_csv_table` - Load CSV/TSV files into tables
- `save_csv_table` - Save tables to CSV/TSV files
- `CsvPlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_csv_table, Resource

table = load_csv_table(Resource(data="table.csv"))
# the column types will be automatically inferred
# or you can provide a Table Schema
```

## Basic Usage

### Loading CSV Files

```python
from fairspec import load_csv_table, Resource
from fairspec_metadata import CsvFileDialect

# Load a simple CSV file
table = load_csv_table(Resource(data="data.csv"))

# Load with custom format
table = load_csv_table(Resource(
    data="data.csv",
    fileDialect=CsvFileDialect(
        delimiter=";",
        headerRows=[1],
    ),
))

# Load multiple CSV files (concatenated)
table = load_csv_table(Resource(data=["part1.csv", "part2.csv", "part3.csv"]))
```

### Saving CSV Files

```python
from fairspec import save_csv_table
from fairspec_metadata import CsvFileDialect

# Save with default options
save_csv_table(table, path="output.csv")

# Save with custom format
save_csv_table(table, path="output.csv", fileDialect=CsvFileDialect(
    delimiter="\t",
    quoteChar="'",
))

# Save as TSV
save_csv_table(table, path="output.tsv", fileDialect=CsvFileDialect(delimiter="\t"))
```

### Automatic Format Detection

```python
from fairspec import load_csv_table, Resource
from fairspec_metadata import CsvFileDialect

# Format is automatically detected when not specified
table = load_csv_table(Resource(data="unknown-dialect.csv"))
# The CSV plugin will automatically infer delimiter, quote characters, etc.

# You can also explicitly specify the format if detection isn't accurate
table = load_csv_table(Resource(
    data="data.csv",
    fileDialect=CsvFileDialect(
        delimiter=",",
        quoteChar='"',
        headerRows=[1],
    ),
))
```

## Advanced Features

### Multi-Header Row Processing

```python
# CSV with multiple header rows:
# Year,2023,2023,2024,2024
# Quarter,Q1,Q2,Q1,Q2
# Revenue,100,120,110,130

from fairspec import load_csv_table, Resource
from fairspec_metadata import CsvFileDialect

table = load_csv_table(Resource(
    data="multi-header.csv",
    fileDialect=CsvFileDialect(
        headerRows=[1, 2],
        headerJoin="_",
    ),
))
# Resulting columns: ["Year_Quarter", "2023_Q1", "2023_Q2", "2024_Q1", "2024_Q2"]
```

### Comment Row Handling

```python
# CSV with comment rows:
# # This is a comment
# # Generated on 2024-01-01
# Name,Age,City
# John,25,NYC

from fairspec import load_csv_table, Resource
from fairspec_metadata import CsvFileDialect

table = load_csv_table(Resource(
    data="with-comments.csv",
    fileDialect=CsvFileDialect(
        commentRows=[1, 2],
        headerRows=[3],
    ),
))

# Or use commentPrefix to skip lines starting with a specific character
table = load_csv_table(Resource(
    data="with-comments.csv",
    fileDialect=CsvFileDialect(
        commentPrefix="#",
        headerRows=[1],
    ),
))
```

### Remote File Loading

```python
from fairspec import load_csv_table, Resource

# Load from URL
table = load_csv_table(Resource(data="https://example.com/data.csv"))

# Load multiple remote files
table = load_csv_table(Resource(data=[
    "https://api.example.com/data-2023.csv",
    "https://api.example.com/data-2024.csv",
]))
```
