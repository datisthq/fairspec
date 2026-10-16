---
title: Working with Files in Python
label: File
path: /python/file/
order: 4
---
File operations for copying, describing, validating, and analyzing local or remote files — from Python.

## Available Functions

The file API provides utilities for working with raw files (independent of their tabular or JSON content):

- `copy_file` - Copy a file from one path to another (local or remote source)
- `describe_file` - Get a file's size, textual flag, and integrity hash
- `validate_file` - Validate a file against the metadata declared on a resource
- `infer_file_dialect` - Detect a file's dialect (CSV, JSON, Parquet, …)
- `infer_textual`, `infer_integrity`, `infer_hash`, `infer_bytes` - Lower-level inference primitives
- `prefetch_file`, `prefetch_files` - Download remote files to a local cache
- `load_file`, `save_file`, `load_file_stream`, `save_file_stream` - Raw byte I/O

## Copying Files

Copy a file from a local or remote source to a local destination:

```python
from fairspec import copy_file

# Copy a local file
copy_file(source_path="data.csv", target_path="output.csv")

# Copy a remote file
copy_file(
    source_path="https://example.com/data.csv",
    target_path="local-data.csv",
)

# Bounded copy (downloads at most max_bytes)
copy_file(source_path="https://example.com/large.csv", target_path="preview.csv", max_bytes=10_000)
```

Parameters:

- `source_path` (required, keyword-only) — source path or URL
- `target_path` (required, keyword-only) — local destination path
- `max_bytes` (optional, keyword-only) — cap the number of bytes copied

## Describing Files

Get a file's size, textual flag, and integrity hash:

```python
from fairspec import describe_file

description = describe_file("data.csv")
# FileDescription(bytes=1024, textual=True, integrity=Integrity(type="sha256", hash="a1b2c3..."))

# Specify a different hash algorithm
description = describe_file("data.bin", hash_type="md5")
```

The returned `FileDescription` has:

- `bytes: int | None` — file size in bytes
- `textual: bool | None` — whether the file is text-based (detected by sampling the first 10 KiB)
- `integrity: Integrity | None` — hash value and algorithm

Supported hash types: `md5`, `sha1`, `sha256` (default), `sha512`.

## Validating Files

Validate a file against the metadata declared on a `Resource` — checks the textual flag and integrity hash:

```python
from fairspec import Integrity, Resource, validate_file

resource = Resource(
    data="data.csv",
    integrity=Integrity(type="sha256", hash="a1b2c3d4e5f6..."),
)

report = validate_file(resource)

if not report.valid:
    for error in report.errors:
        print(f"[{error.type}] {error.message}")
```

The returned `Report` carries integrity and textual errors. Example error:

```python
# IntegrityError(
#   type='file/integrity',
#   hashType='sha256',
#   expectedHash='a1b2c3d4e5f6...',
#   actualHash='different...',
# )
```

## Inferring a File Dialect

Detect the dialect (CSV delimiter, JSON pointer, Excel sheet, etc.) of a file without loading the table:

```python
from fairspec import Resource, infer_file_dialect

dialect = infer_file_dialect(Resource(data="data.csv"))
# CsvFileDialect(format='csv', delimiter=',', quoteChar='"')

# Sample more bytes for trickier files
dialect = infer_file_dialect(Resource(data="data.txt"), sample_bytes=50_000)
```

Detected formats:

- `csv`, `tsv` - delimited text
- `json`, `jsonl` - JSON and JSON Lines
- `xlsx`, `ods` - spreadsheets
- `parquet`, `arrow` - columnar binary formats
- `sqlite` - SQLite databases

## Inference Primitives

For more fine-grained control, the inference primitives are exposed individually:

```python
from fairspec import Resource, infer_bytes, infer_hash, infer_integrity, infer_textual

resource = Resource(data="data.csv")

size = infer_bytes(resource)              # 1024
is_text = infer_textual(resource)         # True
hash_value = infer_hash(resource)         # 'a1b2c3...'
integrity = infer_integrity(resource)     # Integrity(type='sha256', hash='a1b2c3...')

# Use a different hash algorithm
integrity = infer_integrity(resource, hash_type="md5")
```

## Working with File Streams

Read or write raw bytes directly:

```python
from fairspec import load_file, save_file

# Read all bytes (with optional cap)
data = load_file("data.bin")
preview = load_file("data.bin", max_bytes=1024)

# Write bytes (won't overwrite by default)
save_file("output.bin", data)
save_file("output.bin", data, overwrite=True)
```

For larger files, use the stream APIs:

```python
from fairspec import load_file_stream, save_file_stream

# Read a binary stream
with load_file_stream("large.bin") as stream:
    chunk = stream.read(4096)

# Write from a stream
with open("source.bin", "rb") as stream:
    save_file_stream(stream, path="dest.bin")
```

## Working with Remote Files

`prefetch_file` downloads a remote file to a local temp path and returns that path. This is useful when an operation requires a local file but the source is remote:

```python
from fairspec import prefetch_file, prefetch_files

local_path = prefetch_file("https://example.com/data.csv")
# Now you have a local path you can pass to other tools

# Batch prefetch
local_paths = prefetch_files([
    "https://example.com/a.csv",
    "https://example.com/b.csv",
])
```

Note that `load_table`, `load_data`, `copy_file`, and other high-level functions already handle remote URLs transparently — only reach for `prefetch_file` when you need a local path for an external tool.

## Common Workflows

### Copy and Verify

```python
from fairspec import Integrity, Resource, copy_file, describe_file, validate_file

# Copy a remote file
copy_file(source_path="https://example.com/data.csv", target_path="local-data.csv")

# Capture its hash
description = describe_file("local-data.csv")

# Validate later that the file hasn't changed
resource = Resource(data="local-data.csv", integrity=description.integrity)
report = validate_file(resource)
assert report.valid
```

### Batch Hash All Files in a Folder

```python
from pathlib import Path
from fairspec import describe_file

for path in Path("data").glob("*.csv"):
    description = describe_file(str(path))
    print(f"{path.name}: {description.integrity.hash}  ({description.bytes} bytes)")
```

### Process Dataset Resources

```python
from fairspec import Dataset, copy_file, describe_file, infer_file_dialect, load_dataset

dataset = Dataset.model_validate(load_dataset("dataset.json"))

for resource in dataset.resources or []:
    description = describe_file(resource.data)
    dialect = infer_file_dialect(resource)
    print(f"{resource.name}: {description.bytes} bytes, dialect={dialect}")
```

## Examples

### Pin Resource Integrity

```python
from fairspec import Dataset, describe_file, load_dataset, save_dataset

dataset = Dataset.model_validate(load_dataset("dataset.json"))

for resource in dataset.resources or []:
    description = describe_file(resource.data)
    resource.integrity = description.integrity
    resource.bytes = description.bytes

save_dataset(dataset, target="./dataset-pinned")
```

### Cache a Remote File Once

```python
from fairspec import describe_file, prefetch_file, validate_file, Resource

local_path = prefetch_file("https://example.com/large-dataset.csv")
description = describe_file(local_path)

# Re-use the cached file for further work
resource = Resource(data=local_path, integrity=description.integrity)
assert validate_file(resource).valid
```

### Automation with Hashes

```python
from fairspec import describe_file

description = describe_file("data.csv", hash_type="sha256")
print(description.integrity.hash)
```
