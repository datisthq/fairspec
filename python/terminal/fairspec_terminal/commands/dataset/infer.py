from fairspec_library import infer_dataset
from fairspec_metadata import Dataset

from fairspec_terminal.params import (
    ArrayType,
    ColumnTypes,
    CommaDecimal,
    Confidence,
    DateFormat,
    DatetimeFormat,
    Debug,
    DecimalChar,
    FalseValues,
    GroupChar,
    Json,
    KeepStrings,
    ListDelimiter,
    ListItemType,
    MissingValues,
    MonthFirst,
    SampleRows,
    TimeFormat,
    TrueValues,
    VariadicPaths,
)
from fairspec_terminal.program import dataset_program
from fairspec_terminal.session import Session


@dataset_program.command()
def infer(
    paths: VariadicPaths,
    json: Json = False,
    debug: Debug = False,
    sample_rows: SampleRows = None,
    confidence: Confidence = None,
    comma_decimal: CommaDecimal = False,
    month_first: MonthFirst = False,
    keep_strings: KeepStrings = False,
    column_types: ColumnTypes = None,
    missing_values: MissingValues = None,
    decimal_char: DecimalChar = None,
    group_char: GroupChar = None,
    true_values: TrueValues = None,
    false_values: FalseValues = None,
    datetime_format: DatetimeFormat = None,
    date_format: DateFormat = None,
    time_format: TimeFormat = None,
    array_type: ArrayType = None,
    list_delimiter: ListDelimiter = None,
    list_item_type: ListItemType = None,
) -> None:
    """Infer a dataset from local or remote file paths."""
    session = Session(debug=debug, json=json)

    def _infer() -> Dataset:
        dataset = Dataset(resources=[{"data": data} for data in paths])  # type: ignore[list-item]
        return infer_dataset(dataset)

    dataset = session.task("Inferring dataset", _infer)

    session.render_data_result(dataset.model_dump(exclude_none=True))
