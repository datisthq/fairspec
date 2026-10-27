from __future__ import annotations

import json


def decode_json_buffer(data: bytes, *, is_lines: bool) -> object:
    text = data.decode("utf-8-sig")
    if is_lines:
        lines = (line.removesuffix("\r") for line in text.split("\n"))
        return [json.loads(line) for line in lines if line]
    return json.loads(text)
