from __future__ import annotations

NUMBER_COLUMN_NAME = "fairspec:number"
ERROR_COLUMN_NAME = "fairspec:error"

# The engine polars executes a query plan with, for collects and sinks alike. The
# in-memory engine holds the whole source while it runs, so peak memory grows with
# the file; the streaming engine keeps a bounded working set instead, and 50MB,
# 500MB and 1GB validations all fit in the same 384MB.
QUERY_ENGINE = "streaming"

BASE64_REGEX = (
    r"^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$"
)
HEX_REGEX = r"^[0-9a-fA-F]*$"
RFC5322_EMAIL_REGEX = r'^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$'
URL_REGEX = r"^https?://.+"
