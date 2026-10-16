from __future__ import annotations

from typing import Union

from .cell import CellError
from .column import ColumnError
from .foreign_key import ForeignKeyError
from .row import RowError

TableError = Union[ColumnError, RowError, CellError, ForeignKeyError]
