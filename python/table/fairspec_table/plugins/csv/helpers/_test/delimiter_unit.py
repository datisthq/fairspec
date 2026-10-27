from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import CsvFileDialect

from ..delimiter import CHUNK_SIZE, DelimiterSwap, swap_delimiter


class TestSwapDelimiter:
    def test_should_keep_a_single_byte_delimiter(self):
        path = write_temp_file("a;b\n")
        assert swap_delimiter([path], CsvFileDialect(delimiter=";")) is None

    def test_should_swap_a_delimiter_cut_by_a_chunk_boundary(self):
        text = f"{'a' * (CHUNK_SIZE - 1)}¦b\n"
        path = write_temp_file(text)
        swapped = swap_delimiter([path], CsvFileDialect(delimiter="¦"))

        assert swapped is not None
        assert swapped.swap == DelimiterSwap(delimiter="¦", placeholder="\x1f")
        with open(swapped.paths[0], encoding="utf-8", newline="") as file:
            assert file.read() == text.replace("¦", "\x1f")

    def test_should_take_a_control_byte_the_file_does_not_hold(self):
        path = write_temp_file("a¦\x1f\n")
        swapped = swap_delimiter([path], CsvFileDialect(delimiter="¦"))

        assert swapped is not None
        assert swapped.swap.placeholder == "\x1e"

    def test_should_read_no_further_than_max_bytes(self):
        path = write_temp_file("a¦b\n\x1f¦c\n")
        swapped = swap_delimiter([path], CsvFileDialect(delimiter="¦"), max_bytes=5)

        assert swapped is not None
        assert swapped.swap.placeholder == "\x1f"
        with open(swapped.paths[0], encoding="utf-8", newline="") as file:
            assert file.read() == "a\x1fb\n"
