from __future__ import annotations

import re

from fairspec_metadata import CellError, WktColumn

from fairspec_table.models import ColumnMapping
from fairspec_table.models.table import Table

from .._shared.helpers import inspect_text_column


def inspect_wkt_column(
    column: WktColumn, table: Table, mapping: ColumnMapping | None = None
) -> list[CellError]:
    # imported here: shapely pulls in numpy, about 50ms on every CLI start
    import shapely

    def parse_wkt(source: str) -> object:
        if not _get_are_words_known(source):
            return None
        return shapely.from_wkt(source)

    return inspect_text_column(column, table, mapping=mapping, parse=parse_wkt)


_KINDS = [
    "POINT",
    "LINESTRING",
    "LINEARRING",
    "POLYGON",
    "MULTIPOINT",
    "MULTILINESTRING",
    "MULTIPOLYGON",
    "GEOMETRYCOLLECTION",
]
_SUFFIXES = ["Z", "M", "ZM"]
_KNOWN_WORDS = {
    *_KINDS,
    *_SUFFIXES,
    "EMPTY",
    *(kind + suffix for kind in _KINDS for suffix in _SUFFIXES),
}

_WORD_REGEX = re.compile(r"[^\n\r\t() ,]+")
_NUMBER_REGEX = re.compile(
    r"[+-]?(?:"
    r"(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:e[+-]?[0-9]+)?"
    r"|0x(?:[0-9a-f]+(?:\.[0-9a-f]*)?|\.[0-9a-f]+)(?:p[+-]?[0-9]+)?"
    r"|inf|infinity|nan(?:\([0-9a-z_]*\))?"
    r")",
    re.IGNORECASE,
)


def _get_are_words_known(text: str) -> bool:
    """Whether every word is a number or a WKT keyword, as the TypeScript and Rust
    readers tokenize; GEOS takes a type by prefix, so reads `POINTX (1 2)` as a point"""
    for match in _WORD_REGEX.finditer(text.split("\0")[0]):
        word = match.group().lstrip(" \t\n\r\f\v")
        if word and not _NUMBER_REGEX.fullmatch(word):
            if word.upper() not in _KNOWN_WORDS:
                return False
    return True
