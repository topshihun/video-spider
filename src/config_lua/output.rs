use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum OutputTarget {
    Stdout,
    Stderr,
    File(File),
    Buffer(Arc<Mutex<Vec<u8>>>),
}

impl Write for OutputTarget {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            OutputTarget::Stdout => std::io::stdout().write(buf),
            OutputTarget::Stderr => std::io::stderr().write(buf),
            OutputTarget::File(file) => file.write(buf),
            OutputTarget::Buffer(buffer) => buffer.lock().unwrap().write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            OutputTarget::Stdout => std::io::stdout().flush(),
            OutputTarget::Stderr => std::io::stderr().flush(),
            OutputTarget::File(file) => file.flush(),
            OutputTarget::Buffer(buffer) => buffer.lock().unwrap().flush(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Output(Arc<Mutex<OutputTarget>>);

impl Output {
    fn new(target: OutputTarget) -> Self {
        Self(Arc::new(Mutex::new(target)))
    }

    pub fn stdout() -> Self {
        Self::new(OutputTarget::Stdout)
    }

    pub fn stderr() -> Self {
        Self::new(OutputTarget::Stderr)
    }

    pub fn file(file: File) -> Self {
        Self::new(OutputTarget::File(file))
    }

    pub fn buffer(vec: Arc<Mutex<Vec<u8>>>) -> Self {
        Self::new(OutputTarget::Buffer(vec))
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut output_target = self.0.lock().unwrap();
        output_target.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut output_target = self.0.lock().unwrap();
        output_target.flush()
    }
}

impl Deref for Output {
    type Target = Arc<Mutex<OutputTarget>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
