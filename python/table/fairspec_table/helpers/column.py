from __future__ import annotations

from fairspec_metadata import (
    CategoricalColumn,
    IntegerCategoryItem,
    StringCategoryItem,
)


def get_categorical_values_and_labels(
    column: CategoricalColumn,
) -> tuple[list[str | int], list[str]]:
    values: list[str | int] = []
    labels: list[str] = []

    for item in column.property.categories or []:
        if isinstance(item, (IntegerCategoryItem, StringCategoryItem)):
            values.append(item.value)
            labels.append(item.label)
        else:
            values.append(item)
            labels.append(str(item))

    return values, labels
