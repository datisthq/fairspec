from __future__ import annotations

from typing import TYPE_CHECKING

from pydantic import BaseModel

from fairspec_metadata.actions.frictionless.list import convert_field_names
from fairspec_metadata.actions.frictionless.mapping import skip_frictionless_record
from fairspec_metadata.actions.frictionless.name import convert_name_from_frictionless
from fairspec_metadata.models.frictionless.field import FrictionlessConstraints

if TYPE_CHECKING:
    from fairspec_metadata.actions.frictionless.mapping import FrictionlessRecorder
    from fairspec_metadata.models.descriptor import Descriptor
    from fairspec_metadata.models.frictionless.field import (
        FrictionlessField,
        FrictionlessMissingValue,
    )
    from fairspec_metadata.models.frictionless.schema import FrictionlessSchema

FIELD_KEYS = {
    "type": "type",
    "title": "title",
    "description": "description",
    "rdfType": "rdfType",
    "examples": "example",
    "missingValues": "missingValues",
    "enum": "constraints/enum",
    "minimum": "constraints/minimum",
    "maximum": "constraints/maximum",
    "exclusiveMinimum": "constraints/exclusiveMinimum",
    "exclusiveMaximum": "constraints/exclusiveMaximum",
    "minLength": "constraints/minLength",
    "maxLength": "constraints/maxLength",
    "minItems": "constraints/minLength",
    "maxItems": "constraints/maxLength",
    "pattern": "constraints/pattern",
    "decimalChar": "decimalChar",
    "groupChar": "groupChar",
    "withText": "bareNumber",
    "trueValues": "trueValues",
    "falseValues": "falseValues",
    "categories": "categories",
    "withOrder": "categoriesOrdered",
    "delimiter": "delimiter",
    "itemType": "itemType",
    "temporalFormat": "format",
}
VERBATIM_KEYS = ("missingValues", "enum", "trueValues", "falseValues", "categories")

STRING_TYPES = ("string", "email", "url", "base64", "categorical", "yearmonth")
NUMERIC_TYPES = ("integer", "number", "year")
NUMERIC_KEYS = ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum")
ITEM_TYPES = {
    "string": "string",
    "integer": "integer",
    "number": "number",
    "boolean": "boolean",
    "date": "date",
    "time": "local-time",
    "datetime": "date-time",
}


def convert_table_schema_from_frictionless(
    schema: FrictionlessSchema, *, record: FrictionlessRecorder = skip_frictionless_record
) -> Descriptor:
    """Convert a Frictionless Table Schema to a Fairspec Table Schema descriptor"""
    table_schema: Descriptor = {}
    properties: dict[str, Descriptor] = {}
    unique_keys = list(schema.uniqueKeys or [])

    for index, field in enumerate(schema.fields):
        property = _convert_field(field)
        properties[field.name] = property

        fairspec = f"/properties/{_escape_pointer_key(field.name)}"
        frictionless = f"/fields/{index}"
        record(fairspec, frictionless)
        for key in property:
            target = FIELD_KEYS.get(key)
            if not target:
                continue
            record(
                f"{fairspec}/{key}",
                f"{frictionless}/{target}",
                verbatim=key in VERBATIM_KEYS,
            )

        if field.constraints and field.constraints.unique:
            record(
                f"/uniqueKeys/{len(unique_keys)}", f"{frictionless}/constraints/unique"
            )
            unique_keys.append([field.name])

    record("/properties", "/fields")
    record("/missingValues", "/missingValues", verbatim=True)
    record("/primaryKey", "/primaryKey")
    for index in range(len(schema.uniqueKeys or [])):
        record(f"/uniqueKeys/{index}", f"/uniqueKeys/{index}", verbatim=True)
    for index in range(len(schema.foreignKeys or [])):
        prefix = f"/foreignKeys/{index}"
        record(prefix, prefix)
        record(f"{prefix}/columns", f"{prefix}/fields")
        record(f"{prefix}/reference", f"{prefix}/reference")
        record(f"{prefix}/reference/resource", f"{prefix}/reference/resource")
        record(f"{prefix}/reference/columns", f"{prefix}/reference/fields")

    table_schema["properties"] = properties

    # Fairspec formats already read an empty cell as null, Frictionless's default `[""]`
    missing_values = _dump_missing_values(schema.missingValues or [])
    is_default = missing_values == [""]
    if missing_values and not is_default:
        table_schema["missingValues"] = missing_values

    if schema.primaryKey:
        primary_key = convert_field_names(schema.primaryKey)
        if primary_key:
            table_schema["primaryKey"] = primary_key

    if unique_keys:
        table_schema["uniqueKeys"] = unique_keys

    if schema.foreignKeys:
        foreign_keys: list[Descriptor] = []
        for foreign_key in schema.foreignKeys:
            reference: Descriptor = {}
            if foreign_key.reference.resource:
                reference["resource"] = convert_name_from_frictionless(
                    foreign_key.reference.resource
                )
            reference["columns"] = convert_field_names(foreign_key.reference.fields)
            foreign_keys.append(
                {
                    "columns": convert_field_names(foreign_key.fields),
                    "reference": reference,
                }
            )
        table_schema["foreignKeys"] = foreign_keys

    return table_schema


