from __future__ import annotations

import re

from fairspec_metadata import CellError, DurationColumn

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table
from fairspec_table.settings import DURATION_REGEX

from .._shared.helpers import inspect_text_column

DURATION_PATTERN = re.compile(DURATION_REGEX)


def inspect_duration_column(
    column: DurationColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    return inspect_text_column(column, table, mapping=mapping, parse=_parse_duration)


def _parse_duration(text: str) -> object:
    # `fullmatch`, since `$` also matches before a final newline
    return text if DURATION_PATTERN.fullmatch(text) else None
