from __future__ import annotations

import json

from click.testing import CliRunner
from fairspec_dataset import write_temp_file

from fairspec_terminal.program import program


class TestProgram:
    def test_should_list_every_command_once_in_llms_manifest(self):
        result = CliRunner().invoke(program, ["--llms"])

        names = [tool["name"] for tool in json.loads(result.stdout)["tools"]]
        assert len(set(names)) == len(names)
        assert "table validate" in names
        assert "table infer-dialect" in names
        assert "data infer-dialect" in names
        assert "file infer-dialect" in names
        assert "mcp" in names

    def test_should_use_camel_case_keys_in_llms_manifest(self):
        result = CliRunner().invoke(program, ["--llms"])

        tools = {tool["name"]: tool for tool in json.loads(result.stdout)["tools"]}
        properties = tools["table validate"]["inputSchema"]["properties"]
        assert "headerRows" in properties
        assert "schema" in properties

    def test_should_take_command_input_as_json(self):
        path = write_temp_file("id,name\n1,alice\n2,bob", format="csv")

        result = CliRunner().invoke(
            program, ["table", "preview", "--json", json.dumps({"path": path})]
        )

        assert result.exit_code == 0, result.output
        assert json.loads(result.stdout) == [
            {"id": 1, "name": "alice"},
            {"id": 2, "name": "bob"},
        ]

    def test_should_parse_comma_separated_option_from_command_line(self):
        path = write_temp_file("id,name\n1,alice\n2,-", format="csv")

        result = CliRunner().invoke(
            program,
            ["table", "infer-schema", path, "--missing-values", "-,n/a", "--json"],
        )

        assert json.loads(result.stdout)["missingValues"] == ["-", "n/a"]

    def test_should_collect_repeated_check_option(self):
        path = write_temp_file("id\n1\n2", format="csv")

        result = CliRunner().invoke(
            program,
            [
                "table",
                "validate",
                path,
                "--check",
                "a=id > 1",
                "--check",
                "b=id > 2",
                "--json",
            ],
        )

        check_names = {
            error["checkName"] for error in json.loads(result.stdout)["errors"]
        }
        assert check_names == {"a", "b"}
        assert result.exit_code == 1
