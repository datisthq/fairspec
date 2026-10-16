from .actions.file_dialect.infer import infer_xlsx_file_dialect
from .actions.table.load import load_xlsx_table
from .actions.table.save import save_xlsx_table
from .plugin import XlsxPlugin

__all__ = [
    "XlsxPlugin",
    "infer_xlsx_file_dialect",
    "load_xlsx_table",
    "save_xlsx_table",
]
