---
title: Examples
path: /overview/examples/
order: 3
---

This guide demonstrates how to create Fairspec descriptors for real-world use cases. We'll walk through examples of catalogs, datasets, and table schemas.

## Catalog Example

A Fairspec Catalog is a JSON Lines file listing multiple datasets with their update timestamps:

```jsonl
{"loc": "https://climate.example.org/datasets/temperature-2024.json", "upd": "2024-03-15T10:30:00Z"}
{"loc": "https://climate.example.org/datasets/precipitation-2024.json", "upd": "2024-03-10T14:20:00Z"}
{"loc": "https://climate.example.org/datasets/wind-patterns-2024.json", "upd": "2024-03-05T09:15:00Z"}
{"loc": "https://climate.example.org/datasets/solar-radiation-2024.json", "upd": "2024-02-28T16:45:00Z"}
{"loc": "https://climate.example.org/datasets/atmospheric-pressure-2024.json", "upd": "2024-02-20T11:00:00Z"}
```

## Dataset Example

A Fairspec Dataset describes a collection of related data resources with rich metadata. Here's an example of a climate research dataset:

```json
{
  "$schema": "https://fairspec.org/profiles/1.0.0/dataset.json",
  "doi": "10.1234/climate-2024",
  "title": "Global Temperature Measurements 2024",
  "description": "Daily temperature readings from weather stations across 50 countries, collected during January-March 2024.",
  "creators": [
    {
      "name": "Smith, Jane",
      "nameType": "Personal",
      "affiliation": "Climate Research Institute"
    }
  ],
  "publicationYear": 2024,
  "publisher": "Climate Research Institute",
  "subjects": [
    {
      "subject": "Climate Science"
    },
    {
      "subject": "Meteorology"
    }
  ],
  "language": "en",
  "resourceType": {
    "resourceTypeGeneral": "Dataset"
  },
  "resources": [
    {
      "name": "measurements",
      "data": "temperature_data.csv",
      "format": {
        "name": "csv",
        "delimiter": ",",
        "headerRows": [1]
      },
      "integrity": {
        "type": "sha256",
        "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      },
      "tableSchema": "temperature_schema.json",
      "title": "Temperature Measurements",
      "description": "Daily temperature readings with station metadata"
    },
    {
      "name": "stations",
      "data": "stations.json",
      "format": {
        "name": "json"
      },
      "title": "Weather Stations",
      "description": "Metadata about weather station locations and equipment",
      "geoLocations": [
        {
          "geoLocationPlace": "Global"
        }
      ]
    }
  ]
}
```

## File Dialect Example

Fairspec File Dialect defines how file formats should be interpreted:

```json
{
  "format": "csv",
  "delimiter": ";",
  "headerRows": [1],
  "commentPrefix": "#"
}
```

This file dialect specifies a CSV file using semicolons as delimiters, with headers in the first row and lines starting with `#` treated as comments.

## Data Schema Example

Fairspec Data Schema defines the structure and validation rules for JSON data. Here's an example schema for a research instrument configuration:

```json
{
  "$schema": "https://fairspec.org/profiles/1.0.0/data-schema.json",
  "type": "object",
  "title": "Spectrometer Configuration",
  "description": "Configuration schema for UV-Vis spectrometer settings",
  "required": ["instrument_id", "wavelength_range", "scan_parameters"],
  "properties": {
    "instrument_id": {
      "type": "string",
      "description": "Unique identifier for the instrument",
      "pattern": "^SPEC-[0-9]{4}$"
    },
    "wavelength_range": {
      "type": "object",
      "description": "Wavelength scan range in nanometers",
      "required": ["min", "max"],
      "properties": {
        "min": {
          "type": "number",
          "minimum": 190,
          "maximum": 1100
        },
        "max": {
          "type": "number",
          "minimum": 190,
          "maximum": 1100
        }
      }
    },
    "scan_parameters": {
      "type": "object",
      "required": ["scan_speed", "data_interval"],
      "properties": {
        "scan_speed": {
          "type": "string",
          "enum": ["slow", "medium", "fast", "very_fast"]
        },
        "data_interval": {
          "type": "number",
          "description": "Data collection interval in nm",
          "minimum": 0.1,
          "maximum": 5.0
        },
        "baseline_correction": {
          "type": "boolean",
          "default": true
        }
      }
    },
    "calibration_date": {
      "type": "string",
      "format": "date",
      "description": "Date of last calibration"
    },
    "operator": {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "id": {
          "type": "string"
        }
      }
    }
  }
}
```

