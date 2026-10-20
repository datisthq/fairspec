import os

os.environ.setdefault("GX_ANALYTICS_ENABLED", "false")

import great_expectations as gx
import great_expectations.expectations as gxe
import pandas as pd
from common import BOOLEAN_PATTERN, Schema, load_schema, print_result, read_arguments


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = load_schema(schema_path)
    frame = pd.read_csv(csv_path, dtype=str, keep_default_na=False, na_values=[""])
    for column in schema.columns:
        if column.kind in ("integer", "number"):
            frame[column.name] = pd.to_numeric(frame[column.name], errors="coerce")

    context = gx.get_context(mode="ephemeral")
    asset = context.data_sources.add_pandas("pandas").add_dataframe_asset("table")
    batch = asset.add_batch_definition_whole_dataframe("batch").get_batch(
        batch_parameters={"dataframe": frame}
    )
    suite = context.suites.add(create_suite(schema))
    result = batch.validate(suite, result_format="BASIC")
    print_result(
        sum(item.result.get("unexpected_count", 0) or 0 for item in result.results)
    )


def create_suite(schema: Schema) -> gx.ExpectationSuite:
    suite = gx.ExpectationSuite(name="suite")
    for column in schema.columns:
        name = column.name
        if not column.nullable:
            suite.add_expectation(gxe.ExpectColumnValuesToNotBeNull(column=name))
        if column.minimum is not None or column.maximum is not None:
            suite.add_expectation(
                gxe.ExpectColumnValuesToBeBetween(
                    column=name, min_value=column.minimum, max_value=column.maximum
                )
            )
        if column.enum is not None:
            suite.add_expectation(
                gxe.ExpectColumnValuesToBeInSet(column=name, value_set=column.enum)
            )
        if column.pattern is not None:
            suite.add_expectation(
                gxe.ExpectColumnValuesToMatchRegex(column=name, regex=column.pattern)
            )
        if column.min_length is not None:
            suite.add_expectation(
                gxe.ExpectColumnValueLengthsToBeBetween(
                    column=name, min_value=column.min_length
                )
            )
        if column.kind == "date-time":
            suite.add_expectation(
                gxe.ExpectColumnValuesToMatchStrftimeFormat(
                    column=name, strftime_format="%Y-%m-%dT%H:%M:%S"
                )
            )
        if column.kind == "boolean":
            suite.add_expectation(
                gxe.ExpectColumnValuesToMatchRegex(column=name, regex=BOOLEAN_PATTERN)
            )
    for name in schema.primary_key:
        suite.add_expectation(gxe.ExpectColumnValuesToBeUnique(column=name))
    return suite


if __name__ == "__main__":
    main()
