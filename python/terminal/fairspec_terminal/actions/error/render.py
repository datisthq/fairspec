from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata import FairspecError


def render_error(error: FairspecError) -> str:
    match error.type:
        case "cell/type":
            return _render_cell_type_error(error)
        case "cell/missing":
            return _render_cell_missing_error(error)
        case "cell/minimum":
            return _render_cell_minimum_error(error)
        case "cell/maximum":
            return _render_cell_maximum_error(error)
        case "cell/exclusiveMinimum":
            return _render_cell_exclusive_minimum_error(error)
        case "cell/exclusiveMaximum":
            return _render_cell_exclusive_maximum_error(error)
        case "cell/multipleOf":
            return _render_cell_multiple_of_error(error)
        case "cell/minLength":
            return _render_cell_min_length_error(error)
        case "cell/maxLength":
            return _render_cell_max_length_error(error)
        case "cell/pattern":
            return _render_cell_pattern_error(error)
        case "cell/unique":
            return _render_cell_unique_error(error)
        case "cell/const":
            return _render_cell_const_error(error)
        case "cell/enum":
            return _render_cell_enum_error(error)
        case "cell/json":
            return _render_cell_json_error(error)
        case "cell/minItems":
            return _render_cell_min_items_error(error)
        case "cell/maxItems":
            return _render_cell_max_items_error(error)
        case "column/missing":
            return _render_column_missing_error(error)
        case "column/type":
            return _render_column_type_error(error)
        case "data":
            return _render_data_error(error)
        case "file/textual":
            return _render_textual_error(error)
        case "file/integrity":
            return _render_integrity_error(error)
        case "foreignKey":
            return _render_foreign_key_error(error)
        case "metadata":
            return _render_metadata_error(error)
        case "row/primaryKey":
            return _render_row_primary_key_error(error)
        case "row/uniqueKey":
            return _render_row_unique_key_error(error)
        case "resource/missing":
            return _render_resource_missing_error(error)
        case "resource/type":
            return _render_resource_type_error(error)
        case _:
            return str(error)


def _b(value: object) -> str:
    return f"[bold]{value}[/bold]"


def _in_resource(resource_name: str | None) -> str:
    return f" in resource {_b(resource_name)}" if resource_name else ""


def _render_cell_type_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is not {_b(error.columnType)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_missing_error(error: object) -> str:
    return f"A cell in column {_b(error.columnName)} of row {_b(error.rowNumber)} is missing{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_minimum_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is less than {_b(error.minimum)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_maximum_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is more than {_b(error.maximum)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_exclusive_minimum_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is less or equal to {_b(error.minimum)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_exclusive_maximum_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is greater or equal to {_b(error.maximum)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_multiple_of_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is not a multiple of {_b(error.multipleOf)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_min_length_error(error: object) -> str:
    return f"Length of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is less than {_b(error.minLength)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_max_length_error(error: object) -> str:
    return f"Length of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is more than {_b(error.maxLength)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_pattern_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} does not match the {_b(error.pattern)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_unique_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is not unique{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_const_error(error: object) -> str:
    const_val = getattr(error, "const", None)
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is not allowed value {_b(const_val)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_enum_error(error: object) -> str:
    enum_values = ", ".join(_b(v) for v in error.enum)  # type: ignore[union-attr]
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} is not in the allowed values {enum_values}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_json_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} violates JSON schema at {_b(error.jsonPointer)}: {error.message}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_min_items_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} has less than {_b(error.minItems)} items{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_cell_max_items_error(error: object) -> str:
    return f"Value of the cell {_b(error.cell)} in column {_b(error.columnName)} of row {_b(error.rowNumber)} has more than {_b(error.maxItems)} items{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_column_missing_error(error: object) -> str:
    return f"Required column {_b(error.columnName)} is missing{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_column_type_error(error: object) -> str:
    return f"Column {_b(error.columnName)} is expected to be {_b(error.expectedColumnType)}, but it is {_b(error.actualColumnType)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_data_error(error: object) -> str:
    return f"Data error at {_b(error.jsonPointer)}: {error.message}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_textual_error(error: object) -> str:
    actual_encoding = getattr(error, "actualEncoding", None)
    encoding_text = _b(actual_encoding) if actual_encoding else "binary"
    return f"File is expected to be textual with utf-8 encoding but it is {encoding_text}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_integrity_error(error: object) -> str:
    return f"File hash {_b(error.hashType)} is expected to be {_b(error.expectedHash)}, but it is {_b(error.actualHash)}){_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_foreign_key_error(error: object) -> str:
    cells = ", ".join(_b(c) for c in error.cells)  # type: ignore[union-attr]
    return f"Foreign key constraint violated as cells {cells} do not reference existing values{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_metadata_error(error: object) -> str:
    return f"{error.message} at {_b(error.jsonPointer)}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_row_primary_key_error(error: object) -> str:
    column_names = ", ".join(_b(c) for c in error.columnNames)  # type: ignore[union-attr]
    return f"Row {_b(error.rowNumber)} violates primary key constraint on columns {column_names}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_row_unique_key_error(error: object) -> str:
    column_names = ", ".join(_b(c) for c in error.columnNames)  # type: ignore[union-attr]
    return f"Row {_b(error.rowNumber)} violates unique key constraint on columns {column_names}{_in_resource(error.resourceName)}"  # type: ignore[union-attr]


def _render_resource_missing_error(error: object) -> str:
    in_ref = _in_resource(getattr(error, "referencingResourceName", None))
    return f"Resource {_b(error.resourceName)} is missing, but expected{in_ref}"  # type: ignore[union-attr]


def _render_resource_type_error(error: object) -> str:
    in_ref = _in_resource(getattr(error, "referencingResourceName", None))
    return f"Resource {_b(error.resourceName)} is expected to be {_b(error.expectedResourceType)}{in_ref}"  # type: ignore[union-attr]
