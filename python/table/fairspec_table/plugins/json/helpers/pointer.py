from __future__ import annotations


def parse_json_pointer(pointer: str) -> list[str]:
    if pointer == "":
        return []
    if not pointer.startswith("/"):
        raise Exception(f"Invalid JSON Pointer: {pointer}")

    return [
        token.replace("~1", "/").replace("~0", "~") for token in pointer[1:].split("/")
    ]


def format_json_pointer(tokens: list[str]) -> str:
    return "".join("/" + token.replace("~", "~0").replace("/", "~1") for token in tokens)
