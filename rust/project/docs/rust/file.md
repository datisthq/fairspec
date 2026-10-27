---
title: Working with Files in Rust
label: File
path: /rust/library/file/
order: 4
---

File operations for copying, describing, validating, and analyzing local or remote files — from Rust.

## Available Functions

The file API provides utilities for working with raw files (independent of their tabular or JSON content). Apart from `infer_file_dialect`, which lives in `fairspec`, they come from `fairspec::fairspec_dataset`:

- `copy_file` - Copy a file from one path to another (local or remote source)
- `describe_file` - Get a file's size, textual flag, and integrity hash
- `preview_file` - Read the first lines of a file as raw text, as a `FilePreview` (`text`, `truncated`)
- `validate_file` - Validate a file against the metadata declared on a resource
- `infer_file_dialect` - Detect a file's dialect (CSV, JSON, Parquet, …)
- `infer_textual`, `infer_integrity`, `infer_hash`, `infer_bytes` - Lower-level inference primitives
- `prefetch_file`, `prefetch_files` - Download remote files to a local cache
- `load_file`, `save_file`, `load_file_stream`, `save_file_stream` - Raw byte I/O

## Copying Files

Copy a file from a local or remote source to a local destination:

```rust
use fairspec::fairspec_dataset::copy_file;

// Copy a local file
copy_file("data.csv", "output.csv", None)?;

// Copy a remote file
copy_file("https://example.com/data.csv", "local-data.csv", None)?;

// Bounded copy (downloads at most max_bytes)
copy_file("https://example.com/large.csv", "preview.csv", Some(10_000))?;
```

Parameters, in order:

- `source_path` — source path or URL
- `target_path` — local destination path
- `max_bytes` — cap the number of bytes copied

## Describing Files

Get a file's size, textual flag, and integrity hash:

```rust
use fairspec::fairspec_dataset::describe_file;
use fairspec::fairspec_metadata::IntegrityType;

let description = describe_file("data.csv", None)?;
// FileDescription { bytes: 1024, textual: true, integrity: Some(Integrity { type: Sha256, hash: "a1b2c3..." }) }

// Specify a different hash algorithm
let description = describe_file("data.bin", Some(IntegrityType::Md5))?;
```

The returned `FileDescription` has:

- `bytes: u64` — file size in bytes
- `textual: bool` — whether the file is text-based (detected by sampling the first 10 KiB)
- `integrity: Option<Integrity>` — hash value and algorithm

Supported hash types: `Md5`, `Sha1`, `Sha256` (default), `Sha512`.

## Validating Files

Validate a file against the metadata declared on a `Resource` — checks the textual flag and integrity hash:

```rust
use fairspec::fairspec_dataset::validate_file;
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": "data.csv",
    "integrity": {"type": "sha256", "hash": "a1b2c3d4e5f6..."},
}))?;

let report = validate_file(&resource, None)?;

if !report.valid {
    for error in &report.errors {
        println!("{}", serde_json::to_string(error)?);
    }
}
```

The returned `Report` carries integrity and textual errors. Example error, serialised:

```json
{
  "type": "file/integrity",
  "hashType": "sha256",
  "expectedHash": "a1b2c3d4e5f6...",
  "actualHash": "different..."
}
```

## Inferring a File Dialect

Detect the dialect (CSV delimiter, JSON pointer, Excel sheet, etc.) of a file without loading the table:

```rust
use fairspec::fairspec_dataset::InferFileDialectOptions;
use fairspec::infer_file_dialect;
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let dialect = infer_file_dialect(&resource, &InferFileDialectOptions::default())?;
// Some(Csv(CsvFileDialect { delimiter: Some(","), .. }))

// Sample more bytes for trickier files
let resource = serde_json::from_value(json!({"data": "data.txt"}))?;
let dialect = infer_file_dialect(&resource, &InferFileDialectOptions {
    sample_bytes: Some(50_000),
    ..Default::default()
})?;
```

Detected formats:

- `csv`, `tsv` - delimited text
- `json`, `jsonl` - JSON and JSON Lines
- `xlsx`, `ods` - spreadsheets
- `parquet`, `arrow` - columnar binary formats
- `sqlite` - SQLite databases

## Inference Primitives

For more fine-grained control, the inference primitives are exposed individually:

```rust
use fairspec::fairspec_dataset::{infer_bytes, infer_hash, infer_integrity, infer_textual};
use fairspec::fairspec_metadata::IntegrityType;
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "data.csv"}))?;

let size = infer_bytes(&resource, None)?; // 1024
let is_text = infer_textual(&resource, None)?; // true
let hash = infer_hash(&resource, None, None)?; // "a1b2c3..."
let integrity = infer_integrity(&resource, None, None)?; // Some(Integrity { type: Sha256, .. })

// Use a different hash algorithm
let integrity = infer_integrity(&resource, Some(IntegrityType::Md5), None)?;
```

## Working with File Streams

Read or write raw bytes directly:

