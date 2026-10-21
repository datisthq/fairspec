use std::num::NonZeroUsize;
use std::sync::{LazyLock, Mutex};

use lru::LruCache;

use super::assert::assert_json_schema;
use crate::actions::descriptor::load::load_descriptor;
use crate::actions::profile::registry::PROFILE_REGISTRY;
use crate::models::exception::Result;
use crate::models::json_schema::JsonSchema;

const METASCHEMA_ID: &str = "https://json-schema.org/draft/2020-12/schema";

static CACHE: LazyLock<Mutex<LruCache<(String, bool), JsonSchema>>> =
    LazyLock::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())));

pub fn load_json_schema(path: &str, only_remote: bool) -> Result<JsonSchema> {
    let key = (path.to_string(), only_remote);
    if let Some(json_schema) = CACHE.lock().unwrap().get(&key) {
        return Ok(json_schema.clone());
    }

    let json_schema = load_json_schema_uncached(path, only_remote)?;
    CACHE.lock().unwrap().put(key, json_schema.clone());
    Ok(json_schema)
}

fn load_json_schema_uncached(path: &str, only_remote: bool) -> Result<JsonSchema> {
    for item in PROFILE_REGISTRY.iter() {
        if item.path == path {
            return Ok(item.profile.clone());
        }
    }

    if path == METASCHEMA_ID
        && let Some(metaschema) = referencing::meta::DRAFT202012.as_object()
    {
        return Ok(metaschema.clone());
    }

    let descriptor = load_descriptor(path, only_remote)?;
    assert_json_schema(descriptor)
}
