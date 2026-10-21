use fairspec_metadata::{Descriptor, Result, get_is_remote_path};

use super::actions::dataset::load::load_dataset_from_github;
use crate::plugin::{DatasetPlugin, MetadataPlugin};

#[derive(Debug, Clone, Copy, Default)]
pub struct GithubPlugin;

impl MetadataPlugin for GithubPlugin {}

impl DatasetPlugin for GithubPlugin {
    fn load_dataset(&self, source: &str) -> Result<Option<Descriptor>> {
        if !get_is_github(source) {
            return Ok(None);
        }
        load_dataset_from_github(source, None).map(Some)
    }
}

pub(crate) fn get_is_github(path: &str) -> bool {
    get_is_remote_path(path)
        && url::Url::parse(path).is_ok_and(|url| url.host_str() == Some("github.com"))
}
