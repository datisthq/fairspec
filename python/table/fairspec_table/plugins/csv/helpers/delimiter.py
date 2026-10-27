from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

import polars as pl

from fairspec_dataset import get_temp_file_path
from fairspec_metadata import CsvFileDialect

if TYPE_CHECKING:
    from collections.abc import Iterator

    from fairspec_metadata import TsvFileDialect
    from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

    from fairspec_table.models.table import Table

# Control bytes a text file almost never holds; tab, LF and CR are left out
PLACEHOLDER_BYTES = [
    *range(31, 13, -1),
    *(12, 11, 8, 7, 6, 5, 4, 3, 2, 1),
]
CHUNK_SIZE = 65536


@dataclass
class DelimiterSwap:
    delimiter: str
    placeholder: str


@dataclass
class SwappedDelimiter:
    swap: DelimiterSwap
    paths: list[str]
    file_dialect: CsvFileDialect


def swap_delimiter(
    paths: list[str],
    file_dialect: CsvFileDialect | TsvFileDialect | FileDialect | None,
    *,
    max_bytes: int | None = None,
) -> SwappedDelimiter | None:
    """Polars splits cells on one byte, so a multi-byte delimiter is swapped in copies
    of the files for a control byte none of them holds; `restore_delimiter` swaps it back
    """
    if not isinstance(file_dialect, CsvFileDialect):
        return None
    delimiter = file_dialect.delimiter
    if not delimiter or len(delimiter.encode()) == 1:
        return None

    reserved = "".join(
        text or ""
        for text in (
            file_dialect.quoteChar,
            file_dialect.lineTerminator,
            file_dialect.nullSequence,
            file_dialect.commentPrefix,
        )
    ).encode()
    placeholder = _find_absent_byte(
        paths, [byte for byte in PLACEHOLDER_BYTES if byte not in reserved], max_bytes
    )
    if placeholder is None:
        raise Exception(
            f'Delimiter "{delimiter}" cannot be read: the file holds every control byte'
        )

    swap = DelimiterSwap(delimiter=delimiter, placeholder=chr(placeholder))

    def swap_text(text: str | None) -> str | None:
        return text.replace(delimiter, swap.placeholder) if text is not None else None

    return SwappedDelimiter(
        swap=swap,
        paths=[
            _swap_file_delimiter(path, delimiter, placeholder, max_bytes)
            for path in paths
        ],
        file_dialect=file_dialect.model_copy(
            update={
                "delimiter": swap.placeholder,
                "nullSequence": swap_text(file_dialect.nullSequence),
                "commentPrefix": swap_text(file_dialect.commentPrefix),
            }
        ),
    )


def restore_delimiter(table: Table, swap: DelimiterSwap) -> Table:
    """Puts the multi-byte delimiter back in every cell and column name"""
    return table.select(
        pl.col(name)
        .str.replace_all(swap.placeholder, swap.delimiter, literal=True)
        .alias(name.replace(swap.placeholder, swap.delimiter))
        for name in table.collect_schema().names()
    )


def _find_absent_byte(
    paths: list[str], candidates: list[int], max_bytes: int | None
) -> int | None:
    for byte in candidates:
        needle = bytes([byte])
        if not any(_get_is_byte_present(path, needle, max_bytes) for path in paths):
            return byte
    return None


def _get_is_byte_present(path: str, needle: bytes, max_bytes: int | None) -> bool:
    return any(needle in chunk for chunk in _read_chunks(path, max_bytes))


def _swap_file_delimiter(
    path: str, delimiter: str, placeholder: int, max_bytes: int | None
) -> str:
    needle = delimiter.encode()
    replacement = bytes([placeholder])
    target = get_temp_file_path(format="csv")
    carry = b""
    with open(target, "wb") as output:
        for chunk in _read_chunks(path, max_bytes):
            data = carry + chunk
            # A delimiter cut by the chunk boundary waits for the next chunk
            cut = len(data)
            for size in range(len(needle) - 1, 0, -1):
                if data.endswith(needle[:size]):
                    cut = len(data) - size
                    break
            carry = data[cut:]
            output.write(data[:cut].replace(needle, replacement))
        output.write(carry)
    return target


# A preview reads only its first bytes, so neither pass reads past them
def _read_chunks(path: str, max_bytes: int | None) -> Iterator[bytes]:
    read = 0
    with open(path, "rb") as file:
        while chunk := file.read(
            min(CHUNK_SIZE, max_bytes - read) if max_bytes else CHUNK_SIZE
        ):
            read += len(chunk)
            yield chunk
