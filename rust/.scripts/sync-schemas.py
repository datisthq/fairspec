"""Write Python's MCP tool schemas into the Rust server.

The tools/list result of Python's server is split into `defs.json`, every `$defs` entry
shared by the schemas, and `tools.json`, where each schema's `$defs` is the list of its
names; `mcp-server/src/helpers/schema.rs` puts them back together.
"""

import json
import subprocess
import sys
from pathlib import Path

SCHEMAS = Path(__file__).parent.parent / "mcp-server" / "schemas"
SERVER = "from fairspec_mcp_server import server; server.run('stdio')"


def main() -> None:
    tools = list_tools()

    defs: dict[str, object] = {}
    for tool in tools:
        for key in ("inputSchema", "outputSchema"):
            for name, value in tool.get(key, {}).get("$defs", {}).items():
                if defs.setdefault(name, value) != value:
                    raise SystemExit(f"Schemas define {name} differently")
            if "$defs" in tool.get(key, {}):
                tool[key]["$defs"] = list(tool[key]["$defs"])

    write_json(SCHEMAS / "defs.json", dict(sorted(defs.items())))
    write_json(SCHEMAS / "tools.json", tools)


def list_tools() -> list[dict]:
    process = subprocess.Popen(
        [sys.executable, "-c", SERVER],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
        encoding="utf-8",
    )
    assert process.stdin and process.stdout

    def send(message: dict) -> None:
        process.stdin.write(json.dumps({"jsonrpc": "2.0", **message}) + "\n")
        process.stdin.flush()

    send(
        {
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "sync-schemas", "version": "1"},
            },
        }
    )
    process.stdout.readline()
    send({"method": "notifications/initialized"})
    send({"id": 2, "method": "tools/list"})
    tools = json.loads(process.stdout.readline())["result"]["tools"]

    process.stdin.close()
    process.wait()
    return tools


def write_json(path: Path, value: object) -> None:
    text = json.dumps(value, indent=1, ensure_ascii=False)
    path.write_text(text, encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
