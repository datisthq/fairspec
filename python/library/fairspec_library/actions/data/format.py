from __future__ import annotations

import re
from collections.abc import Callable

from fairspec_table import DURATION_REGEX, EMAIL_REGEX

_DATE_REGEX = re.compile(r"([0-9]{4})-([0-9]{2})-([0-9]{2})")
_TIME_REGEX = re.compile(
    r"([0-9]{2}):([0-9]{2}):([0-9]{2})(?:\.[0-9]+)?(?:[Zz]|([+-])([0-9]{2}):([0-9]{2}))"
)
_DEC_OCTET = r"(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])"
_IPV4_REGEX = re.compile(rf"{_DEC_OCTET}(?:\.{_DEC_OCTET}){{3}}")
_UUID_REGEX = re.compile(
    r"[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}"
)
_EMAIL_PATTERN = re.compile(EMAIL_REGEX)
_DURATION_PATTERN = re.compile(DURATION_REGEX)


def _get_is_date(value: str) -> bool:
    match = _DATE_REGEX.fullmatch(value)
    if not match:
        return False

    year, month, day = (int(part) for part in match.groups())
    is_leap = year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)
    days = [31, 29 if is_leap else 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    return 1 <= month <= 12 and 1 <= day <= days[month - 1]


# RFC 3339 allows a leap second only at the last minute of a UTC day
def _get_is_time(value: str) -> bool:
    match = _TIME_REGEX.fullmatch(value)
    if not match:
        return False

    hour, minute, second = (int(part) for part in match.groups()[:3])
    sign = -1 if match.group(4) == "-" else 1
    offset_hour = int(match.group(5) or 0)
    offset_minute = int(match.group(6) or 0)
    if hour > 23 or minute > 59 or second > 60:
        return False
    if offset_hour > 23 or offset_minute > 59:
        return False
    if second < 60:
        return True

    utc_minutes = hour * 60 + minute - sign * (offset_hour * 60 + offset_minute)
    return utc_minutes % 1440 == 23 * 60 + 59


def _get_is_date_time(value: str) -> bool:
    return (
        value[10:11] in ("T", "t")
        and _get_is_date(value[:10])
        and _get_is_time(value[11:])
    )


DATA_SCHEMA_FORMATS: dict[str, Callable[[str], bool]] = {
    "date": _get_is_date,
    "time": _get_is_time,
    "date-time": _get_is_date_time,
    "duration": lambda value: _DURATION_PATTERN.fullmatch(value) is not None,
    "email": lambda value: _EMAIL_PATTERN.fullmatch(value) is not None,
    "ipv4": lambda value: _IPV4_REGEX.fullmatch(value) is not None,
    "uuid": lambda value: _UUID_REGEX.fullmatch(value) is not None,
}
"""The `format` values a Data Schema asserts; any other format is an annotation"""
