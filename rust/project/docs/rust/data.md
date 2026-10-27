---
title: Working with JSON Data in Rust
label: Data
path: /rust/library/data/
order: 3
---

JSON data validation and schema operations using JSON Schema standards — from Rust.

## Available Functions

The data API provides utilities for working with JSON data (non-tabular):

- `load_data` - Load a JSON payload from a resource (inline or file)
- `validate_data` - Validate a JSON payload against a Data Schema
- `infer_data_schema` - Automatically infer a Data Schema from JSON data
- `validate_data_schema` - Validate the Data Schema itself
- `render_data_schema_as` - Render a Data Schema through the render plugins

A "Data Schema" is a JSON Schema (Draft 2020-12 compatible) describing arbitrary structured JSON — distinct from a Table Schema, which describes columnar tabular data. In Rust, `DataSchema` is a `serde_json::Map`.

`validate_data_schema` and `load_data_schema` live in `fairspec::fairspec_metadata`; the rest in `fairspec`.

## Loading Data

Load a JSON payload from a resource. The payload can be inline (a literal JSON value stored on the resource) or fetched from a local file or remote URL:

```rust
use fairspec::load_data;
use serde_json::json;

// Load from a local file
let resource = serde_json::from_value(json!({"data": "data.json"}))?;
let data = load_data(&resource)?;

// Load from a remote URL
let resource = serde_json::from_value(json!({"data": "https://example.com/data.json"}))?;
let data = load_data(&resource)?;

// Load inline data (descriptor already contains the payload)
let resource = serde_json::from_value(json!({
    "data": {"users": [{"id": 1, "name": "Alice"}]},
}))?;
let data = load_data(&resource)?;
```

`load_data` returns `Option<serde_json::Value>` — typically an object, array, or primitive, depending on the JSON content. The result is `None` if the source cannot be resolved.

## Validating Data

Validate a JSON payload against the `dataSchema` attached to the resource:

```rust
use fairspec::validate_data;
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": "user.json",
    "dataSchema": {
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "email": {"type": "string", "format": "email"},
            "age": {"type": "integer", "minimum": 0},
        },
        "required": ["name", "email"],
    },
}))?;

let report = validate_data(&resource)?;

if !report.valid {
    for error in &report.errors {
        println!("{}", serde_json::to_value(error)?["message"]);
    }
}
```

The returned `Report` has `valid: bool` and `errors: Vec<FairspecError>`. A data error carries a `message` and a `jsonPointer` to the failing value. Messages are jsonschema-rs wording, so they differ from the Python and TypeScript libraries.

The Data Schema is checked first: a `$schema` other than a Fairspec Data Schema profile or `https://json-schema.org/draft/2020-12/schema`, and a `$ref` or `$dynamicRef` that does not resolve within the schema, are `metadata` errors under `/dataSchema`, and no data is checked. A remote reference is never fetched. `format` is asserted for `date`, `time`, `date-time`, `duration`, `email`, `ipv4` and `uuid`; any other format is an annotation.

Example error, serialised:

```json
{
  "type": "data",
  "message": "-1 is less than the minimum of 0",
  "jsonPointer": "/users/0/age"
}
```

## Inferring a Data Schema

Automatically generate a JSON Schema from a JSON payload:

```rust
use fairspec::infer_data_schema;
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "users.json"}))?;
let schema = infer_data_schema(&resource)?;
```

The inference detects:

- Data types (`string`, `number`, `integer`, `boolean`, `null`)
- Object structures and nested properties
- Array items and their types
- Required properties based on presence
- Enum values for low-cardinality fields

The result is an `Option<JsonSchema>`, a JSON object usable directly as a `dataSchema`.

Example — given `users.json`:

```json
[
  { "id": 1, "name": "Alice", "email": "alice@example.com", "age": 30, "active": true },
  { "id": 2, "name": "Bob", "email": "bob@example.com", "age": 25, "active": false }
]
```

The inferred schema is:

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "id": { "type": "integer" },
      "name": { "type": "string" },
      "email": { "type": "string" },
      "age": { "type": "integer" },
      "active": { "type": "boolean" }
    },
    "required": ["id", "name", "email", "age", "active"]
  }
}
```

## Validating a Data Schema

Validate that a Data Schema itself is well-formed (valid JSON Schema):

```rust
use fairspec::fairspec_metadata::validate_data_schema;

let result = validate_data_schema("schema.json", None)?;

if result.valid {
    println!("Schema is valid");
} else {
    for error in &result.errors {
        println!("{}", serde_json::to_value(error)?["message"]);
    }
}
```

`validate_data_schema` accepts either a descriptor path/URL or a literal `Descriptor`. The check confirms the schema is:

- Valid JSON
- Compliant with JSON Schema Draft 2020-12
- Using valid keywords and formats

To validate only a sub-schema inside a larger descriptor, pass a root JSON pointer:

```rust
use fairspec::fairspec_metadata::validate_data_schema;

