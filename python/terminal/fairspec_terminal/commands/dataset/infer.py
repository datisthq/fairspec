from fairspec_library import infer_dataset
from fairspec_metadata import Dataset

from fairspec_terminal.helpers.table_schema import create_infer_table_schema_options
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
    options = create_infer_table_schema_options(
        sample_rows=sample_rows,
        confidence=confidence,
        comma_decimal=comma_decimal,
        month_first=month_first,
        keep_strings=keep_strings,
        column_types=column_types,
        missing_values=missing_values,
        decimal_char=decimal_char,
        group_char=group_char,
        true_values=true_values,
        false_values=false_values,
        datetime_format=datetime_format,
        date_format=date_format,
        time_format=time_format,
        array_type=array_type,
        list_delimiter=list_delimiter,
        list_item_type=list_item_type,
    )

    def _infer() -> Dataset:
        dataset = Dataset(resources=[{"data": data} for data in paths])  # type: ignore[list-item]
        return infer_dataset(dataset, **options)

    dataset = session.task("Inferring dataset", _infer)

    session.render_data_result(dataset.model_dump(exclude_none=True))
