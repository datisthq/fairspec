from fairspec_table.models.table import LoadTableOptions


class ValidateTableOptions(LoadTableOptions, total=False):
    noInfer: bool
    maxErrors: int
