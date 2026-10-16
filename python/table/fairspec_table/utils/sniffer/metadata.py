from __future__ import annotations

from enum import StrEnum

from fairspec_metadata.models.base import FairspecModel


class LineTerminator(StrEnum):
    LF = "LF"
    CRLF = "CRLF"
    CR = "CR"


class Quote(FairspecModel):
    char: int | None = None


class Header(FairspecModel):
    has_header_row: bool
    num_preamble_rows: int


class Dialect(FairspecModel):
    delimiter: int
    header: Header
    quote: Quote
    flexible: bool
    is_utf8: bool
    line_terminator: LineTerminator


class Metadata(FairspecModel):
    dialect: Dialect
    avg_record_len: float
    num_fields: int
    fields: list[str]
