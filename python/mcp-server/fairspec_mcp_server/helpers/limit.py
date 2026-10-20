from collections.abc import Sequence

from fairspec_library import Report
from pydantic_core import to_json

DEFAULT_LIMIT = 100
MAX_LIMIT = 1000
MAX_RESPONSE_CHARS = 50_000

# room for the object a trimmed list is wrapped in
MAX_ITEMS_CHARS = MAX_RESPONSE_CHARS - 1_000


def limit_items[T](items: Sequence[T], *, limit: int) -> tuple[list[T], bool]:
    """Cut a list to `limit` items, then to as many as fit the response budget.

    Pass one item more than the limit to learn whether the list was truncated.
    """
    chars = 0
    count = 0
    for item in items[:limit]:
        chars += len(to_json(item, by_alias=True, exclude_none=True)) + 1
        if chars > MAX_ITEMS_CHARS:
            break
        count += 1

    return list(items[:count]), count < len(items)


def limit_report(report: Report, *, max_errors: int | None) -> tuple[Report, bool]:
    """Cut a report's errors to `max_errors`, then to the response budget."""
    limit = max_errors if max_errors is not None else DEFAULT_LIMIT
    errors, truncated = limit_items(report.errors, limit=limit)
    return Report(valid=report.valid, errors=errors), truncated
