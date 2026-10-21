import json
import sys
import time

from fairspec_library import validate_table
from fairspec_metadata import Resource


def main() -> None:
    started = time.perf_counter()
    _, csv_path, schema_path = sys.argv
    report = validate_table(Resource(data=csv_path, tableSchema=schema_path))
    elapsed_ms = (time.perf_counter() - started) * 1000
    print(json.dumps({"errorCount": len(report.errors), "elapsedMs": elapsed_ms}))


if __name__ == "__main__":
    main()
