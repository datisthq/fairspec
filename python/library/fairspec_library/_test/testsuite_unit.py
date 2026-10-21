from __future__ import annotations

import base64
import json
import os
import tempfile
from collections.abc import Callable
from typing import TypedDict

import polars as pl
import pytest
from fairspec_dataset import describe_file, preview_file, validate_file, write_temp_file
from fairspec_metadata import (
    Dataset,
    Resource,
    denormalize_dataset,
    infer_resource_name,
    inspect_json,
    validate_data_schema,
)
from fairspec_table import Table, query_table
from pydantic import BaseModel

from ..actions.data.validate import validate_data
from ..actions.data_schema.infer import infer_data_schema
from ..actions.dataset.infer import infer_dataset
from ..actions.dataset.load import load_dataset
from ..actions.dataset.query import query_dataset
from ..actions.dataset.validate import validate_dataset
from ..actions.file_dialect.infer import infer_file_dialect
from ..actions.table.load import load_table
from ..actions.table.validate import validate_table
from ..actions.table_schema.infer import infer_table_schema

SUITE = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "testsuite")
)
TEST_SCHEMA = os.path.join(SUITE, "test-schema.json")

UNSPECIFIED_MESSAGE_ERRORS = ("data", "metadata", "cell/json")
TEMPORAL_FORMATS = (
    (pl.Date, "%Y-%m-%d"),
    (pl.Time, "%H:%M:%S%.f"),
    (pl.Datetime, "%Y-%m-%dT%H:%M:%S%.f"),
)
ZONED_DATETIME_FORMAT = "%Y-%m-%dT%H:%M:%S%.f%:z"

type Json = None | bool | int | float | str | list[Json] | dict[str, Json]


class Case(TypedDict, total=False):
    description: str
    data: Json
    valid: bool
    errors: list[Json]
    expected: Json
    throws: bool


class Group(TypedDict, total=False):
    description: str
    format: str
    inline: bool
    fileDialect: dict[str, object]
    tableSchema: dict[str, object]
    dataSchema: dict[str, object]
    integrity: dict[str, object]
    textual: bool
    dataset: dict[str, object]
    query: str
    options: dict[str, object]
    tests: list[Case]


def run_data_infer_schema(group: Group, test: Case) -> object:
    return infer_data_schema(create_resource(group, test))


def run_data_validate(group: Group, test: Case) -> object:
    return validate_data(create_resource(group, test))


def run_data_validate_schema(group: Group, test: Case) -> object:
    data = test["data"]
    if not isinstance(data, dict):
        raise TypeError("data-validate-schema data must be a schema object")
    report = validate_data_schema(data)
    return report.model_copy(update={"data_schema": None})


def run_dataset_infer(group: Group, test: Case) -> object:
    path = write_dataset(group, test)
    dataset = Dataset.model_validate(load_dataset(path))
    inferred = infer_dataset(dataset, **group.get("options", {}))
    return denormalize_dataset(inferred, basepath=os.path.dirname(path))


def run_dataset_list(group: Group, test: Case) -> object:
    dataset = Dataset.model_validate(load_dataset(write_dataset(group, test)))
    return [
        resource.name or infer_resource_name(resource, resource_number=index + 1)
        for index, resource in enumerate(dataset.resources or [])
    ]


def run_dataset_query(group: Group, test: Case) -> object:
    query = group.get("query")
    if query is None:
        return None
    return read_rows(query_dataset(write_dataset(group, test), query))


def run_dataset_validate(group: Group, test: Case) -> object:
    return validate_dataset(write_dataset(group, test))


def run_file_describe(group: Group, test: Case) -> object:
    hash_type = group.get("options", {}).get("hashType", "sha256")
    assert isinstance(hash_type, str)
    return describe_file(write_data(group, test["data"]), hash_type=hash_type)


def run_file_preview(group: Group, test: Case) -> object:
    options = group.get("options", {})
    max_bytes = options.get("maxBytes")
    max_lines = options.get("maxLines")
    assert max_bytes is None or isinstance(max_bytes, int)
    assert max_lines is None or isinstance(max_lines, int)
    return preview_file(
        write_data(group, test["data"]), max_bytes=max_bytes, max_lines=max_lines
    )


def run_file_validate(group: Group, test: Case) -> object:
    return validate_file(create_resource(group, test))


def run_table_infer_dialect(group: Group, test: Case) -> object:
    options = dict(group.get("options", {}))
    if "sampleBytes" in options:
        options["sample_bytes"] = options.pop("sampleBytes")
    return infer_file_dialect(create_resource(group, test), **options)


def run_table_infer_schema(group: Group, test: Case) -> object:
    return infer_table_schema(create_resource(group, test), **group.get("options", {}))


def run_table_load(group: Group, test: Case) -> object:
    table = load_table(create_resource(group, test), **group.get("options", {}))
    return None if table is None else read_rows(table)


def run_table_query(group: Group, test: Case) -> object:
    table = load_table(create_resource(group, test), **group.get("options", {}))
    query = group.get("query")
    if table is None or query is None:
        return None
    return read_rows(query_table(table, query))


def run_table_validate(group: Group, test: Case) -> object:
    return validate_table(create_resource(group, test), **group.get("options", {}))