This schema validates JSON configuration files like:

```json
{
  "instrument_id": "SPEC-0042",
  "wavelength_range": {
    "min": 200,
    "max": 800
  },
  "scan_parameters": {
    "scan_speed": "medium",
    "data_interval": 1.0,
    "baseline_correction": true
  },
  "calibration_date": "2024-03-01",
  "operator": {
    "name": "Alice Johnson",
    "id": "USER-123"
  }
}
```

## Table Schema Example

Fairspec Table Schema defines the structure and constraints for tabular data. Here's the schema for the temperature measurements above:

```json
{
  "$schema": "https://fairspec.org/profiles/1.0.0/schema.json",
  "title": "Temperature Measurement Schema",
  "description": "Schema for daily temperature readings from weather stations",
  "required": ["station_id", "date", "temperature"],
  "properties": {
    "station_id": {
      "type": "string",
      "title": "Station ID",
      "description": "Unique identifier for the weather station",
      "pattern": "^[A-Z]{2}-[0-9]{4}$",
      "examples": ["US-0123", "UK-5678"]
    },
    "date": {
      "type": "string",
      "format": "date",
      "title": "Measurement Date",
      "description": "Date when the temperature was recorded"
    },
    "temperature": {
      "type": "number",
      "title": "Temperature (°C)",
      "description": "Temperature in degrees Celsius",
      "minimum": -89.2,
      "maximum": 56.7
    },
    "humidity": {
      "type": "number",
      "title": "Relative Humidity (%)",
      "description": "Relative humidity percentage",
      "minimum": 0,
      "maximum": 100,
      "missingValues": [{ "value": -999, "label": "Sensor malfunction" }]
    },
    "quality": {
      "type": "integer",
      "format": "categorical",
      "title": "Quality Rating",
      "description": "Data quality assessment",
      "withOrder": true,
      "categories": [
        { "value": 1, "label": "Poor" },
        { "value": 2, "label": "Fair" },
        { "value": 3, "label": "Good" },
        { "value": 4, "label": "Excellent" }
      ]
    }
  },
  "primaryKey": ["station_id", "date"],
  "foreignKeys": [
    {
      "columns": ["station_id"],
      "reference": {
        "resource": "stations",
        "columns": ["id"]
      }
    }
  ],
  "missingValues": ["NA", "N/A", ""]
}
```

## Complete Example: Research Laboratory Dataset

Here's a complete example showing how all three components work together for a materials science laboratory:

### Directory Structure

```
materials-lab-2024/
├── dataset.json          # Main dataset descriptor
├── experiments.csv       # Experimental results
├── samples.xlsx          # Sample metadata
├── experiments.schema.json  # Table schema for experiments
└── samples.schema.json      # Table schema for samples
```

### dataset.json

