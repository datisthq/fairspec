use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, Once};

static PATHS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
static REGISTER: Once = Once::new();

/// Deletes a temporary file or folder when the process exits, like Python's `atexit`.
pub fn register_cleanup(path: PathBuf) {
    REGISTER.call_once(|| {
        // SAFETY: `cleanup_paths` is a plain `extern "C"` function that never unwinds
        unsafe {
            libc::atexit(cleanup_paths);
        }
    });
    if let Ok(mut paths) = PATHS.lock() {
        paths.push(path);
    }
}

extern "C" fn cleanup_paths() {
    let Ok(mut paths) = PATHS.lock() else {
        return;
    };
    for path in paths.drain(..) {
        if path.is_dir() {
            let _ = fs::remove_dir_all(&path);
        } else {
            let _ = fs::remove_file(&path);
        }
    }
}
