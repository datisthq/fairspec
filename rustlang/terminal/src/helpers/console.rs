use std::io::{self, IsTerminal, Write};
use std::sync::{Arc, Mutex};

type SharedWriter = Arc<Mutex<Box<dyn Write + Send>>>;

/// Where a command writes: the process's stdout and stderr, or buffers in tests.
#[derive(Clone)]
pub struct Console {
    stdout: SharedWriter,
    stderr: SharedWriter,
    pub is_terminal: bool,
}

impl Console {
    pub fn new() -> Self {
        Console {
            stdout: Arc::new(Mutex::new(Box::new(io::stdout()))),
            stderr: Arc::new(Mutex::new(Box::new(io::stderr()))),
            is_terminal: io::stdout().is_terminal(),
        }
    }

    pub fn from_writers(stdout: Box<dyn Write + Send>, stderr: Box<dyn Write + Send>) -> Self {
        Console {
            stdout: Arc::new(Mutex::new(stdout)),
            stderr: Arc::new(Mutex::new(stderr)),
            is_terminal: false,
        }
    }

    pub fn write_out(&self, text: &str) {
        write_shared(&self.stdout, text.as_bytes());
    }

    pub fn write_err(&self, text: &str) {
        write_shared(&self.stderr, text.as_bytes());
    }

    /// The stdout and stderr writers fireargs prints help, manifests and results to.
    pub fn create_writers(&self) -> (ConsoleWriter, ConsoleWriter) {
        (
            ConsoleWriter(Arc::clone(&self.stdout)),
            ConsoleWriter(Arc::clone(&self.stderr)),
        )
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConsoleWriter(SharedWriter);

impl Write for ConsoleWriter {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        write_shared(&self.0, data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn write_shared(writer: &SharedWriter, data: &[u8]) {
    let mut writer = writer.lock().unwrap_or_else(|error| error.into_inner());
    let _ = writer.write_all(data);
    let _ = writer.flush();
}