```json
{
  "$schema": "https://fairspec.org/profiles/1.0.0/dataset.json",
  "doi": "10.5678/materials-2024-q1",
  "title": "Polymer Synthesis Experiments - Q1 2024",
  "description": "Experimental results from polymer synthesis research conducted at the Materials Science Laboratory during January-March 2024.",
  "creators": [
    {
      "name": "Johnson, Robert",
      "nameType": "Personal",
      "affiliation": "State University Materials Lab"
    },
    {
      "name": "Chen, Li",
      "nameType": "Personal",
      "affiliation": "State University Materials Lab"
    }
  ],
  "contributors": [
    {
      "name": "Materials Science Laboratory",
      "nameType": "Organizational",
      "contributorType": "HostingInstitution"
    }
  ],
  "publicationYear": 2024,
  "publisher": "State University",
  "subjects": [
    {
      "subject": "Materials Science"
    },
    {
      "subject": "Polymer Chemistry"
    }
  ],
  "dates": [
    {
      "date": "2024-01-15",
      "dateType": "Collected"
    },
    {
      "date": "2024-03-30",
      "dateType": "Submitted"
    }
  ],
  "language": "en",
  "resourceType": {
    "resourceTypeGeneral": "Dataset",
    "resourceType": "Experimental Data"
  },
  "relatedIdentifiers": [
    {
      "relatedIdentifier": "10.1234/paper-2024",
      "relatedIdentifierType": "DOI",
      "relationType": "IsSupplementTo"
    }
  ],
  "resources": [
    {
      "name": "experiments",
      "data": "experiments.csv",
      "format": {
        "name": "csv",
        "delimiter": ",",
        "headerRows": [1],
        "commentPrefix": "#"
      },
      "textual": true,
      "integrity": {
        "type": "sha256",
        "hash": "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6"
      },
      "tableSchema": "experiments.schema.json",
      "title": "Experiment Results",
      "description": "Results from 150 polymer synthesis experiments including reaction conditions and measured properties"
    },
    {
      "name": "samples",
      "data": "samples.xlsx",
      "format": {
        "name": "xlsx",
        "sheetName": "Sample Data",
        "headerRows": [1, 2],
        "headerJoin": "_"
      },
      "integrity": {
        "type": "sha256",
        "hash": "z6y5x4w3v2u1t0s9r8q7p6o5n4m3l2k1j0i9h8g7f6e5d4c3b2a1"
      },
      "tableSchema": "samples.schema.json",
      "title": "Sample Information",
      "description": "Detailed information about each polymer sample including chemical composition and physical properties"
    }
  ]
}
```

### experiments.schema.json

```json
{
  "$schema": "https://fairspec.org/profiles/1.0.0/schema.json",
  "title": "Polymer Synthesis Experiment Schema",
  "required": ["experiment_id", "sample_id", "date", "temperature", "pressure"],
  "properties": {
    "experiment_id": {
      "type": "string",
      "title": "Experiment ID",
      "description": "Unique identifier for the experiment",
      "pattern": "^EXP-[0-9]{6}$"
    },
    "sample_id": {
      "type": "string",
      "title": "Sample ID",
      "description": "Identifier for the polymer sample produced",
      "pattern": "^POLY-[0-9]{4}$"
    },
    "date": {
      "type": "string",
      "format": "date",
      "title": "Experiment Date",
      "description": "Date when the experiment was conducted"
    },
    "temperature": {
      "type": "number",
      "title": "Reaction Temperature (°C)",
      "description": "Temperature maintained during polymerization",
      "minimum": 20,
      "maximum": 300
    },
    "pressure": {
      "type": "number",
      "title": "Reaction Pressure (bar)",
      "description": "Pressure maintained during polymerization",
      "minimum": 1,
      "maximum": 100
    },
    "catalyst": {
      "type": "string",
      "title": "Catalyst Used",
      "description": "Chemical name of the catalyst",
      "enum": ["TiCl4", "ZnCl2", "AlCl3", "none"]
    },
    "yield_percent": {
      "type": "number",
      "title": "Yield (%)",
      "description": "Percentage yield of the polymer product",
      "minimum": 0,
      "maximum": 100,
      "missingValues": [{ "value": -1, "label": "Experiment failed" }]
    },
    "molecular_weight": {
      "type": "integer",
      "title": "Molecular Weight (g/mol)",
      "description": "Average molecular weight of the polymer",
      "minimum": 1000,
      "groupChar": ","
    },
    "success": {
      "type": "boolean",
      "title": "Success Status",
      "description": "Whether the experiment was successful",
      "trueValues": ["yes", "true", "1"],
      "falseValues": ["no", "false", "0"]
    },
    "notes": {
      "type": "string",
      "title": "Notes",
      "description": "Additional observations and notes",
      "maxLength": 500
    }
  },
  "primaryKey": ["experiment_id"],
  "foreignKeys": [
    {
      "columns": ["sample_id"],
      "reference": {
        "resource": "samples",
        "columns": ["sample_id"]
      }
    }
  ],
  "missingValues": ["NA", "N/A", "", "not measured"]
}
```

