use std::fmt;

#[derive(Debug)]
pub enum CraiteError {
    Io(std::io::Error),
    Database(rusqlite::Error),
    Scanner(String),
    Classifier(String),
    Audio(String),
}

impl fmt::Display for CraiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CraiteError::Io(err) => write!(f, "IO error: {err}"),
            CraiteError::Database(err) => write!(f, "Database error: {err}"),
            CraiteError::Scanner(msg) => write!(f, "Scanner error: {msg}"),
            CraiteError::Classifier(msg) => write!(f, "Classifier error: {msg}"),
            CraiteError::Audio(msg) => write!(f, "Audio error: {msg}"),
        }
    }
}

impl std::error::Error for CraiteError {}

impl From<std::io::Error> for CraiteError {
    fn from(err: std::io::Error) -> Self {
        CraiteError::Io(err)
    }
}

impl From<rusqlite::Error> for CraiteError {
    fn from(err: rusqlite::Error) -> Self {
        CraiteError::Database(err)
    }
}

impl From<CraiteError> for String {
    fn from(err: CraiteError) -> Self {
        err.to_string()
    }
}
