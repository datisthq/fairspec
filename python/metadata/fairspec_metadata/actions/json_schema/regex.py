# A port of jsonschema-regex's is_valid_ecma_regex, which the Rust twin calls: keep the two
# in step rather than switching to `re`, which rejects `(?<name>…)` and `\p{L}`
from __future__ import annotations

import unicodedata
from dataclasses import dataclass, field
from typing import Literal

Previous = Literal["nothing", "atom", "assertion", "quantifier"]
GroupKind = Literal["capturing", "nonCapturing", "lookaround"]


def is_valid_ecma_regex(pattern: str) -> bool:
    """Whether a pattern is a valid ECMA-262 regular expression under the `u` flag, widened with escapes of non-alphanumeric characters"""
    try:
        _Parser(pattern).parse()
    except _RegexSyntaxError:
        return False
    return True


_U64_MAX = 2**64 - 1


class _RegexSyntaxError(Exception):
    pass


@dataclass
class _Frame:
    kind: GroupKind
    current: list[str] = field(default_factory=list)
    finished: list[str] = field(default_factory=list)


class _Parser:
    def __init__(self, pattern: str) -> None:
        self.chars = pattern
        self.position = 0
        self.previous: Previous = "nothing"
        self.frames = [_Frame("nonCapturing")]
        self.capturing_groups = 0
        self.largest_backreference = 0
        self.referenced_names: list[str] = []
        self.declared_names: list[str] = []

    def parse(self) -> None:
        while self.peek() is not None:
            c = self.bump()
            if c == "|":
                frame = self.frames[-1]
                frame.finished.extend(frame.current)
                frame.current.clear()
                self.previous = "nothing"
            elif c == "(":
                self.open_group()
                self.previous = "nothing"
            elif c == ")":
                self.previous = self.close_group()
            elif c in "*+?":
                self.quantify()
            elif c == "{":
                self.parse_braced_quantifier()
                self.quantify()
            elif c in "}]":
                raise _RegexSyntaxError
            elif c in "^$":
                self.previous = "assertion"
            elif c == "\\":
                self.previous = self.parse_atom_escape()
            elif c == "[":
                self.parse_class()
                self.previous = "atom"
            else:
                self.previous = "atom"
        if len(self.frames) != 1 or self.largest_backreference > self.capturing_groups:
            raise _RegexSyntaxError
        if any(name not in self.declared_names for name in self.referenced_names):
            raise _RegexSyntaxError

    def peek(self, offset: int = 0) -> str | None:
        index = self.position + offset
        return self.chars[index] if index < len(self.chars) else None

    def bump(self) -> str:
        c = self.peek()
        if c is None:
            raise _RegexSyntaxError
        self.position += 1
        return c

    def eat(self, expected: str) -> bool:
        if self.peek() != expected:
            return False
        self.position += 1
        return True

    def expect(self, expected: str) -> None:
        if not self.eat(expected):
            raise _RegexSyntaxError

    def quantify(self) -> None:
        if self.previous != "atom":
            raise _RegexSyntaxError
        self.eat("?")
        self.previous = "quantifier"

    def parse_braced_quantifier(self) -> None:
        lower = self.parse_decimal_digits()
        if self.eat(",") and self.peek() != "}":
            upper = self.parse_decimal_digits()
            if _compare_decimal(lower, upper) > 0:
                raise _RegexSyntaxError
        self.expect("}")

    def parse_decimal_digits(self) -> str:
        start = self.position
        while _is_ascii_digit(self.peek()):
            self.position += 1
        if self.position == start:
            raise _RegexSyntaxError
        return self.chars[start : self.position]

    def open_group(self) -> None:
        kind: GroupKind = "capturing"
        if self.eat("?"):
            c = self.bump()
            if c == ":":
                kind = "nonCapturing"
            elif c in "=!":
                kind = "lookaround"
            elif c == "<":
                if self.peek() in ("=", "!"):
                    self.position += 1
                    kind = "lookaround"
                else:
                    self.declare_name(self.parse_group_name())
                    self.capturing_groups += 1
            else:
                self.parse_modifiers(c)
                kind = "nonCapturing"
        else:
            self.capturing_groups += 1
        self.frames.append(_Frame(kind))

    def parse_modifiers(self, first: str) -> None:
        seen: set[str] = set()
        dashes = 0
        c = first
        while c != ":":
            if c in ("i", "m", "s"):
                if c in seen:
                    raise _RegexSyntaxError
                seen.add(c)
            elif c == "-" and dashes == 0:
                dashes = 1
            else:
                raise _RegexSyntaxError
            c = self.bump()
        if not seen:
            raise _RegexSyntaxError

    def close_group(self) -> Previous:
        if len(self.frames) == 1:
            raise _RegexSyntaxError
        frame = self.frames.pop()
        self.frames[-1].current.extend([*frame.current, *frame.finished])
        return "assertion" if frame.kind == "lookaround" else "atom"

    def declare_name(self, name: str) -> None:
        if any(name in frame.current for frame in self.frames):
            raise _RegexSyntaxError
        self.declared_names.append(name)
        self.frames[-1].current.append(name)

    def parse_group_name(self) -> str:
        name = ""
        first = True
        while True:
            c = self.bump()
            if c == ">":
                break
            if c == "\\":
                self.expect("u")
                code_point = self.parse_unicode_escape()
                if 0xD800 <= code_point <= 0xDFFF:
                    raise _RegexSyntaxError
                c = chr(code_point)
            allowed = _is_identifier_start(c) if first else _is_identifier_part(c)
            if not allowed:
                raise _RegexSyntaxError
            name += c
            first = False
        if first:
            raise _RegexSyntaxError
        return name

    def parse_atom_escape(self) -> Previous:
        c = self.bump()
        if c in ("b", "B"):
            return "assertion"
        if c in "dDsSwW":
            return "atom"
        if c in ("p", "P"):
            self.parse_unicode_property()
        elif c == "k":
            self.expect("<")
            self.referenced_names.append(self.parse_group_name())
        elif "1" <= c <= "9":
            self.position -= 1
            number = _parse_backreference_number(self.parse_decimal_digits())
            self.largest_backreference = max(self.largest_backreference, number)
        else:
            self.parse_character_escape(c)
        return "atom"

    def parse_character_escape(self, c: str) -> int:
        simple = {"f": 0x0C, "n": 0x0A, "r": 0x0D, "t": 0x09, "v": 0x0B}
        if c in simple:
            return simple[c]
        if c == "c":
            letter = self.bump()
            if not (letter.isascii() and letter.isalpha()):
                raise _RegexSyntaxError
            return ord(letter) % 32
        if c == "0":
            if _is_ascii_digit(self.peek()):
                raise _RegexSyntaxError
            return 0
        if c == "x":
            return self.parse_hex_digit() * 16 + self.parse_hex_digit()
        if c == "u":
            return self.parse_unicode_escape()
        if not _is_alphanumeric(c):
            return ord(c)
        raise _RegexSyntaxError

    def parse_hex_digit(self) -> int:
        c = self.bump()
        if c not in "0123456789abcdefABCDEF":
            raise _RegexSyntaxError
        return int(c, 16)

    def parse_unicode_escape(self) -> int:
        if self.eat("{"):
            value = 0
            digits = 0
            while not self.eat("}"):
                value = min(value * 16 + self.parse_hex_digit(), 0x110000)
                digits += 1
            if digits == 0 or value > 0x10FFFF:
                raise _RegexSyntaxError
            return value
        lead = self.parse_hex4()
        if 0xD800 <= lead <= 0xDBFF and self.peek() == "\\" and self.peek(1) == "u":
            position = self.position
            self.position += 2
            try:
                trail = self.parse_hex4()
                if 0xDC00 <= trail <= 0xDFFF:
                    return 0x10000 + ((lead - 0xD800) << 10) + (trail - 0xDC00)
            except _RegexSyntaxError:
                pass
            self.position = position
        return lead

    def parse_hex4(self) -> int:
        value = 0
        for _ in range(4):
            value = value * 16 + self.parse_hex_digit()
        return value

    def parse_unicode_property(self) -> None:
        self.expect("{")
        self.parse_property_word()
        if self.eat("="):
            self.parse_property_word()
        self.expect("}")

    def parse_property_word(self) -> None:
        start = self.position
        while (c := self.peek()) is not None and (
            c.isascii() and (c.isalnum() or c == "_")
        ):
            self.position += 1
        if self.position == start:
            raise _RegexSyntaxError

    def parse_class(self) -> None:
        self.eat("^")
        while True:
            if self.eat("]"):
                return
            start = self.parse_class_atom()
            if self.peek() == "-" and self.peek(1) != "]":
                self.position += 1
                end = self.parse_class_atom()
                if start is None or end is None or start > end:
                    raise _RegexSyntaxError

    def parse_class_atom(self) -> int | None:
        c = self.bump()
        if c != "\\":
            return ord(c)
        escaped = self.bump()
        if escaped == "b":
            return 0x08
        if escaped == "-":
            return ord("-")
        if escaped in "dDsSwW":
            return None
        if escaped in ("p", "P"):
            self.parse_unicode_property()
            return None
        return self.parse_character_escape(escaped)


def _is_ascii_digit(c: str | None) -> bool:
    return c is not None and "0" <= c <= "9"


def _compare_decimal(left: str, right: str) -> int:
    a = left.lstrip("0")
    b = right.lstrip("0")
    if len(a) != len(b):
        return len(a) - len(b)
    return (a > b) - (a < b)


def _parse_backreference_number(digits: str) -> int:
    # int() refuses strings past 4300 digits; saturate like the Rust twin's u64 parse
    return min(int(digits), _U64_MAX) if len(digits) <= 20 else _U64_MAX


def _is_alphabetic(c: str) -> bool:
    return c.isalpha() or unicodedata.category(c) == "Nl"


def _is_numeric(c: str) -> bool:
    return unicodedata.category(c) in ("Nd", "Nl", "No")


def _is_alphanumeric(c: str) -> bool:
    return _is_alphabetic(c) or _is_numeric(c)


def _is_identifier_start(c: str) -> bool:
    return _is_alphabetic(c) or c in "$_"


def _is_identifier_part(c: str) -> bool:
    return _is_alphanumeric(c) or c in "$_‌‍"
