from typing import Annotated, Literal

from fairspec_library import render_table_schema_as
from fairspec_metadata import RenderTableSchemaOptions, TableSchema, resolve_table_schema
from fireargs import Context, command
from pydantic import Field

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import TextResult
from fairspec_terminal.params import Debug, RequiredFilePath, Silent, ToPath
from fairspec_terminal.session import Session


class RenderTableSchemaInput(CommandInput):
    path: RequiredFilePath
    to_format: Annotated[
        Literal["markdown", "html"], Field(description="target schema format")
    ]
    to_path: ToPath = None
    silent: Silent = False
    debug: Debug = False


@command(description="Render a Table Schema as HTML or Markdown")
def render_table_schema_command(
    input: RenderTableSchemaInput, context: Context
) -> TextResult:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)

    def _load() -> TableSchema:
        table_schema = resolve_table_schema(input.path)
        if not table_schema:
            raise ValueError("Could not load table schema")
        return table_schema

    table_schema = session.task("Loading table schema", _load)

    rendered = session.task(
        "Rendering table schema",
        lambda: (
            render_table_schema_as(
                table_schema, RenderTableSchemaOptions(format=input.to_format)
            )
            or ""
        ),
    )

    if not input.to_path:
        session.render_raw_result(rendered)
        return TextResult(result=rendered)

    to_path = input.to_path

    def _save() -> None:
        with open(to_path, "w", encoding="utf-8") as file:
            file.write(rendered)

    session.task("Saving rendered schema", _save)

    return session.render_text_result(
        f"Saved rendered schema to [bold]{to_path}[/bold]",
        status="success",
    )
