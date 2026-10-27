from __future__ import annotations

import re


def convert_name_from_frictionless(name: str) -> str:
    """Convert a Frictionless name to a Fairspec resource name (letters, digits and `_`)"""
    return re.sub(r"[^a-zA-Z0-9_]", "_", name)
