from __future__ import annotations

import json


def decode_json_buffer(data: bytes, *, is_lines: bool) -> object:
    text = data.decode("utf-8")
    if is_lines:
        return [json.loads(line) for line in text.split("\n") if line.strip()]
    return json.loads(text)