let result = validate_data_schema("dataset.json", Some("/resources/0/dataSchema"))?;
```

## Rendering a Data Schema

`render_data_schema_as` asks the render plugins for the requested format:

```rust
use fairspec::fairspec_metadata::{RenderDataSchemaOptions, load_data_schema};
use fairspec::render_data_schema_as;

let schema = load_data_schema("schema.json")?;
let markdown = render_data_schema_as(&schema, &RenderDataSchemaOptions {
    format: "markdown".into(),
    ..Default::default()
})?;
```

No Rust plugin renders yet, so the result is `None` for every format.

## Working with DataSchema

`DataSchema` is a JSON object. You can construct one literally or load it from a file:

```rust
use fairspec::fairspec_metadata::{DataSchema, load_data_schema};
use serde_json::json;

// From a literal
let schema: DataSchema = serde_json::from_value(json!({
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "properties": {
        "host": {"type": "string"},
        "port": {"type": "integer", "minimum": 1, "maximum": 65535},
        "ssl": {"type": "boolean"},
    },
    "required": ["host", "port"],
}))?;

// From a file
let schema = load_data_schema("config-schema.json")?;
```

## Common Workflows

### Create and Validate with a Schema

```rust
use fairspec::{infer_data_schema, validate_data};
use serde_json::json;

// Infer schema from existing data
let sample = serde_json::from_value(json!({"data": "sample-data.json"}))?;
let schema = infer_data_schema(&sample)?;

// Validate new data against the schema
let resource = serde_json::from_value(json!({"data": "new-data.json", "dataSchema": schema}))?;
let report = validate_data(&resource)?;

assert!(report.valid, "{:?}", report.errors);
```

### Schema-Driven Development

```rust
use fairspec::fairspec_metadata::{DataSchema, validate_data_schema};
use fairspec::validate_data;
use serde_json::json;

let api_schema: DataSchema = serde_json::from_value(json!({
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "properties": {
        "users": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": {"type": "integer"},
                    "username": {"type": "string", "minLength": 3},
                    "email": {"type": "string", "format": "email"},
                },
                "required": ["id", "username", "email"],
            },
        },
    },
}))?;

// Validate the schema itself first
let schema_result = validate_data_schema(&api_schema, None)?;
assert!(schema_result.valid);

// Then validate an API response against it
let resource =
    serde_json::from_value(json!({"data": "response.json", "dataSchema": api_schema}))?;
let report = validate_data(&resource)?;
assert!(report.valid);
```

### Automated Testing

```rust
use fairspec::fairspec_metadata::load_data_schema;
use fairspec::validate_data;
use serde_json::json;

let schema = load_data_schema("schema.json")?;

let mut failed = false;
for entry in std::fs::read_dir("test-data")? {
    let path = entry?.path();
    if path.extension().is_none_or(|extension| extension != "json") {
        continue;
    }
    let resource = serde_json::from_value(json!({
        "data": path.to_string_lossy(),
        "dataSchema": schema,
    }))?;
    if validate_data(&resource)?.valid {
        println!("OK   {}", path.display());
    } else {
        println!("FAIL {}", path.display());
        failed = true;
    }
}

std::process::exit(if failed { 1 } else { 0 });
```

## Examples

### API Response Validation

```rust
use fairspec::{infer_data_schema, load_data, validate_data};
use serde_json::json;

// Fetch an API response and infer its schema
let remote = serde_json::from_value(json!({"data": "https://api.example.com/users"}))?;
let sample = load_data(&remote)?;

let resource = serde_json::from_value(json!({"data": sample}))?;
let schema = infer_data_schema(&resource)?;

// Validate future responses against it
let new_data = load_data(&remote)?;
let resource = serde_json::from_value(json!({"data": new_data, "dataSchema": schema}))?;
let report = validate_data(&resource)?;
assert!(report.valid, "{:?}", report.errors);
```

### Configuration File Validation

```rust
use fairspec::validate_data;
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": "config.json",
    "dataSchema": {
        "type": "object",
        "properties": {
            "host": {"type": "string"},
            "port": {"type": "integer", "minimum": 1, "maximum": 65535},
            "ssl": {"type": "boolean"},
        },
        "required": ["host", "port"],
    },
}))?;

let report = validate_data(&resource)?;
assert!(report.valid);
```

### Schema Evolution

```rust
use fairspec::fairspec_metadata::validate_data_schema;
use fairspec::{infer_data_schema, validate_data};
use serde_json::json;

// Start with the inferred schema from v1 data
let v1 = serde_json::from_value(json!({"data": "data-v1.json"}))?;
let mut schema = infer_data_schema(&v1)?.ok_or("no schema inferred")?;

// Manually evolve to v2 (add an optional property)
if let Some(properties) = schema.get_mut("properties").and_then(|value| value.as_object_mut()) {
    properties.insert("new_optional_field".into(), json!({"type": "string"}));
}

// Confirm v2 schema is still well-formed
assert!(validate_data_schema(&schema, None)?.valid);

// Confirm v1 data is still compatible with v2 schema
let resource = serde_json::from_value(json!({"data": "data-v1.json", "dataSchema": schema}))?;
assert!(validate_data(&resource)?.valid);
```
