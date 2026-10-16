from typing import Annotated

import typer
from fairspec_library import render_table_schema_as
from fairspec_metadata import RenderTableSchemaOptions, TableSchema, resolve_table_schema

from fairspec_terminal.params import Debug, Json, RequiredFilePath, Silent, ToPath
from fairspec_terminal.program import table_program
from fairspec_terminal.session import Session

ToFormat = Annotated[str, typer.Option("--to-format", help="target schema format")]


@table_program.command(name="render-schema")
def render_schema(
    path: RequiredFilePath,
    to_format: ToFormat,
    to_path: ToPath = None,
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Render a Table Schema as HTML or Markdown."""
    session = Session(silent=silent, debug=debug, json=json)

    if not to_format:
        raise ValueError("--to-format must be specified")

    def _load() -> TableSchema | None:
        return resolve_table_schema(path)

    table_schema = session.task("Loading table schema", _load)

    if not table_schema:
        raise ValueError("Could not load table schema")

    def _render() -> str | None:
        return render_table_schema_as(
            table_schema, RenderTableSchemaOptions(format=to_format)
        )

    rendered = session.task("Rendering table schema", _render)

    def _save() -> bool:
        if not to_path:
            return False
        with open(to_path, "w", encoding="utf-8") as f:
            f.write(rendered or "")
        return True

    is_saved = session.task("Saving rendered schema", _save)

    if not is_saved:
        session.render_text_result(rendered or "")
        return

    session.render_text_result(
        f"Saved rendered schema to [bold]{to_path}[/bold]",
        status="success",
    )
