from __future__ import annotations

from fairspec_dataset import (
    CkanPlugin,
    DescriptorPlugin,
    FolderPlugin,
    GithubPlugin,
    ZenodoPlugin,
    ZipPlugin,
)
from fairspec_metadata import MetadataPlugin
from fairspec_table import (
    ArrowPlugin,
    CsvPlugin,
    InlinePlugin,
    JsonPlugin,
    ParquetPlugin,
    SqlitePlugin,
    XlsxPlugin,
)


class System:
    plugins: list[MetadataPlugin]

    def __init__(self) -> None:
        self.plugins = []

    def register(self, plugin_class: type[MetadataPlugin]) -> None:
        self.plugins.insert(0, plugin_class())


system = System()

system.register(CkanPlugin)
system.register(DescriptorPlugin)
system.register(GithubPlugin)
system.register(ZenodoPlugin)
system.register(FolderPlugin)
system.register(ZipPlugin)

system.register(ArrowPlugin)
system.register(CsvPlugin)
system.register(InlinePlugin)
system.register(JsonPlugin)
system.register(ParquetPlugin)
system.register(SqlitePlugin)
system.register(XlsxPlugin)
