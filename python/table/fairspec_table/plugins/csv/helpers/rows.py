from __future__ import annotations

import re
from dataclasses import dataclass
from typing import TYPE_CHECKING

from fairspec_dataset import get_temp_file_path
from fairspec_metadata import CsvFileDialect

from fairspec_table.helpers.file_dialect import get_header_rows
from fairspec_table.plugins.csv.helpers.delimiter import read_file_chunks

if TYPE_CHECKING:
    from typing import BinaryIO

    from fairspec_metadata import TsvFileDialect

CR = 0x0D
LF = 0x0A
BOM = b"\xef\xbb\xbf"


@dataclass
class CleanedRows:
    paths: list[str]
    file_dialect: CsvFileDialect | TsvFileDialect


@dataclass
class RowScanner:
    eol: int
    pattern: re.Pattern[bytes]
    delimiter: int
    quote: int | None
    null_sequence: bytes | None
    last_header_row: int


def clean_csv_rows(
    paths: list[str],
    file_dialect: CsvFileDialect | TsvFileDialect,
    *,
    max_bytes: int | None = None,
) -> CleanedRows | None:
    """Polars reads a blank line as a row of nulls and a quoted `nullSequence` as null,
    so files holding a blank line, or read with a `nullSequence`, are copied with blank
    lines dropped and unquoted sequences emptied; the dialect returned has none
    """
    eol = (file_dialect.lineTerminator or "\n").encode()[-1]
    null_sequence = file_dialect.nullSequence
    if null_sequence is None and not any(
        _get_has_blank_line(path, eol, max_bytes) for path in paths
    ):
        return None

    delimiter, quote = "\t", None
    if isinstance(file_dialect, CsvFileDialect):
        delimiter, quote = file_dialect.delimiter or ",", file_dialect.quoteChar or '"'
    scanner = RowScanner(
        eol=eol,
        pattern=_create_row_pattern(
            eol,
            delimiter.encode()[0],
            quote.encode()[0] if quote else None,
            (file_dialect.commentPrefix or "").encode(),
        ),
        delimiter=delimiter.encode()[0],
        quote=quote.encode()[0] if quote else None,
        null_sequence=null_sequence.encode() if null_sequence is not None else None,
        last_header_row=max([0, *get_header_rows(file_dialect)]),
    )

    return CleanedRows(
        paths=[_clean_file_rows(path, scanner, max_bytes) for path in paths],
        file_dialect=file_dialect.model_copy(update={"nullSequence": None}),
    )


# A blank line is two line ends with nothing (or only a CR) between them, or one at
# the start of the file after any BOM; the copy drops it unless inside a quoted cell
def _get_has_blank_line(path: str, eol: int, max_bytes: int | None) -> bool:
    needles = [bytes([eol, eol])]
    if eol == LF:
        needles.append(bytes([LF, CR, LF]))

    carry = b""
    is_start = True
    for chunk in read_file_chunks(path, max_bytes):
        data = carry + chunk
        if is_start:
            start = len(BOM) if data.startswith(BOM) else 0
            first = data[start : start + 2]
            if first[:1] == bytes([eol]) or (eol == LF and first == b"\r\n"):
                return True
            is_start = len(data) < start + 2
        if any(needle in data for needle in needles):
            return True
        carry = data[-2:]
    return False


# A row is a record of the file: a line end inside a quoted cell does not end it. A
# quote opens a cell only at its start, and a doubled quote inside one is a quote
def _create_row_pattern(
    eol: int, delimiter: int, quote: int | None, comment_prefix: bytes
) -> re.Pattern[bytes]:
    e, d = re.escape(bytes([eol])), re.escape(bytes([delimiter]))
    if quote is None:
        cell = rb"[^" + d + e + rb"]*"
    else:
        q = re.escape(bytes([quote]))
        cell = (
            q + rb"(?:[^" + q + rb"]|" + q + q + rb")*" + q + rb"[^" + d + e + rb"]*"
            rb"|(?:[^" + q + d + e + rb"][^" + d + e + rb"]*)?"
        )
    row = rb"(?:" + cell + rb")(?:" + d + rb"(?:" + cell + rb"))*"
    if comment_prefix:
        row = rb"(?P<comment>" + re.escape(comment_prefix) + rb"[^" + e + rb"]*)|" + row
    return re.compile(rb"(?:" + row + rb")(?P<eol>" + e + rb")?", re.DOTALL)


def _clean_file_rows(path: str, scanner: RowScanner, max_bytes: int | None) -> str:
    target = get_temp_file_path(format="csv")
    row_number = 0
    carry = b""
    pending: list[bytes] = []
    pending_size = 0
    is_start = True
    with open(target, "wb") as output:
        for chunk in read_file_chunks(path, max_bytes):
            # The copy drops a BOM, so a quote or a line end right after it reads as at
            # the start
            if is_start and chunk.startswith(BOM):
                chunk = chunk[len(BOM) :]
            is_start = False
            # An unfinished row is matched again only once as many new bytes arrived, or
            # a long quoted cell would be rescanned for every chunk (quadratic)
            pending.append(chunk)
            pending_size += len(chunk)
            if pending_size < len(carry):
                continue
            data = carry + b"".join(pending)
            pending, pending_size = [], 0
            position = 0
            while True:
                match = scanner.pattern.match(data, position)
                if not match or not match.group("eol"):
                    break
                row_number = _write_row(output, match, row_number, scanner)
                position = match.end()
            carry = data[position:]
        carry += b"".join(pending)
        position = 0
        while position < len(carry):
            match = scanner.pattern.match(carry, position)
            if not match or match.end() == position:
                output.write(carry[position:])
                break
            row_number = _write_row(output, match, row_number, scanner)
            position = match.end()
    return target


def _write_row(
    output: BinaryIO, match: re.Match[bytes], row_number: int, scanner: RowScanner
) -> int:
    is_terminated = match.group("eol") is not None
    row = match.group(0)[:-1] if is_terminated else match.group(0)
    content = row[:-1] if scanner.eol == LF and row.endswith(b"\r") else row
    if not content:
        return row_number

    row_number += 1
    is_comment = "comment" in match.re.groupindex and match.group("comment") is not None
    is_data = row_number > scanner.last_header_row and not is_comment
    if scanner.null_sequence is not None and is_data:
        row = _empty_null_cells(row, scanner)
    output.write(row + (bytes([scanner.eol]) if is_terminated else b""))
    return row_number


def _empty_null_cells(row: bytes, scanner: RowScanner) -> bytes:
    delimiter = bytes([scanner.delimiter])
    cells: list[bytes] = []
    start = 0
    is_quoted = False
    is_start = True
    is_closed = False
    for position, byte in enumerate(row):
        if is_quoted:
            if byte == scanner.quote:
                is_quoted = False
                is_closed = True
            continue
        if byte == scanner.delimiter:
            cells.append(row[start:position])
            start = position + 1
            is_start, is_closed = True, False
            continue
        if (
            scanner.quote is not None
            and byte == scanner.quote
            and (is_start or is_closed)
        ):
            is_quoted = True
        is_start = is_closed = False
    cells.append(row[start:])

    def empty_cell(cell: bytes) -> bytes:
        text = cell[:-1] if scanner.eol == LF and cell.endswith(b"\r") else cell
        return cell[len(text) :] if text == scanner.null_sequence else cell

    return delimiter.join(empty_cell(cell) for cell in cells)
