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

/// Extension trait to convert any error into a String result.
/// Eliminates repetitive `.map_err(|e| e.to_string())` patterns.
pub trait ResultExt<T> {
    fn str_err(self) -> Result<T, String>;
}

impl<T, E: fmt::Display> ResultExt<T> for Result<T, E> {
    fn str_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

/// Run a blocking closure on a dedicated thread.
/// Wraps `tokio::task::spawn_blocking` with automatic error mapping.
pub async fn run_blocking<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce() -> Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f).await.str_err()?
}
