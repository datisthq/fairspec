from __future__ import annotations


def convert_field_names(names: str | list[str]) -> list[str]:
    """Normalize a Frictionless field list, which may be a single name"""
    return [names] if isinstance(names, str) else names
