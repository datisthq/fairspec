from fairspec_metadata.models.base import FairspecModel

from .column import SqliteColumn


class SqliteSchema(FairspecModel):
    name: str
    columns: list[SqliteColumn]
    isView: bool = False
    primaryKey: list[str] | None = None
