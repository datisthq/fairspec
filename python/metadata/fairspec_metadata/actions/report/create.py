from __future__ import annotations

from fairspec_metadata.models.error.error import FairspecError
from fairspec_metadata.models.report import Report


def create_report(
    errors: list[FairspecError] | None = None,
    *,
    max_errors: int | None = None,
) -> Report:
    errors = (errors or [])[:max_errors]
    valid = len(errors) == 0
    return Report(valid=valid, errors=errors)
