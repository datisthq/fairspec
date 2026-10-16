from fairspec_metadata.models.base import FairspecModel


class SqliteColumn(FairspecModel):
    name: str
    dataType: str
    isNullable: bool = True
    comment: str | None = None
    isAutoIncrementing: bool = False
    hasDefaultValue: bool = False
