use fairspec_metadata::Result;

use crate::actions::stream::save::save_file_stream;

pub fn save_file(path: &str, data: &[u8], overwrite: bool) -> Result<()> {
    save_file_stream(data, path, overwrite)
}
