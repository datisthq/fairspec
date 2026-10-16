from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .metadata import LineTerminator, Quote

DELIMITERS: list[int] = [
    44,  # , (comma)
    59,  # ; (semicolon)
    9,  # \t (tab)
    124,  # | (pipe)
    94,  # ^ (caret)
    126,  # ~ (tilde)
    35,  # # (hash)
    38,  # & (ampersand)
    167,  # § (section)
    47,  # / (slash)
]

QUOTE_CHARS: list[Quote] = [
    Quote(char=None),
    Quote(char=34),  # " (double quote)
    Quote(char=39),  # ' (single quote)
]


class PotentialDialect(FairspecModel):
    delimiter: int
    quote: Quote
    line_terminator: LineTerminator


def detect_line_terminator(data: bytes) -> LineTerminator:
    has_cr = False
    has_lf = False
    has_crlf = False

    i = 0
    while i < len(data):
        if data[i] == 13:
            if i + 1 < len(data) and data[i + 1] == 10:
                has_crlf = True
                i += 1
            else:
                has_cr = True
        elif data[i] == 10:
            has_lf = True
        i += 1

    if has_crlf:
        return LineTerminator.CRLF
    if has_lf:
        return LineTerminator.LF
    if has_cr:
        return LineTerminator.CR

    return LineTerminator.LF


def generate_potential_dialects(
    line_terminator: LineTerminator,
) -> list[PotentialDialect]:
    dialects: list[PotentialDialect] = []

    for delimiter in DELIMITERS:
        for quote in QUOTE_CHARS:
            dialects.append(
                PotentialDialect(
                    delimiter=delimiter,
                    quote=quote,
                    line_terminator=line_terminator,
                )
            )

    return dialects
