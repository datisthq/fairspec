---
title: Working with JSON Data in Python
label: Data
path: /python/data/
order: 3
---
JSON data validation and schema operations using JSON Schema standards — from Python.

## Available Functions

The data API provides utilities for working with JSON data (non-tabular):

- `load_data` - Load a JSON payload from a resource (inline or file)
- `validate_data` - Validate a JSON payload against a Data Schema
- `infer_data_schema` - Automatically infer a Data Schema from JSON data
- `validate_data_schema` - Validate the Data Schema itself
- `render_data_schema_as` - Render a Data Schema as Markdown or another format

A "Data Schema" is a JSON Schema (Draft 2020-12 compatible) describing arbitrary structured JSON — distinct from a Table Schema, which describes columnar tabular data.

## Loading Data

Load a JSON payload from a resource. The payload can be inline (literal Python object stored on the resource) or fetched from a local file or remote URL:

```python
from fairspec import Resource, load_data

# Load from a local file
data = load_data(Resource(data="data.json"))

# Load from a remote URL
data = load_data(Resource(data="https://example.com/data.json"))

# Load inline data (descriptor already contains the payload)
data = load_data(Resource(data={"users": [{"id": 1, "name": "Alice"}]}))
```

`load_data` returns `object | None` — typically a `dict`, `list`, or primitive, depending on the JSON content. The result is `None` if the source cannot be resolved.

## Validating Data

Validate a JSON payload against the `dataSchema` attached to the resource:

```python
from fairspec import DataSchema, Resource, validate_data

resource = Resource(
    data="user.json",
    dataSchema=DataSchema.model_validate({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "email": {"type": "string", "format": "email"},
            "age": {"type": "integer", "minimum": 0},
        },
        "required": ["name", "email"],
    }),
)

report = validate_data(resource)

if not report.valid:
    for error in report.errors:
        print(error.message)
```

The returned `Report` has `valid: bool` and `errors: list[FairspecError]`. Errors carry context including `instancePath` (JSON Pointer to the failing value) and `keyword` (the failing schema keyword such as `format` or `minimum`).

Example error output:

```python
# valid=False
# errors=[
#   DataError(
#     type='data',
#     instancePath='/users/0/email',
#     schemaPath='#/properties/users/items/properties/email/format',
#     keyword='format',
#     message='must match format "email"',
#   )
# ]
```

## Inferring a Data Schema

Automatically generate a JSON Schema from a JSON payload:

```python
from fairspec import Resource, infer_data_schema

schema = infer_data_schema(Resource(data="users.json"))
```

The inference detects:

- Data types (`string`, `number`, `integer`, `boolean`, `null`)
- Object structures and nested properties
- Array items and their types
- Required properties based on presence
- Enum values for low-cardinality fields

The result is a `JsonSchema` (a dict-like object) that you can wrap as a `DataSchema`:

```python
from fairspec import DataSchema

schema = DataSchema.model_validate(schema)
```

Example — given `users.json`:

```json
[
  { "id": 1, "name": "Alice", "email": "alice@example.com", "age": 30, "active": true },
  { "id": 2, "name": "Bob",   "email": "bob@example.com",   "age": 25, "active": false }
]
```

The inferred schema is:

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "id":     { "type": "integer" },
      "name":   { "type": "string"  },
      "email":  { "type": "string"  },
      "age":    { "type": "integer" },
      "active": { "type": "boolean" }
    },
    "required": ["id", "name", "email", "age", "active"]
  }
}
```

## Validating a Data Schema

Validate that a Data Schema itself is well-formed (valid JSON Schema):

```python
from fairspec import validate_data_schema

result = validate_data_schema("schema.json")

if result.valid:
    print("Schema is valid")
else:
    for error in result.errors:
        print(error.message)
```

`validate_data_schema` accepts either a descriptor path/URL or a literal `Descriptor`. The check confirms the schema is:

- Valid JSON
- Compliant with JSON Schema Draft 2020-12
- Using valid keywords and formats

To validate only a sub-schema inside a larger descriptor, pass `root_json_pointer`:

```python
result = validate_data_schema("dataset.json", root_json_pointer="/resources/0/dataSchema")
```

## Rendering a Data Schema

Render a Data Schema as Markdown documentation or another supported format:

```python
from fairspec import RenderDataSchemaOptions, render_data_schema_as

