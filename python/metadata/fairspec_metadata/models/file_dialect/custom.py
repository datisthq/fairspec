from __future__ import annotations

from pydantic import field_validator

from .base import BaseFileDialect
from .builtin import BUILTIN_FILE_DIALECT_FORMATS


class CustomFileDialect(BaseFileDialect):
    format: str

    @field_validator("format")
    @classmethod
    def check_format(cls, value: str) -> str:
        if value in BUILTIN_FILE_DIALECT_FORMATS:
            raise ValueError(f"'{value}' is a built-in format")
        return value
