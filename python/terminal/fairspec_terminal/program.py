from importlib.metadata import version

from fireargs import Program

from .commands.data.infer_schema import infer_data_schema_command
from .commands.data.validate import validate_data_command
from .commands.data.validate_schema import validate_data_schema_command
from .commands.dataset.copy import copy_dataset_command
from .commands.dataset.infer import infer_dataset_command
from .commands.dataset.list import list_dataset_command
from .commands.dataset.query import query_dataset_command
from .commands.dataset.script import script_dataset_command
from .commands.dataset.validate import validate_dataset_command
from .commands.file.copy import copy_file_command
from .commands.file.describe import describe_file_command
from .commands.file.preview import preview_file_command
from .commands.file.validate import validate_file_command
from .commands.file_dialect.infer import infer_file_dialect_command
from .commands.general.mcp import mcp_command
from .commands.table.describe import describe_table_command
from .commands.table.infer_schema import infer_table_schema_command
from .commands.table.preview import preview_table_command
from .commands.table.query import query_table_command
from .commands.table.render_schema import render_table_schema_command
from .commands.table.script import script_table_command
from .commands.table.validate import validate_table_command
from .commands.table.validate_schema import validate_table_schema_command

dataset_program = Program(
    description="Dataset related commands",
    router={
        "infer": infer_dataset_command,
        "copy": copy_dataset_command,
        "validate": validate_dataset_command,
        "list": list_dataset_command,
        "query": query_dataset_command,
        "script": script_dataset_command,
    },
)

table_program = Program(
    description="Table related commands",
    router={
        "describe": describe_table_command,
        "preview": preview_table_command,
        "query": query_table_command,
        "script": script_table_command,
        "validate": validate_table_command,
        "infer-schema": infer_table_schema_command,
        "render-schema": render_table_schema_command,
        "validate-schema": validate_table_schema_command,
        "infer-dialect": infer_file_dialect_command,
    },
)

data_program = Program(
    description="Data related commands",
    router={
        "validate": validate_data_command,
        "infer-schema": infer_data_schema_command,
        "validate-schema": validate_data_schema_command,
        "infer-dialect": infer_file_dialect_command,
    },
)

file_program = Program(
    description="File related commands",
    router={
        "copy": copy_file_command,
        "describe": describe_file_command,
        "preview": preview_file_command,
        "validate": validate_file_command,
        "infer-dialect": infer_file_dialect_command,
    },
)

program = Program(
    name="fairspec",
    description="Fairspec Python is a fast data management framework built on top of the Fairspec standard and Polars DataFrames",
    version=version("fairspec-terminal"),
    version_option={"flags": "-v, --version"},
    router={
        "dataset": dataset_program,
        "table": table_program,
        "data": data_program,
        "file": file_program,
        "mcp": mcp_command,
    },
)
