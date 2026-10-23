use std::cell::RefCell;
use std::io::{self, IsTerminal, Write};
use std::rc::Rc;

/// Where a command writes: the process's stdout and stderr, or buffers in tests.
#[derive(Clone)]
pub struct Console {
    stdout: Rc<RefCell<Box<dyn Write>>>,
    stderr: Rc<RefCell<Box<dyn Write>>>,
    pub is_terminal: bool,
}

impl Console {
    pub fn new() -> Self {
        Console {
            stdout: Rc::new(RefCell::new(Box::new(io::stdout()))),
            stderr: Rc::new(RefCell::new(Box::new(io::stderr()))),
            is_terminal: io::stdout().is_terminal(),
        }
    }

    pub fn from_writers(stdout: Box<dyn Write>, stderr: Box<dyn Write>) -> Self {
        Console {
            stdout: Rc::new(RefCell::new(stdout)),
            stderr: Rc::new(RefCell::new(stderr)),
            is_terminal: false,
        }
    }

    pub fn write_out(&self, text: &str) {
        let mut stdout = self.stdout.borrow_mut();
        let _ = stdout.write_all(text.as_bytes());
        let _ = stdout.flush();
    }

    pub fn write_err(&self, text: &str) {
        let mut stderr = self.stderr.borrow_mut();
        let _ = stderr.write_all(text.as_bytes());
        let _ = stderr.flush();
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}
