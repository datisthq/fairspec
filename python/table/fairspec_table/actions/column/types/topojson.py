from __future__ import annotations

import json
from importlib.resources import files

from fairspec_metadata import CellError, TopojsonColumn

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table

from .._shared.helpers import inspect_json_column


def inspect_topojson_column(
    column: TopojsonColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    return inspect_json_column(
        column, table, mapping=mapping, type_json_schema=_load_topojson_schema()
    )


def _load_topojson_schema() -> dict[str, object]:
    schema_file = files("fairspec_table.schemas").joinpath("topojson.json")
    return json.loads(schema_file.read_text(encoding="utf-8"))
