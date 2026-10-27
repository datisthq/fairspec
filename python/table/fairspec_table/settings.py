from __future__ import annotations

NUMBER_COLUMN_NAME = "fairspec:number"
ERROR_COLUMN_NAME = "fairspec:error"
COUNT_COLUMN_NAME = "fairspec:count"

# The engine polars executes a query plan with, for collects and sinks alike. The
# in-memory engine holds the whole source while it runs, so peak memory grows with
# the file; the streaming engine keeps a bounded working set instead, and 50MB,
# 500MB and 1GB validations all fit in the same 384MB.
QUERY_ENGINE = "streaming"

BASE64_REGEX = (
    r"^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$"
)
HEX_REGEX = r"^(?:[0-9A-Fa-f]{2})*$"

# Every class is spelled out in ASCII: `\d` and `(?i)` match non-ASCII in Polars' engine
_HEX_GROUP = r"[0-9A-Fa-f]{1,4}"
_DEC_OCTET = r"(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])"
_IPV4_ADDRESS = rf"{_DEC_OCTET}(?:\.{_DEC_OCTET}){{3}}"
_LS32 = rf"(?:{_HEX_GROUP}:{_HEX_GROUP}|{_IPV4_ADDRESS})"
_IPV6_ADDRESS = (
    "(?:"
    + "|".join(
        [
            rf"(?:{_HEX_GROUP}:){{6}}{_LS32}",
            rf"::(?:{_HEX_GROUP}:){{5}}{_LS32}",
            rf"(?:{_HEX_GROUP})?::(?:{_HEX_GROUP}:){{4}}{_LS32}",
            rf"(?:(?:{_HEX_GROUP}:){{0,1}}{_HEX_GROUP})?::(?:{_HEX_GROUP}:){{3}}{_LS32}",
            rf"(?:(?:{_HEX_GROUP}:){{0,2}}{_HEX_GROUP})?::(?:{_HEX_GROUP}:){{2}}{_LS32}",
            rf"(?:(?:{_HEX_GROUP}:){{0,3}}{_HEX_GROUP})?::{_HEX_GROUP}:{_LS32}",
            rf"(?:(?:{_HEX_GROUP}:){{0,4}}{_HEX_GROUP})?::{_LS32}",
            rf"(?:(?:{_HEX_GROUP}:){{0,5}}{_HEX_GROUP})?::{_HEX_GROUP}",
            rf"(?:(?:{_HEX_GROUP}:){{0,6}}{_HEX_GROUP})?::",
        ]
    )
    + ")"
)
_URL_CHARS = r"A-Za-z0-9\-._~!$&'()*+,;="
_PCT_ENCODED = r"%[0-9A-Fa-f]{2}"
URL_REGEX = (
    rf"^[Hh][Tt][Tt][Pp][Ss]?://(?:(?:[{_URL_CHARS}:]|{_PCT_ENCODED})*@)?"
    rf"(?:\[(?:{_IPV6_ADDRESS}|[Vv][0-9A-Fa-f]+\.[{_URL_CHARS}:]+)\]"
    rf"|(?:[{_URL_CHARS}]|{_PCT_ENCODED})+)(?::[0-9]*)?"
    rf"(?:/(?:[{_URL_CHARS}:@]|{_PCT_ENCODED})*)*"
    rf"(?:\?(?:[{_URL_CHARS}:@/?]|{_PCT_ENCODED})*)?"
    rf"(?:#(?:[{_URL_CHARS}:@/?]|{_PCT_ENCODED})*)?$"
)

_ATEXT = r"[A-Za-z0-9!#$%&'*+\-/=?^_`{|}~]"
_SUB_DOMAIN = r"[A-Za-z0-9]+(?:-+[A-Za-z0-9]+)*"
_SNUM = r"(?:25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})"
# A general address literal's tag also matches `IPv6`, so it covers the IPv6 literal
EMAIL_REGEX = (
    rf'^(?:{_ATEXT}+(?:\.{_ATEXT}+)*|"(?:[ !#-\[\]-~]|\\[ -~])*")@'
    rf"(?:{_SUB_DOMAIN}(?:\.{_SUB_DOMAIN})*"
    rf"|\[(?:{_SNUM}(?:\.{_SNUM}){{3}}|(?:-*[A-Za-z0-9])+:[!-Z^-~]+)\])$"
)

_DURATION_TIME = (
    r"[Tt][0-9]+(?:[Hh](?:[0-9]+[Mm](?:[0-9]+[Ss])?)?|[Mm](?:[0-9]+[Ss])?|[Ss])"
)
DURATION_REGEX = (
    r"^[Pp](?:[0-9]+(?:[Dd]|[Mm](?:[0-9]+[Dd])?|[Yy](?:[0-9]+[Mm](?:[0-9]+[Dd])?)?)"
    rf"(?:{_DURATION_TIME})?|{_DURATION_TIME}|[0-9]+[Ww])$"
)
