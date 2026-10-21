use fairspec_metadata::{Descriptor, Result, get_is_remote_path};

use super::actions::dataset::load::load_dataset_from_ckan;
use crate::plugin::{DatasetPlugin, MetadataPlugin};

#[derive(Debug, Clone, Copy, Default)]
pub struct CkanPlugin;

impl MetadataPlugin for CkanPlugin {}

impl DatasetPlugin for CkanPlugin {
    fn load_dataset(&self, source: &str) -> Result<Option<Descriptor>> {
        if !get_is_ckan(source) {
            return Ok(None);
        }
        load_dataset_from_ckan(source, None).map(Some)
    }
}

pub(crate) fn get_is_ckan(path: &str) -> bool {
    get_is_remote_path(path) && path.contains("/dataset/")
}
