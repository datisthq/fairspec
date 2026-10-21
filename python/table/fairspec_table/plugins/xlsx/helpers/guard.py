from __future__ import annotations

import codecs
import re
import zipfile
from collections.abc import Callable

MAX_SHEET_CELLS = 50_000_000


def assert_sheet_size(path: str) -> None:
    """Raise when the workbook's sheets span more cells than a load can hold.

    calamine allocates every cell between the first and the last used one before
    returning, so a tiny file naming a far cell would abort the process.
    """
    with open(path, "rb") as file:
        if file.read(4) != ZIP_MAGIC:
            return

    total = 0

    def check_cells(cells: int) -> None:
        if total + cells > MAX_SHEET_CELLS:
            raise Exception(
                f"Sheet is too large to load: it spans more than {MAX_SHEET_CELLS} cells"
            )

    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
        for name in names:
            if _XLSX_SHEET.match(name):
                scanner = _XlsxScanner(check_cells)
            elif name == "content.xml":
                scanner = _OdsScanner(check_cells)
            else:
                continue

            decoder = codecs.getincrementaldecoder("utf-8")(errors="replace")
            with archive.open(name) as stream:
                while chunk := stream.read(1 << 20):
                    scanner.push(decoder.decode(chunk))
                scanner.push(decoder.decode(b"", final=True), final=True)
            total += scanner.finish()

        if "xl/workbook.bin" in names:
            raise Exception("Spreadsheet format xlsb is not supported")


ZIP_MAGIC = b"PK\x03\x04"
_XLSX_SHEET = re.compile(r"^xl/worksheets/[^/]+\.xml$")
_MAX_TAG_LENGTH = 1 << 24

# Quoted attribute values may hold ">", which must not end the tag early
_ATTRS = r"""([^>"']*(?:(?:"[^"]*"|'[^']*')[^>"']*)*)"""
_XLSX_TAG = re.compile(rf"<(?:[\w.-]+:)?(c|row)(?=[\s/>]){_ATTRS}>")
_ODS_TAG = re.compile(
    rf"<(/?)(?:[\w.-]+:)?(table-row|table-cell|covered-table-cell|table)(?=[\s/>]){_ATTRS}>"
)
_DIGITS = "0123456789"
_ATTRIBUTES: dict[str, re.Pattern[str]] = {}


class _Bounds:
    def __init__(self, check_cells: Callable[[int], None]) -> None:
        self.check_cells = check_cells
        self.min_row = self.min_col = float("inf")
        self.max_row = self.max_col = 0

    def count(self) -> int:
        if not self.max_row:
            return 0
        rows = self.max_row - int(self.min_row) + 1
        return rows * (self.max_col - int(self.min_col) + 1)

    def add(self, from_row: int, to_row: int, from_col: int, to_col: int) -> None:
        if (
            from_row >= self.min_row
            and to_row <= self.max_row
            and from_col >= self.min_col
            and to_col <= self.max_col
        ):
            return
        self.min_row = min(self.min_row, from_row)
        self.max_row = max(self.max_row, to_row)
        self.min_col = min(self.min_col, from_col)
        self.max_col = max(self.max_col, to_col)
        self.check_cells(self.count())


class _TagReader:
    def __init__(self, pattern: re.Pattern[str]) -> None:
        self.pattern = pattern
        self.rest = ""

    def push(self, text: str, *, final: bool = False) -> None:
        data = self.rest + text

        # A tag cannot hold "<", so only the text from the last one can be incomplete
        cut = len(data) if final else max(data.rfind("<"), 0)
        self.rest = data[cut:]
        if len(self.rest) > _MAX_TAG_LENGTH:
            raise Exception("Spreadsheet markup is malformed")

        self.on_tags(self.pattern.findall(data, 0, cut))

    def on_tags(self, tags: list[tuple[str, ...]]) -> None:
        raise NotImplementedError

    def finish(self) -> int:
        raise NotImplementedError


