from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .report import Report


class FairspecException(Exception):
    report: Report | None

    def __init__(self, message: str, *, report: Report | None = None) -> None:
        super().__init__(message)
        self.report = report