## Multi-Format Dataset Example

Fairspec supports various data formats. Here's a dataset combining different file types:

```json
{
  "$schema": "https://fairspec.org/profiles/1.0.0/dataset.json",
  "title": "Multi-Format Environmental Dataset",
  "resources": [
    {
      "name": "time_series",
      "data": "sensors.parquet",
      "format": {
        "name": "parquet"
      },
      "title": "High-frequency sensor data in Parquet format"
    },
    {
      "name": "locations",
      "data": "locations.geojson",
      "format": {
        "name": "json",
        "jsonPointer": "/features"
      },
      "title": "Sensor locations in GeoJSON format"
    },
    {
      "name": "daily_summary",
      "data": "summary.jsonl",
      "format": {
        "name": "jsonl",
        "commentPrefix": "//",
        "rowType": "object"
      },
      "title": "Daily aggregated statistics"
    },
    {
      "name": "metadata",
      "data": {
        "project": "Environmental Monitoring 2024",
        "version": "1.0",
        "sensors": ["temperature", "humidity", "pressure"]
      },
      "title": "Inline metadata object"
    },
    {
      "name": "images",
      "data": ["photos/site1.jpg", "photos/site2.jpg", "photos/site3.jpg"],
      "title": "Site photographs",
      "description": "Visual documentation of sensor installation sites"
    }
  ]
}
```

## Advanced Schema Example

This example demonstrates advanced features like complex types and validation:

```json
{
  "$schema": "https://fairspec.org/profiles/1.0.0/schema.json",
  "title": "Advanced Feature Schema",
  "properties": {
    "id": {
      "type": "integer",
      "title": "Record ID"
    },
    "tags": {
      "type": "string",
      "format": "list",
      "title": "Tags",
      "delimiter": ";",
      "itemType": "string",
      "minItems": 1,
      "maxItems": 10
    },
    "coordinates": {
      "type": "array",
      "title": "Geographic Coordinates",
      "items": {
        "type": "number"
      },
      "minItems": 2,
      "maxItems": 2,
      "description": "[longitude, latitude]"
    },
    "properties": {
      "type": "object",
      "title": "Additional Properties",
      "properties": {
        "color": { "type": "string" },
        "size": { "type": "number" }
      }
    },
    "geometry": {
      "type": "object",
      "format": "geojson",
      "title": "GeoJSON Geometry"
    },
    "binary_data": {
      "type": "string",
      "format": "base64",
      "title": "Binary Data (Base64 encoded)",
      "maxLength": 10000
    },
    "price": {
      "type": "string",
      "format": "decimal",
      "title": "Price",
      "decimalChar": ",",
      "groupChar": ".",
      "withText": true
    },
    "measurement_time": {
      "type": "string",
      "format": "date-time",
      "title": "Measurement Timestamp",
      "temporalFormat": "%Y-%m-%d %H:%M:%S"
    }
  },
  "primaryKey": ["id"],
  "uniqueKeys": [["tags"]]
}
```

## Best Practices

### 1. Always Include Schema Versions

```json
{
  "$schema": "https://fairspec.org/profiles/1.0.0/dataset.json"
}
```

Specify exact versions to ensure compatibility and validation.

### 2. Provide Rich Metadata

```json
{
  "title": "Clear, descriptive title",
  "description": "Detailed description of what the data contains, how it was collected, and what it can be used for",
  "creators": [...],
  "subjects": [...],
  "dates": [...]
}
```

Good metadata makes your data discoverable and understandable.

### 3. Use Integrity Checks

```json
{
  "integrity": {
    "type": "sha256",
    "hash": "..."
  }
}
```

Protect data integrity with cryptographic hashes.

### 4. Define Clear Schema Constraints

```json
{
  "required": ["essential_field1", "essential_field2"],
  "primaryKey": ["id"],
  "foreignKeys": [...]
}
```

Explicit constraints help catch data quality issues early.

### 5. Document Missing Values

```json
{
  "missingValues": [
    { "value": "NA", "label": "Not Available" },
    { "value": -999, "label": "Sensor Error" }
  ]
}
```

Clear documentation of missing value codes prevents misinterpretation.
