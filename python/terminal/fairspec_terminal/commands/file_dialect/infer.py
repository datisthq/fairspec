from fairspec_library import infer_file_dialect
from fairspec_metadata import Resource

from fairspec_terminal.helpers.file import select_file
from fairspec_terminal.params import (
    Debug,
    FromDataset,
    FromResource,
    Json,
    OptionalPath,
    SampleBytes,
    Silent,
)
from fairspec_terminal.session import Session


def create_infer_dialect_command(program: object) -> None:
    import typer

    assert isinstance(program, typer.Typer)

    @program.command(name="infer-dialect")
    def infer_dialect(
        path: OptionalPath = None,
        dataset: FromDataset = None,
        resource: FromResource = None,
        sample_bytes: SampleBytes = None,
        silent: Silent = False,
        debug: Debug = False,
        json: Json = False,
    ) -> None:
        """Infer the dialect of a file."""
        session = Session(silent=silent, debug=debug, json=json)

        if not path:
            path = select_file(session, dataset=dataset, resource=resource)

        def _infer() -> object:
            kwargs = {}
            if sample_bytes is not None:
                kwargs["sampleBytes"] = sample_bytes
            file_dialect = infer_file_dialect(Resource(data=path), **kwargs)
            if not file_dialect:
                raise ValueError("Could not infer dialect")
            return file_dialect

        file_dialect = session.task("Inferring dialect", _infer)

        session.render_data_result(file_dialect)
