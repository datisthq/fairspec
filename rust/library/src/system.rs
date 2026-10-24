use std::sync::{LazyLock, PoisonError, RwLock, RwLockReadGuard};

use fairspec_dataset::{
    CkanPlugin, DatasetPlugin, DescriptorPlugin, FolderPlugin, GithubPlugin, MetadataPlugin,
    ZenodoPlugin, ZipPlugin,
};
use fairspec_table::{
    ArrowPlugin, CsvPlugin, InlinePlugin, JsonPlugin, ParquetPlugin, SqlitePlugin, TablePlugin,
    XlsxPlugin,
};

/// A registered plugin, by the widest trait it implements: Python's `isinstance` checks.
pub enum SystemPlugin {
    Metadata(Box<dyn MetadataPlugin>),
    Dataset(Box<dyn DatasetPlugin>),
    Table(Box<dyn TablePlugin>),
}

impl SystemPlugin {
    pub fn get_metadata_plugin(&self) -> &dyn MetadataPlugin {
        match self {
            SystemPlugin::Metadata(plugin) => plugin.as_ref(),
            SystemPlugin::Dataset(plugin) => plugin.as_ref(),
            SystemPlugin::Table(plugin) => plugin.as_ref(),
        }
    }

    pub fn get_dataset_plugin(&self) -> Option<&dyn DatasetPlugin> {
        match self {
            SystemPlugin::Metadata(_) => None,
            SystemPlugin::Dataset(plugin) => Some(plugin.as_ref()),
            SystemPlugin::Table(plugin) => Some(plugin.as_ref()),
        }
    }

    pub fn get_table_plugin(&self) -> Option<&dyn TablePlugin> {
        match self {
            SystemPlugin::Table(plugin) => Some(plugin.as_ref()),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct System {
    pub plugins: Vec<SystemPlugin>,
}

impl System {
    /// A later plugin is asked first.
    pub fn register(&mut self, plugin: SystemPlugin) {
        self.plugins.insert(0, plugin);
    }
}

/// Python's `system`, the registry every action asks in order.
pub static SYSTEM: LazyLock<RwLock<System>> = LazyLock::new(|| {
    let mut system = System::default();
    system.register(SystemPlugin::Dataset(Box::new(CkanPlugin)));
    system.register(SystemPlugin::Dataset(Box::new(DescriptorPlugin)));
    system.register(SystemPlugin::Dataset(Box::new(GithubPlugin)));
    system.register(SystemPlugin::Dataset(Box::new(ZenodoPlugin)));
    system.register(SystemPlugin::Dataset(Box::new(FolderPlugin)));
    system.register(SystemPlugin::Dataset(Box::new(ZipPlugin)));
    system.register(SystemPlugin::Table(Box::new(ArrowPlugin)));
    system.register(SystemPlugin::Table(Box::new(CsvPlugin)));
    system.register(SystemPlugin::Table(Box::new(InlinePlugin)));
    system.register(SystemPlugin::Table(Box::new(JsonPlugin)));
    system.register(SystemPlugin::Table(Box::new(ParquetPlugin)));
    system.register(SystemPlugin::Table(Box::new(SqlitePlugin)));
    system.register(SystemPlugin::Table(Box::new(XlsxPlugin)));
    RwLock::new(system)
});

pub(crate) fn get_system() -> RwLockReadGuard<'static, System> {
    SYSTEM.read().unwrap_or_else(PoisonError::into_inner)
}
