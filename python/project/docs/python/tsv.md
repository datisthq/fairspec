---
title: Working with TSV in Python
label: TSV
path: /python/tsv/
order: 11
---
Tab-separated values (TSV) file handling with automatic format detection and high-performance data operations.

## Installation

```bash
pip install fairspec
```

## Getting Started

The TSV format is handled by the CSV plugin, which provides:

- `load_csv_table` - Load TSV files into tables
- `save_csv_table` - Save tables to TSV files
- `CsvPlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_csv_table, Resource

table = load_csv_table(Resource(data="table.tsv"))
# the column types will be automatically inferred
```

## Basic Usage

### Loading TSV Files

```python
from fairspec import load_csv_table, Resource
from fairspec_metadata import CsvFileDialect

# Load a simple TSV file
table = load_csv_table(Resource(data="data.tsv"))

# Load with explicit format
table = load_csv_table(Resource(
    data="data.tsv",
    fileDialect=CsvFileDialect(delimiter="\t", headerRows=[1]),
))

# Load multiple TSV files (concatenated)
table = load_csv_table(Resource(data=["part1.tsv", "part2.tsv", "part3.tsv"]))
```

### Saving TSV Files

```python
from fairspec import save_csv_table
from fairspec_metadata import CsvFileDialect

# Save with default options
save_csv_table(table, path="output.tsv", fileDialect=CsvFileDialect(delimiter="\t"))

# Save with line terminator option
save_csv_table(table, path="output.tsv", fileDialect=CsvFileDialect(
    delimiter="\t",
    lineTerminator="\r\n",
))
```

## Advanced Features

### Multi-Header Row Processing

```python
from fairspec import load_csv_table, Resource
from fairspec_metadata import CsvFileDialect

# TSV with multiple header rows
table = load_csv_table(Resource(
    data="multi-header.tsv",
    fileDialect=CsvFileDialect(
        delimiter="\t",
        headerRows=[1, 2],
        headerJoin="_",
    ),
))
```

### Comment Handling

```python
from fairspec import load_csv_table, Resource
from fairspec_metadata import CsvFileDialect

# TSV with comment lines
table = load_csv_table(Resource(
    data="with-comments.tsv",
    fileDialect=CsvFileDialect(
        delimiter="\t",
        commentPrefix="#",
        headerRows=[1],
    ),
))

# Or specify specific comment row numbers
table = load_csv_table(Resource(
    data="with-comments.tsv",
    fileDialect=CsvFileDialect(
        delimiter="\t",
        commentRows=[1, 2],
        headerRows=[3],
    ),
))
```

### Remote File Loading

```python
from fairspec import load_csv_table, Resource

# Load from URL
table = load_csv_table(Resource(data="https://example.com/data.tsv"))

# Load multiple remote files
table = load_csv_table(Resource(data=[
    "https://api.example.com/data-2023.tsv",
    "https://api.example.com/data-2024.tsv",
]))
```
