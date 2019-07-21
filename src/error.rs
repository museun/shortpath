#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub inner: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}. inner: {}", self.kind, self.inner)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidDirectory(String),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidDirectory(dir) => write!(f, "invalid directory: `{}`", dir),
        }
    }
}
