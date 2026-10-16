from __future__ import annotations

import json


def encode_json_buffer(data: object, *, is_lines: bool) -> bytes:
    if is_lines:
        assert isinstance(data, list)
        text = "\n".join(json.dumps(item, ensure_ascii=False) for item in data)
    else:
        text = json.dumps(data, indent=2, ensure_ascii=False)
    return text.encode("utf-8")