```rust
use fairspec::fairspec_dataset::{load_file, save_file};

// Read all bytes (with optional cap)
let data = load_file("data.bin", None)?;
let preview = load_file("data.bin", Some(1024))?;

// Write bytes (won't overwrite unless asked)
save_file("output.bin", &data, false)?;
save_file("output.bin", &data, true)?;
```

For larger files, use the stream APIs:

```rust
use std::io::Read;

use fairspec::fairspec_dataset::{load_file_stream, save_file_stream};

// Read a binary stream
let mut stream = load_file_stream("large.bin", None, None)?;
let mut chunk = [0; 4096];
let read = stream.read(&mut chunk)?;

// Write from a stream
let source = std::fs::File::open("source.bin")?;
save_file_stream(source, "dest.bin", false)?;
```

`FileStream` is a `Box<dyn Read + Send>`; the library is synchronous, with no async runtime to set up.

## Working with Remote Files

`prefetch_file` downloads a remote file to a local temp path and returns that path. This is useful when an operation requires a local file but the source is remote:

```rust
use fairspec::fairspec_dataset::{prefetch_file, prefetch_files};
use serde_json::json;

let local_path = prefetch_file("https://example.com/data.csv", None)?;
// Now you have a local path you can pass to other tools

// Batch prefetch every path of a resource
let resource = serde_json::from_value(json!({
    "data": ["https://example.com/a.csv", "https://example.com/b.csv"],
}))?;
let local_paths = prefetch_files(&resource, None, None)?;
```

Temp files are deleted when the process exits. Note that `load_table`, `load_data`, `copy_file`, and other high-level functions already handle remote URLs transparently — only reach for `prefetch_file` when you need a local path for an external tool.

## Common Workflows

### Copy and Verify

```rust
use fairspec::fairspec_dataset::{copy_file, describe_file, validate_file};
use serde_json::json;

// Copy a remote file
copy_file("https://example.com/data.csv", "local-data.csv", None)?;

// Capture its hash
let description = describe_file("local-data.csv", None)?;

// Validate later that the file hasn't changed
let resource = serde_json::from_value(json!({
    "data": "local-data.csv",
    "integrity": description.integrity,
}))?;
let report = validate_file(&resource, None)?;
assert!(report.valid);
```

### Batch Hash All Files in a Folder

```rust
use fairspec::fairspec_dataset::describe_file;

for entry in std::fs::read_dir("data")? {
    let path = entry?.path();
    if path.extension().is_none_or(|extension| extension != "csv") {
        continue;
    }
    let description = describe_file(&path.to_string_lossy(), None)?;
    let hash = description.integrity.map(|integrity| integrity.hash).unwrap_or_default();
    println!("{}: {hash}  ({} bytes)", path.display(), description.bytes);
}
```

### Process Dataset Resources

```rust
use fairspec::fairspec_dataset::{InferFileDialectOptions, describe_file};
use fairspec::fairspec_metadata::Dataset;
use fairspec::{infer_file_dialect, load_dataset};

let descriptor = load_dataset("dataset.json")?.ok_or("unsupported source")?;
let dataset: Dataset = serde_json::from_value(descriptor.into())?;

for resource in dataset.resources.iter().flatten() {
    let Some(path) = resource.data.as_ref().and_then(|data| data.as_str()) else {
        continue;
    };
    let description = describe_file(path, None)?;
    let dialect = infer_file_dialect(resource, &InferFileDialectOptions::default())?;
    println!("{:?}: {} bytes, dialect={dialect:?}", resource.name, description.bytes);
}
```

## Examples

### Pin Resource Integrity

```rust
use fairspec::fairspec_dataset::{SaveDatasetOptions, describe_file};
use fairspec::fairspec_metadata::Dataset;
use fairspec::{load_dataset, save_dataset};

let descriptor = load_dataset("dataset.json")?.ok_or("unsupported source")?;
let mut dataset: Dataset = serde_json::from_value(descriptor.into())?;

for resource in dataset.resources.iter_mut().flatten() {
    let Some(path) = resource.data.as_ref().and_then(|data| data.as_str()) else {
        continue;
    };
    resource.integrity = describe_file(path, None)?.integrity;
}

save_dataset(&dataset, &SaveDatasetOptions {
    target: "./dataset-pinned".into(),
    with_remote: false,
})?;
```

### Cache a Remote File Once

```rust
use fairspec::fairspec_dataset::{describe_file, prefetch_file, validate_file};
use serde_json::json;

let local_path = prefetch_file("https://example.com/large-dataset.csv", None)?;
let description = describe_file(&local_path, None)?;

// Re-use the cached file for further work
let resource = serde_json::from_value(json!({
    "data": local_path,
    "integrity": description.integrity,
}))?;
assert!(validate_file(&resource, None)?.valid);
```

### Automation with Hashes

```rust
use fairspec::fairspec_dataset::describe_file;
use fairspec::fairspec_metadata::IntegrityType;

let description = describe_file("data.csv", Some(IntegrityType::Sha256))?;
println!("{:?}", description.integrity.map(|integrity| integrity.hash));
```
