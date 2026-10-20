import json
import sys
from dataclasses import dataclass
from typing import Literal

Kind = Literal["integer", "number", "string", "boolean", "date-time"]


@dataclass
class Column:
    name: str
    kind: Kind
    nullable: bool
    minimum: float | None
    maximum: float | None
    min_length: int | None
    pattern: str | None
    enum: list[str] | None


@dataclass
class Schema:
    columns: list[Column]
    primary_key: list[str]


DATE_TIME_PATTERN = r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$"
BOOLEAN_PATTERN = r"^(true|false)$"


def load_schema(path: str) -> Schema:
    with open(path) as file:
        descriptor = json.load(file)
    required = set(descriptor.get("required", []))
    columns: list[Column] = []
    for name, prop in descriptor["properties"].items():
        types = prop["type"] if isinstance(prop["type"], list) else [prop["type"]]
        base = next(item for item in types if item != "null")
        kind: Kind = "date-time" if prop.get("format") == "local-date-time" else base
        columns.append(
            Column(
                name=name,
                kind=kind,
                nullable=name not in required,
                minimum=prop.get("minimum"),
                maximum=prop.get("maximum"),
                min_length=prop.get("minLength"),
                pattern=prop.get("pattern"),
                enum=prop.get("enum"),
            )
        )
    return Schema(columns=columns, primary_key=descriptor.get("primaryKey", []))


def print_result(error_count: int) -> None:
    print(json.dumps({"errorCount": error_count}))


def read_arguments() -> tuple[str, str]:
    _, csv_path, schema_path = sys.argv
    return csv_path, schema_path
