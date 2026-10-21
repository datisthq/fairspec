from __future__ import annotations

import json

from fairspec_dataset import write_temp_file
from click.testing import CliRunner

from fairspec_terminal.program import file_program


class TestFilePreviewCommand:
    def test_should_preview_csv_file_as_json(self):
        path = write_temp_file("id,name\n1,[alice]\n", format="csv")

        result = CliRunner().invoke(file_program, ["preview", path, "--json"])

        assert result.exit_code == 0
        assert json.loads(result.stdout) == {
            "text": "id,name\n1,[alice]\n",
            "truncated": False,
        }

    def test_should_limit_preview_to_lines(self):
        path = write_temp_file("id\n1\n2\n3\n", format="csv")

        result = CliRunner().invoke(
            file_program, ["preview", path, "--lines", "2", "--json"]
        )

        assert json.loads(result.stdout) == {"text": "id\n1\n", "truncated": True}

    def test_should_limit_preview_to_sample_bytes(self):
        path = write_temp_file("abcdefgh", format="txt")

        result = CliRunner().invoke(
            file_program, ["preview", path, "--sample-bytes", "3", "--json"]
        )

        assert json.loads(result.stdout) == {"text": "abc", "truncated": True}

    def test_should_print_raw_source_with_brackets_intact(self):
        path = write_temp_file("id,name\n1,[alice]\n2,[bold]bob[/bold]\n", format="csv")

        result = CliRunner().invoke(file_program, ["preview", path])

        assert result.exit_code == 0
        assert "1,[alice]\n2,[bold]bob[/bold]\n" in result.stdout
