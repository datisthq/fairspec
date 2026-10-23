use fairspec_library::fairspec_dataset::preview_file;
use fairspec_library::fairspec_metadata::Result;
use serde::{Deserialize, Serialize};

use crate::tools::_shared::schemas::check_range;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewFileInput {
    pub path: String,
    #[serde(default)]
    pub max_lines: Option<usize>,
    #[serde(default)]
    pub max_bytes: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreviewFileResult {
    pub text: String,
    pub truncated: bool,
}

/// Preview the raw source of a local or remote file as UTF-8 text.
pub fn preview_file_tool(input: PreviewFileInput) -> Result<PreviewFileResult> {
    check_range("maxLines", input.max_lines, 1, 200)?;
    check_range("maxBytes", input.max_bytes, 1, 20_000)?;
    let preview = preview_file(&input.path, input.max_bytes, input.max_lines)?;
    Ok(PreviewFileResult {
        text: preview.text,
        truncated: preview.truncated,
    })
}
