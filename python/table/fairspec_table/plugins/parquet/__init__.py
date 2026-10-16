from .actions.table.load import load_parquet_table
from .actions.table.save import save_parquet_table
from .plugin import ParquetPlugin

__all__ = [
    "ParquetPlugin",
    "load_parquet_table",
    "save_parquet_table",
]
