from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_dataset import load_file_stream
from fairspec_metadata import get_data_first_path
from fairspec_metadata.models.file_dialect.csv import CsvFileDialect
from fairspec_metadata.models.file_dialect.tsv import TsvFileDialect

from fairspec_table.utils.sniffer.sniffer import Sniffer

if TYPE_CHECKING:
    from fairspec_metadata import Resource

    from fairspec_table.models.table import LoadTableOptions


def infer_csv_file_dialect(
    resource: Resource,
    options: LoadTableOptions | None = None,
) -> CsvFileDialect | TsvFileDialect | None:
    sample_bytes = 10_000

    data_path = get_data_first_path(resource)
    if not data_path:
        return None

    stream = load_file_stream(data_path, max_bytes=sample_bytes)
    raw_bytes = stream.read()

    sniffer = Sniffer()
    try:
        result = sniffer.sniff_bytes(raw_bytes)
    except Exception:
        return CsvFileDialect()

    lt = result.dialect.line_terminator
    line_terminator = "\n" if lt == "LF" else "\r\n" if lt == "CRLF" else "\r"

    is_tsv = result.dialect.delimiter == 9

    if is_tsv:
        kwargs: dict[str, object] = {"lineTerminator": line_terminator}

        if result.dialect.header.has_header_row:
            kwargs["headerRows"] = [result.dialect.header.num_preamble_rows + 1]
        elif result.num_fields > 0:
            kwargs["headerRows"] = False

        return TsvFileDialect(**kwargs)  # type: ignore[arg-type]

    kwargs = {
        "delimiter": chr(result.dialect.delimiter),
        "lineTerminator": line_terminator,
    }

    if result.dialect.quote.char is not None:
        kwargs["quoteChar"] = chr(result.dialect.quote.char)

    if result.dialect.header.has_header_row:
        kwargs["headerRows"] = [result.dialect.header.num_preamble_rows + 1]
    elif result.num_fields > 0:
        kwargs["headerRows"] = False

    return CsvFileDialect(**kwargs)  # type: ignore[arg-type]
