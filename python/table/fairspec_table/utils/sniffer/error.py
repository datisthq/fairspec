from __future__ import annotations


class SnifferError(Exception):
    pass


class EncodingError(SnifferError):
    pass


class ParseError(SnifferError):
    pass
