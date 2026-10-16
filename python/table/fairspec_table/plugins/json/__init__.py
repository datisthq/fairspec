from .actions.file_dialect.infer import infer_json_file_dialect
from .actions.table.load import load_json_table
from .actions.table.save import save_json_table
from .plugin import JsonPlugin

__all__ = [
    "JsonPlugin",
    "infer_json_file_dialect",
    "load_json_table",
    "save_json_table",
]
