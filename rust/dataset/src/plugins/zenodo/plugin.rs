use fairspec_metadata::{Descriptor, Result, get_is_remote_path};

use super::actions::dataset::load::load_dataset_from_zenodo;
use crate::plugin::{DatasetPlugin, MetadataPlugin};

#[derive(Debug, Clone, Copy, Default)]
pub struct ZenodoPlugin;

impl MetadataPlugin for ZenodoPlugin {}

impl DatasetPlugin for ZenodoPlugin {
    fn load_dataset(&self, source: &str) -> Result<Option<Descriptor>> {
        if !get_is_zenodo(source) {
            return Ok(None);
        }
        load_dataset_from_zenodo(source, None).map(Some)
    }
}

pub(crate) fn get_is_zenodo(path: &str) -> bool {
    get_is_remote_path(path)
        && url::Url::parse(path)
            .is_ok_and(|url| url.host_str().unwrap_or_default().ends_with("zenodo.org"))
}