class _XlsxScanner(_TagReader):
    def __init__(self, check_cells: Callable[[int], None]) -> None:
        super().__init__(_XLSX_TAG)
        self.bounds = _Bounds(check_cells)
        self.columns: dict[str, int] = {}
        self.row = 0
        self.col = 0

    # One loop over plain tuples: a call per tag would triple the scan
    def on_tags(self, tags: list[tuple[str, ...]]) -> None:
        bounds = self.bounds
        columns = self.columns
        row = self.row
        col = self.col
        for name, attrs in tags:
            if attrs.startswith(' r="'):
                ref = attrs[4 : attrs.index('"', 4)]
            else:
                ref = _get_attribute(attrs, "r")

            if name == "row":
                row = _to_number(ref) or row + 1
                col = 0
                continue

            letters = ref.rstrip(_DIGITS) if ref else ""
            digits = ref[len(letters) :] if ref else ""
            if letters.isalpha() and letters.isascii() and digits.isdigit():
                col = columns.get(letters) or _parse_column_letters(letters)
                columns[letters] = col
                row = int(digits)
            else:
                col += 1

            if attrs[-1:] != "/" and not (
                bounds.min_row <= row <= bounds.max_row
                and bounds.min_col <= col <= bounds.max_col
            ):
                bounds.add(row, row, col, col)

        self.row = row
        self.col = col

    def finish(self) -> int:
        return self.bounds.count()


class _OdsScanner(_TagReader):
    def __init__(self, check_cells: Callable[[int], None]) -> None:
        super().__init__(_ODS_TAG)
        self.check_cells = check_cells
        self.previous = 0
        self.bounds = _Bounds(check_cells)
        self.row = 0
        self.col = 0
        self.row_repeat = 1
        self.row_min_col = float("inf")
        self.row_max_col = 0

    def on_tags(self, tags: list[tuple[str, ...]]) -> None:
        for tag in tags:
            self.on_tag(*tag)

    def on_tag(self, closing: str, name: str, attrs: str) -> None:
        is_closing = closing == "/"
        is_empty = attrs.endswith("/")

        if name == "table" and not is_closing:
            self.previous += self.bounds.count()
            previous = self.previous
            self.bounds = _Bounds(lambda cells: self.check_cells(previous + cells))
            self.row = 0
            return

        if name == "table-row":
            if not is_closing:
                self.row_repeat = _get_repeat(attrs, "number-rows-repeated")
                self.col = 0
            if is_closing or is_empty:
                if self.row_max_col:
                    self.bounds.add(
                        self.row + 1,
                        self.row + self.row_repeat,
                        int(self.row_min_col),
                        self.row_max_col,
                    )
                self.row += self.row_repeat
                self.row_min_col = float("inf")
                self.row_max_col = 0
            return

        if name != "table" and not is_closing:
            start = self.col + 1
            self.col += _get_repeat(attrs, "number-columns-repeated")
            if not is_empty or _get_attribute(attrs, "value-type") is not None:
                self.row_min_col = min(self.row_min_col, start)
                self.row_max_col = max(self.row_max_col, self.col)

    def finish(self) -> int:
        return self.previous + self.bounds.count()


def _get_attribute(attrs: str, name: str) -> str | None:
    quick = f' {name}="'
    if attrs.startswith(quick):
        return attrs[len(quick) : attrs.index('"', len(quick))]

    pattern = _ATTRIBUTES.get(name)
    if pattern is None:
        pattern = re.compile(
            rf"""(?:^|\s)(?:[\w.-]+:)?{name}\s*=\s*(?:"([^"]*)"|'([^']*)')"""
        )
        _ATTRIBUTES[name] = pattern

    match = pattern.search(attrs)
    if not match:
        return None
    return match.group(1) if match.group(1) is not None else match.group(2)


def _get_repeat(attrs: str, name: str) -> int:
    return max(1, _to_number(_get_attribute(attrs, name)) or 1)


def _to_number(value: str | None) -> int:
    return int(value) if value and value.isdigit() else 0


def _parse_column_letters(letters: str) -> int:
    col = 0
    for letter in letters.upper():
        col = col * 26 + ord(letter) - 64
    return col
