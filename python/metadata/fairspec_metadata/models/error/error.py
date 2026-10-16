from __future__ import annotations

from typing import Union

from .data import DataError
from .file import FileError
from .metadata import MetadataError
from .resource import ResourceError
from .table import TableError

FairspecError = Union[MetadataError, ResourceError, TableError, DataError, FileError]
