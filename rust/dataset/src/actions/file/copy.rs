use fairspec_metadata::Result;

use crate::actions::stream::load::load_file_stream;
use crate::actions::stream::save::save_file_stream;

pub fn copy_file(source_path: &str, target_path: &str, max_bytes: Option<usize>) -> Result<()> {
    let stream = load_file_stream(source_path, None, max_bytes)?;
    save_file_stream(stream, target_path, false)
}
