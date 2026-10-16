import {
  CkanPlugin,
  DescriptorPlugin,
  FolderPlugin,
  GithubPlugin,
  ZenodoPlugin,
  ZipPlugin,
} from "@fairspec/dataset"
import { ExtensionPlugin } from "@fairspec/extension"
import {
  ArrowPlugin,
  CsvPlugin,
  InlinePlugin,
  JsonPlugin,
  ParquetPlugin,
  SqlitePlugin,
  XlsxPlugin,
} from "@fairspec/table"
import type { Plugin } from "./plugin.ts"

export class System {
  plugins: Plugin[] = []

  register(PluginClass: new () => Plugin) {
    this.plugins.unshift(new PluginClass())
  }
}

export const system = new System()

// Extension

system.register(ExtensionPlugin)

// Dataset

system.register(CkanPlugin)
system.register(DescriptorPlugin)
system.register(GithubPlugin)
system.register(ZenodoPlugin)
system.register(FolderPlugin)
system.register(ZipPlugin)

// Table

system.register(ArrowPlugin)
system.register(CsvPlugin)
system.register(InlinePlugin)
system.register(JsonPlugin)
system.register(ParquetPlugin)
system.register(SqlitePlugin)
system.register(XlsxPlugin)
