from __future__ import annotations

from typing import Union

from .builtin import BuiltinFileDialect
from .custom import CustomFileDialect

FileDialect = Union[BuiltinFileDialect, CustomFileDialect]