def _convert_field(field: FrictionlessField) -> Descriptor:
    constraints = field.constraints or FrictionlessConstraints()
    kind, property = _convert_type(field)

    if field.title:
        property["title"] = field.title
    if field.description:
        property["description"] = field.description
    if field.rdfType:
        property["rdfType"] = field.rdfType
    if "example" in field.model_fields_set:
        property["examples"] = [field.example]
    if field.missingValues:
        property["missingValues"] = _dump_missing_values(field.missingValues)

    if constraints.enum is not None:
        property["enum"] = constraints.enum

    if kind in NUMERIC_TYPES:
        for key in NUMERIC_KEYS:
            value = getattr(constraints, key)
            if isinstance(value, int | float) and not isinstance(value, bool):
                property[key] = value

    if kind in STRING_TYPES:
        if constraints.minLength is not None:
            property["minLength"] = constraints.minLength
        if constraints.maxLength is not None:
            property["maxLength"] = constraints.maxLength
        if constraints.pattern:
            property["pattern"] = constraints.pattern

    if kind == "list":
        if constraints.minLength is not None:
            property["minItems"] = constraints.minLength
        if constraints.maxLength is not None:
            property["maxItems"] = constraints.maxLength

    if kind in ("object", "array") and constraints.jsonSchema is not None:
        property.update(constraints.jsonSchema)

    if not constraints.required and isinstance(property.get("type"), str):
        property["type"] = [property["type"], "null"]

    return property


def _convert_type(field: FrictionlessField) -> tuple[str, Descriptor]:
    format = field.format
    temporal_format = _convert_temporal_format(format)

    match field.type or "string":
        case "string":
            if field.categories is not None:
                return "categorical", _convert_categorical("string", field)
            if format == "email":
                return "email", {"type": "string", "format": "email"}
            if format == "uri":
                return "url", {"type": "string", "format": "url"}
            if format == "binary":
                return "base64", {"type": "string", "format": "base64"}
            return "string", {"type": "string"}
        case "integer":
            if field.categories is not None:
                return "categorical", _convert_categorical("integer", field)
            property: Descriptor = {"type": "integer"}
            if field.groupChar:
                property["groupChar"] = field.groupChar
            if field.bareNumber is False:
                property["withText"] = True
            return "integer", property
        case "number":
            property = {"type": "number"}
            if field.decimalChar:
                property["decimalChar"] = field.decimalChar
            if field.groupChar:
                property["groupChar"] = field.groupChar
            if field.bareNumber is False:
                property["withText"] = True
            return "number", property
        case "boolean":
            property = {"type": "boolean"}
            if field.trueValues is not None:
                property["trueValues"] = field.trueValues
            if field.falseValues is not None:
                property["falseValues"] = field.falseValues
            return "boolean", property
        case "date":
            return "date", _create_temporal("date", temporal_format)
        case "time":
            with_offset = temporal_format is not None and "z" in temporal_format
            return "time", _create_temporal(
                "time" if with_offset else "local-time", temporal_format
            )
        case "datetime":
            is_local = temporal_format is not None and "z" not in temporal_format
            return "datetime", _create_temporal(
                "local-date-time" if is_local else "date-time", temporal_format
            )
        case "year":
            return "year", {"type": "integer"}
        case "yearmonth":
            return "yearmonth", {"type": "string", "pattern": "^\\d{4}-\\d{2}$"}
        case "duration":
            return "duration", {"type": "string", "format": "duration"}
        case "geopoint":
            if format == "array":
                return "array", {"type": "array"}
            if format == "object":
                return "object", {"type": "object"}
            return "string", {"type": "string"}
        case "geojson":
            geo_format = "topojson" if format == "topojson" else "geojson"
            return "geojson", {"type": "object", "format": geo_format}
        case "object":
            return "object", {"type": "object"}
        case "array":
            return "array", {"type": "array"}
        case "list":
            property = {"type": "string", "format": "list"}
            if field.delimiter:
                property["delimiter"] = field.delimiter
            item_type = ITEM_TYPES.get(field.itemType) if field.itemType else None
            if item_type:
                property["itemType"] = item_type
            return "list", property
        case _:
            return "unknown", {}


def _convert_categorical(type: str, field: FrictionlessField) -> Descriptor:
    property: Descriptor = {
        "type": type,
        "format": "categorical",
        "categories": field.categories,
    }
    if field.categoriesOrdered:
        property["withOrder"] = True
    return property


def _create_temporal(format: str, temporal_format: str | None) -> Descriptor:
    property: Descriptor = {"type": "string", "format": format}
    if temporal_format:
        property["temporalFormat"] = temporal_format
    return property


def _convert_temporal_format(format: str | None) -> str | None:
    if not format or format in ("default", "any"):
        return None
    return format[4:] if format.startswith("fmt:") else format


def _dump_missing_values(values: list[FrictionlessMissingValue]) -> list[object]:
    return [
        value.model_dump(by_alias=True, exclude_unset=True)
        if isinstance(value, BaseModel)
        else value
        for value in values
    ]


def _escape_pointer_key(key: str) -> str:
    return key.replace("~", "~0").replace("/", "~1")