markdown = render_data_schema_as(schema, RenderDataSchemaOptions(format="markdown"))
```

## Working with DataSchema

`DataSchema` is a Pydantic model wrapping a JSON Schema. You can construct one literally or load it from a file:

```python
from fairspec import DataSchema, load_data_schema

# From a literal
schema = DataSchema.model_validate({
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "properties": {
        "host": {"type": "string"},
        "port": {"type": "integer", "minimum": 1, "maximum": 65535},
        "ssl":  {"type": "boolean"},
    },
    "required": ["host", "port"],
})

# From a file
schema = load_data_schema("config-schema.json")
```

## Common Workflows

### Create and Validate with a Schema

```python
from fairspec import DataSchema, Resource, infer_data_schema, validate_data

# Infer schema from existing data
schema_dict = infer_data_schema(Resource(data="sample-data.json"))
schema = DataSchema.model_validate(schema_dict)

# Validate new data against the schema
new_resource = Resource(data="new-data.json", dataSchema=schema)
report = validate_data(new_resource)

assert report.valid, report.errors
```

### Schema-Driven Development

```python
from fairspec import DataSchema, Resource, validate_data, validate_data_schema

api_schema = DataSchema.model_validate({
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
})

# Validate the schema itself first
schema_result = validate_data_schema(api_schema.model_dump())
assert schema_result.valid

# Then validate an API response against it
report = validate_data(Resource(data="response.json", dataSchema=api_schema))
assert report.valid
```

### Automated Testing

```python
import sys
from pathlib import Path
from fairspec import DataSchema, Resource, load_data_schema, validate_data

schema = load_data_schema("schema.json")

failed = False
for file in Path("test-data").glob("*.json"):
    report = validate_data(Resource(data=str(file), dataSchema=schema))
    if report.valid:
        print(f"OK   {file}")
    else:
        print(f"FAIL {file}")
        failed = True

sys.exit(1 if failed else 0)
```

## Examples

### API Response Validation

```python
import urllib.request
import json
from fairspec import DataSchema, Resource, infer_data_schema, validate_data

# Fetch an API response and infer its schema
with urllib.request.urlopen("https://api.example.com/users") as response:
    sample = json.load(response)

schema_dict = infer_data_schema(Resource(data=sample))
schema = DataSchema.model_validate(schema_dict)

# Validate future responses against it
with urllib.request.urlopen("https://api.example.com/users") as response:
    new_data = json.load(response)

report = validate_data(Resource(data=new_data, dataSchema=schema))
assert report.valid, report.errors
```

### Configuration File Validation

```python
from fairspec import DataSchema, Resource, validate_data

config_schema = DataSchema.model_validate({
    "type": "object",
    "properties": {
        "host": {"type": "string"},
        "port": {"type": "integer", "minimum": 1, "maximum": 65535},
        "ssl":  {"type": "boolean"},
    },
    "required": ["host", "port"],
})

report = validate_data(Resource(data="config.json", dataSchema=config_schema))
assert report.valid
```

### Schema Evolution

```python
from fairspec import DataSchema, Resource, infer_data_schema, load_data_schema, validate_data, validate_data_schema

# Start with the inferred schema from v1 data
schema_v1_dict = infer_data_schema(Resource(data="data-v1.json"))

# Manually evolve to v2 (add an optional property, relax a constraint)
schema_v2_dict = dict(schema_v1_dict)
schema_v2_dict.setdefault("properties", {})["new_optional_field"] = {"type": "string"}

schema_v2 = DataSchema.model_validate(schema_v2_dict)

# Confirm v2 schema is still well-formed
assert validate_data_schema(schema_v2.model_dump()).valid

# Confirm v1 data is still compatible with v2 schema
assert validate_data(Resource(data="data-v1.json", dataSchema=schema_v2)).valid
```
