from __future__ import annotations


def quote_identifier(name: str) -> str:
    return '"' + name.replace('"', '""') + '"'
