---
title: Extensions
label: Extensions
path: /extensions/
order: 30
icon: puzzle
---

Fairspec Dataset has a simple yet powerful extension mechanism based on JSON Schema. Extensions allow you to create domain-specific profiles that add custom properties and validation rules while maintaining compatibility with the base specification.

> [!TIP]
> Use the [Fairspec Extension](https://fairspec.github.io/fairspec-extension/) repository template for rapid extensions development.

## How Extensions Work

An extension is a JSON Schema that:

1. References the base Fairspec Dataset schema in an `allOf` property
2. Adds domain-specific properties and constraints
3. Can be referenced via the `$schema` property in dataset descriptors

Using JSON Schema features, extensions can:

- Add new custom properties for domain-specific metadata
- Require specific properties or values
- Define expected resource types and their schemas
- Enforce validation rules unique to your domain
- Combine multiple profiles into higher-level extensions

Extensions are ideal for:

- Scientific domains with specialized metadata (spectroscopy, genomics, imaging)
- Organizations with internal data standards
- Research communities with shared conventions
- Compliance with domain-specific requirements

See the [Dataset Specification](../../specs/dataset#extension) for complete technical details.

## Example: Spectroscopy Extension

A dataset using a custom spectroscopy profile:

```json
{
  "$schema": "https://spectroscopy.org/profiels/1.0.0/dataset.json",
  "title": "Infrared Spectroscopy Data",
  "resources": [
    {
      "data": "spectrum.csv",
      "spectralRange": {
        "min": 400,
        "max": 4000,
        "unit": "cm-1"
      }
    }
  ]
}
```

The extension schema that enables this:

```json
{
  "$schema": "http://json-schema.org/draft/2020-12/schema",
  "title": "Fairspec Spectroscopy Profile",
  "allOf": [
    { "$ref": "https://fairspec.org/profiles/1.0.0/dataset.json" },
    { "$ref": "#/$defs/spectroscopyMixin" }
  ],
  "$defs": {
    "spectroscopyMixin": {
      "type": "object",
      "properties": {
        "resources": {
          "type": "array",
          "items": {
            "properties": {
              "spectralRange": {
                "type": "object",
                "required": ["min", "max", "unit"],
                "properties": {
                  "min": { "type": "number" },
                  "max": { "type": "number" },
                  "unit": { "type": "string" }
                }
              }
            }
          }
        }
      }
    }
  }
}
```