OPERATIONS: dict[str, Callable[[Group, Case], object]] = {
    "data-infer-schema": run_data_infer_schema,
    "data-validate": run_data_validate,
    "data-validate-schema": run_data_validate_schema,
    "dataset-infer": run_dataset_infer,
    "dataset-list": run_dataset_list,
    "dataset-query": run_dataset_query,
    "dataset-validate": run_dataset_validate,
    "file-describe": run_file_describe,
    "file-preview": run_file_preview,
    "file-validate": run_file_validate,
    "table-infer-dialect": run_table_infer_dialect,
    "table-infer-schema": run_table_infer_schema,
    "table-load": run_table_load,
    "table-query": run_table_query,
    "table-validate": run_table_validate,
}


def list_files() -> list[tuple[str, str]]:
    tests = os.path.join(SUITE, "tests")
    return [
        (operation, file)
        for operation in sorted(os.listdir(tests))
        for file in sorted(os.listdir(os.path.join(tests, operation)))
    ]


def load_groups(operation: str, file: str) -> list[Group]:
    with open(os.path.join(SUITE, "tests", operation, file), encoding="utf-8") as f:
        return json.load(f)


def list_tests() -> list[object]:
    return [
        pytest.param(
            operation,
            group,
            test,
            id=f"{operation}/{file[:-5]} › {group['description']} › {test['description']}",
        )
        for operation, file in list_files()
        for group in load_groups(operation, file)
        for test in group["tests"]
    ]


class TestTestsuite:
    @pytest.mark.parametrize(
        "operation, file", list_files(), ids=[f"{o}/{f}" for o, f in list_files()]
    )
    def test_file(self, operation: str, file: str):
        assert operation in OPERATIONS
        groups = load_groups(operation, file)
        assert inspect_json(groups, json_schema=TEST_SCHEMA) == []

    @pytest.mark.parametrize("operation, group, test", list_tests())
    def test_case(self, operation: str, group: Group, test: Case):
        run = OPERATIONS[operation]

        if test.get("throws"):
            with pytest.raises(Exception):
                run(group, test)
            return

        result = canonicalize(run(group, test))

        if "expected" in test:
            assert_same(result, test["expected"])

        if "valid" in test:
            assert isinstance(result, dict)
            assert result["valid"] is test["valid"]
            if "errors" in test or test["valid"]:
                assert_same(
                    result, {"valid": test["valid"], "errors": test.get("errors", [])}
                )


def create_resource(group: Group, test: Case) -> Resource:
    data = test["data"] if group.get("inline") else write_data(group, test["data"])
    return Resource.model_validate(
        {
            "data": data,
            "fileDialect": group.get("fileDialect"),
            "tableSchema": group.get("tableSchema"),
            "dataSchema": group.get("dataSchema"),
            "integrity": group.get("integrity"),
            "textual": group.get("textual"),
        }
    )


def write_data(group: Group, data: Json) -> str:
    return write_temp_file(encode_data(data), format=group.get("format", "csv"))


def write_dataset(group: Group, test: Case) -> str:
    files = test["data"]
    assert isinstance(files, dict), "dataset data must map file names to contents"

    folder = tempfile.mkdtemp(prefix="fairspec-testsuite-")
    for name, content in files.items():
        with open(os.path.join(folder, name), "wb") as f:
            encoded = encode_data(content)
            f.write(encoded.encode("utf-8") if isinstance(encoded, str) else encoded)

    path = os.path.join(folder, "dataset.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(group["dataset"], f)
    return path


def read_rows(table: Table) -> object:
    frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
    columns: list[str] = []
    for name, dtype in frame.schema.items():
        format = next((f for t, f in TEMPORAL_FORMATS if isinstance(dtype, t)), None)
        if isinstance(dtype, pl.Datetime) and dtype.time_zone:
            format = ZONED_DATETIME_FORMAT
        quoted = f'"{name}"'
        columns.append(
            f"STRFTIME({quoted}, '{format}') AS {quoted}" if format else quoted
        )
    rendered: pl.DataFrame = query_table(  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        frame.lazy(), f"SELECT {', '.join(columns)} FROM self"
    ).collect()
    return {"columns": rendered.columns, "rows": [list(row) for row in rendered.rows()]}


def encode_data(data: Json) -> str | bytes:
    if isinstance(data, str):
        return data
    if isinstance(data, list):
        return json.dumps(data)
    if isinstance(data, dict):
        encoded = data.get("base64")
        if isinstance(encoded, str):
            return base64.b64decode(encoded)
    raise ValueError("data must be a string, base64 object or array")


def assert_same(actual: Json, expected: Json):
    assert actual == expected
    assert json.dumps(actual, sort_keys=True) == json.dumps(expected, sort_keys=True), (
        "equal only after Python's numeric coercion (1 == 1.0 == True)"
    )


def canonicalize(value: object) -> Json:
    if isinstance(value, BaseModel):
        value = value.model_dump(mode="json", by_alias=True, exclude_none=True)
    result = json.loads(json.dumps(value))
    if isinstance(result, dict) and isinstance(result.get("errors"), list):
        for error in result["errors"]:
            if (
                isinstance(error, dict)
                and error.get("type") in UNSPECIFIED_MESSAGE_ERRORS
            ):
                error.pop("message", None)
    return result
