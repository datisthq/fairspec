from __future__ import annotations

import json

from fairspec_dataset import write_temp_file
from typer.testing import CliRunner

from fairspec_terminal.program import dataset_program, register_commands


class TestDatasetInferCommand:
    def test_should_infer_table_schema(self):
        register_commands()
        path = write_temp_file("id,name\n1,english\n", format="csv")

        runner = CliRunner()
        result = runner.invoke(dataset_program, ["infer", path, "--json"])

        assert result.exit_code == 0
        properties = json.loads(result.stdout)["resources"][0]["tableSchema"][
            "properties"
        ]
        assert properties["id"]["type"] == "integer"

    def test_should_pass_table_schema_options(self):
        register_commands()
        path = write_temp_file("id,name\n1,english\n", format="csv")

        runner = CliRunner()
        result = runner.invoke(
            dataset_program, ["infer", path, "--json", "--keep-strings"]
        )

        assert result.exit_code == 0
        properties = json.loads(result.stdout)["resources"][0]["tableSchema"][
            "properties"
        ]
        assert properties["id"]["type"] == "string"
