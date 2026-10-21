from fairspec_library import infer_file_dialect
from fairspec_dataset import InferFileDialectOptions
from fairspec_metadata import FileDialect, Resource
from fireargs import Context, command

from fairspec_terminal.helpers.file import select_file
from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.params import (
    Debug,
    FilePath,
    FromDataset,
    FromResource,
    SampleBytes,
    Silent,
)
from fairspec_terminal.session import Session


class InferFileDialectInput(CommandInput):
    path: FilePath = None
    dataset: FromDataset = None
    resource: FromResource = None
    sample_bytes: SampleBytes = None
    silent: Silent = False
    debug: Debug = False


@command(description="Infer the dialect of a file")
def infer_file_dialect_command(
    input: InferFileDialectInput, context: Context
) -> FileDialect:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)
    path = input.path or select_file(
        session, dataset=input.dataset, resource=input.resource
    )

    def _infer() -> FileDialect:
        options: InferFileDialectOptions = {}
        if input.sample_bytes is not None:
            options["sample_bytes"] = input.sample_bytes
        file_dialect = infer_file_dialect(Resource(data=path), **options)
        if not file_dialect:
            raise ValueError("Could not infer dialect")
        return file_dialect

    file_dialect = session.task("Inferring dialect", _infer)

    return session.render_data_result(file_dialect)
