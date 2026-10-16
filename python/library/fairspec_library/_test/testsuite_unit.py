from __future__ import annotations

import base64
import json
import os
import tempfile
from collections.abc import Callable
from typing import TypedDict

import polars as pl
import pytest
from fairspec_dataset import describe_file, validate_file, write_temp_file
from fairspec_metadata import Resource, inspect_json
from pydantic import BaseModel

from ..actions.dataset.validate import validate_dataset
from ..actions.file_dialect.infer import infer_file_dialect
from ..actions.table.load import load_table
from ..actions.table.validate import validate_table
from ..actions.table_schema.infer import infer_table_schema

SUITE = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "testsuite")
)
TEST_SCHEMA = os.path.join(SUITE, "test-schema.json")

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
    fileDialect: dict[str, object]
    tableSchema: dict[str, object]
    integrity: dict[str, object]
    dataset: dict[str, object]
    options: dict[str, object]
    tests: list[Case]


def run_file_describe(group: Group, test: Case) -> object:
    return describe_file(write_data(group, test["data"]))


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
    if table is None:
        return None
    frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
    return {"columns": frame.columns, "rows": [list(row) for row in frame.rows()]}


def run_table_validate(group: Group, test: Case) -> object:
    return validate_table(create_resource(group, test), **group.get("options", {}))


def run_dataset_validate(group: Group, test: Case) -> object:
    return validate_dataset(write_dataset(group, test))


OPERATIONS: dict[str, Callable[[Group, Case], object]] = {
    "file-describe": run_file_describe,
    "file-validate": run_file_validate,
    "table-infer-dialect": run_table_infer_dialect,
    "table-infer-schema": run_table_infer_schema,
    "table-load": run_table_load,
    "table-validate": run_table_validate,
    "dataset-validate": run_dataset_validate,
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
    return Resource.model_validate(
        {
            "data": write_data(group, test["data"]),
            "fileDialect": group.get("fileDialect"),
            "tableSchema": group.get("tableSchema"),
            "integrity": group.get("integrity"),
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
    return json.loads(json.dumps(value))
