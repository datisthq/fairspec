import typer

program = typer.Typer(
    name="fairspec",
    help="Fairspec Python is a fast data management framework built on top of the Fairspec standard and Polars DataFrames",
)
dataset_program = typer.Typer(help="Dataset related commands")
table_program = typer.Typer(help="Table related commands")
data_program = typer.Typer(help="Data related commands")
file_program = typer.Typer(help="File related commands")

program.add_typer(dataset_program, name="dataset")
program.add_typer(table_program, name="table")
program.add_typer(data_program, name="data")
program.add_typer(file_program, name="file")


def register_commands() -> None:
    import fairspec_terminal.commands.dataset.copy  # noqa: F401
    import fairspec_terminal.commands.dataset.infer  # noqa: F401
    import fairspec_terminal.commands.dataset.list_  # noqa: F401
    import fairspec_terminal.commands.dataset.script  # noqa: F401
    import fairspec_terminal.commands.dataset.validate  # noqa: F401
    import fairspec_terminal.commands.table.describe  # noqa: F401
    import fairspec_terminal.commands.table.preview  # noqa: F401
    import fairspec_terminal.commands.table.query  # noqa: F401
    import fairspec_terminal.commands.table.script  # noqa: F401
    import fairspec_terminal.commands.table.validate  # noqa: F401
    import fairspec_terminal.commands.table.infer_schema  # noqa: F401
    import fairspec_terminal.commands.table.render_schema  # noqa: F401
    import fairspec_terminal.commands.table.validate_schema  # noqa: F401
    import fairspec_terminal.commands.data.validate  # noqa: F401
    import fairspec_terminal.commands.data.infer_schema  # noqa: F401
    import fairspec_terminal.commands.data.validate_schema  # noqa: F401
    import fairspec_terminal.commands.file.describe  # noqa: F401
    import fairspec_terminal.commands.file.copy  # noqa: F401
    import fairspec_terminal.commands.file.validate  # noqa: F401
    from fairspec_terminal.commands.file_dialect.infer import create_infer_dialect_command

    create_infer_dialect_command(table_program)
    create_infer_dialect_command(data_program)
    create_infer_dialect_command(file_program)
