from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import get_base_property_type, get_columns

if TYPE_CHECKING:
    from fairspec_metadata import Column
    from fairspec_metadata import Descriptor

NUMERIC_KEYS = ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum")
ITEM_TYPES = {
    "string": "string",
    "integer": "integer",
    "number": "number",
    "boolean": "boolean",
    "date": "date",
    "time": "time",
    "local-time": "time",
    "date-time": "datetime",
    "local-date-time": "datetime",
}


def convert_table_schema_to_frictionless(table_schema: Descriptor) -> Descriptor:
    """Convert a Fairspec Table Schema to a Frictionless Table Schema descriptor"""
    schema: Descriptor = {}
    properties: Descriptor = table_schema.get("properties") or {}

    schema["fields"] = [
        _convert_column(column, properties.get(column.name) or {})
        for column in get_columns(table_schema)
    ]

    # Fairspec formats read an empty cell as null, which Frictionless does only through `""`
    missing_values = list(table_schema.get("missingValues") or [])
    if "" not in missing_values:
        missing_values.append("")
    is_default = len(missing_values) == 1
    if not is_default:
        schema["missingValues"] = missing_values

    if table_schema.get("primaryKey") is not None:
        schema["primaryKey"] = table_schema["primaryKey"]
    if table_schema.get("uniqueKeys") is not None:
        schema["uniqueKeys"] = table_schema["uniqueKeys"]

    if table_schema.get("foreignKeys") is not None:
        schema["foreignKeys"] = [
            {
                "fields": foreign_key["columns"],
                "reference": {
                    "resource": foreign_key["reference"].get("resource") or "",
                    "fields": foreign_key["reference"]["columns"],
                },
            }
            for foreign_key in table_schema["foreignKeys"]
        ]

    return schema


def _convert_column(column: Column, property: Descriptor) -> Descriptor:
    field: Descriptor = {"name": column.name, **_convert_type(column, property)}
    constraints: Descriptor = {}

    if property.get("title"):
        field["title"] = property["title"]
    if property.get("description"):
        field["description"] = property["description"]
    if property.get("rdfType"):
        field["rdfType"] = property["rdfType"]

    if not column.nullable and column.type != "unknown":
        constraints["required"] = True

    match column.type:
        case "integer" | "number" | "decimal":
            for key in NUMERIC_KEYS:
                if property.get(key) is not None:
                    constraints[key] = property[key]
            if property.get("groupChar"):
                field["groupChar"] = property["groupChar"]
            if property.get("withText"):
                field["bareNumber"] = False
            if column.type != "integer" and property.get("decimalChar"):
                field["decimalChar"] = property["decimalChar"]
        case "string":
            if property.get("minLength") is not None:
                constraints["minLength"] = property["minLength"]
            if property.get("maxLength") is not None:
                constraints["maxLength"] = property["maxLength"]
            if property.get("pattern"):
                constraints["pattern"] = property["pattern"]
        case "boolean":
            if property.get("trueValues") is not None:
                field["trueValues"] = property["trueValues"]
            if property.get("falseValues") is not None:
                field["falseValues"] = property["falseValues"]
        case "list":
            if property.get("delimiter"):
                field["delimiter"] = property["delimiter"]
            item_type = ITEM_TYPES.get(property.get("itemType") or "")
            if item_type:
                field["itemType"] = item_type
            if property.get("minItems") is not None:
                constraints["minLength"] = property["minItems"]
            if property.get("maxItems") is not None:
                constraints["maxLength"] = property["maxItems"]
        case "categorical":
            if property.get("categories") is not None:
                field["categories"] = property["categories"]
            if property.get("withOrder"):
                field["categoriesOrdered"] = True
        case "date" | "time" | "local-time" | "date-time" | "local-date-time":
            if property.get("temporalFormat"):
                field["format"] = property["temporalFormat"]

    enum = property.get("enum")
    if isinstance(enum, list):
        constraints["enum"] = [value for value in enum if value is not None]

    if constraints:
        field["constraints"] = constraints

    return field


def _convert_type(column: Column, property: Descriptor) -> Descriptor:
    match column.type:
        case "string" | "hex" | "wkt" | "wkb":
            return {"type": "string"}
        case "email":
            return {"type": "string", "format": "email"}
        case "url":
            return {"type": "string", "format": "uri"}
        case "base64":
            return {"type": "string", "format": "binary"}
        case "categorical":
            is_integer = get_base_property_type(property.get("type")) == "integer"
            return {"type": "integer" if is_integer else "string"}
        case "integer":
            return {"type": "integer"}
        case "number" | "decimal":
            return {"type": "number"}
        case "boolean":
            return {"type": "boolean"}
        case "date":
            return {"type": "date"}
        case "time" | "local-time":
            return {"type": "time"}
        case "date-time" | "local-date-time":
            return {"type": "datetime"}
        case "duration":
            return {"type": "duration"}
        case "list":
            return {"type": "list"}
        case "array":
            return {"type": "array"}
        case "object":
            return {"type": "object"}
        case "geojson":
            return {"type": "geojson"}
        case "topojson":
            return {"type": "geojson", "format": "topojson"}
        case _:
            return {"type": "any"}
