---
title: Working with Parquet in Python
label: Parquet
path: /python/library/parquet/
order: 17
---

Apache Parquet file handling for compressed columnar data.

## Installation

```bash
pip install fairspec
```

## Getting Started

The Parquet plugin provides:

- `load_parquet_table` - Load Parquet files into tables
- `save_parquet_table` - Save tables to Parquet files
- `ParquetPlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_parquet_table, Resource

table = load_parquet_table(Resource(data="table.parquet"))
# Efficient columnar format with compression
```

Loaders return a Polars `LazyFrame`: call `.collect()` to get a `DataFrame`. Savers take any `LazyFrame`, so pass `df.lazy()` to save a `DataFrame`.

## Basic Usage

### Loading Parquet Files

```python
from fairspec import load_parquet_table, Resource

# Load from local file
table = load_parquet_table(Resource(data="data.parquet"))

# Load from remote URL
table = load_parquet_table(Resource(data="https://example.com/data.parquet"))

# Load multiple files (concatenated)
table = load_parquet_table(Resource(data=["file1.parquet", "file2.parquet"]))
```

### Saving Parquet Files

```python
from fairspec import save_parquet_table

# Save with default options
save_parquet_table(table, path="output.parquet")
```

## Advanced Features

### Remote File Loading

```python
from fairspec import load_parquet_table, Resource

# Load from URL
table = load_parquet_table(Resource(data="https://example.com/data.parquet"))

# Load multiple remote files
table = load_parquet_table(Resource(data=[
    "https://api.example.com/data-2023.parquet",
    "https://api.example.com/data-2024.parquet",
]))
```
