from __future__ import annotations

import json
import os

from fairspec_dataset import get_temp_folder_path
from typer.testing import CliRunner

from fairspec_terminal.program import register_commands, table_program


class TestTableValidateExitCode:
    def test_should_keep_zero_exit_code_for_valid_table(self, monkeypatch):
        register_commands()
        folder = get_temp_folder_path()
        with open(os.path.join(folder, "data.csv"), "w", newline="") as file:
            file.write("id,age\n1,25\n2,30\n")
        with open(os.path.join(folder, "schema.json"), "w") as file:
            json.dump({"properties": {"age": {"type": "integer"}}}, file)
        monkeypatch.chdir(folder)

        result = CliRunner().invoke(
            table_program, ["validate", "data.csv", "--schema", "schema.json", "--json"]
        )

        assert result.exit_code == 0
        assert json.loads(result.stdout)["valid"] is True

    def test_should_set_non_zero_exit_code_for_invalid_table(self, monkeypatch):
        register_commands()
        folder = get_temp_folder_path()
        with open(os.path.join(folder, "data.csv"), "w", newline="") as file:
            file.write("id,age\n1,200\n")
        with open(os.path.join(folder, "schema.json"), "w") as file:
            json.dump({"properties": {"age": {"type": "integer", "maximum": 150}}}, file)
        monkeypatch.chdir(folder)

        result = CliRunner().invoke(
            table_program, ["validate", "data.csv", "--schema", "schema.json", "--json"]
        )

        assert result.exit_code != 0
        assert json.loads(result.stdout)["valid"] is False
