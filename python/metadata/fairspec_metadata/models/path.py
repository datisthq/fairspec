from __future__ import annotations

from typing import Annotated

from pydantic import Field

InternalPath = Annotated[
    str,
    Field(
        pattern=r"^(?:[^/\\.~A-Za-z][^/\\]*|[A-Za-z][^/\\:]*|\.[^/\\.][^/\\]*|\.\.[^/\\]+|\.)(?:/(?:[^/\\.][^/\\]*|\.[^/\\.][^/\\]*|\.\.[^/\\]+|\.))*$"
    ),
]

ExternalPath = Annotated[str, Field(pattern=r"^[Hh][Tt][Tt][Pp][Ss]?://")]

Path = InternalPath | ExternalPath
