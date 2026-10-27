from __future__ import annotations

import json


def decode_json_buffer(data: bytes, *, is_lines: bool) -> object:
    text = data.decode("utf-8-sig")
    if is_lines:
        lines = text.split("\n")
        return [json.loads(line) for line in lines if line.strip(" \t\r")]
    return json.loads(text)
