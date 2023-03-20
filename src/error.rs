use clap::error::ErrorKind;
use inflector::Inflector;

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Internal io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Internal error: no name in validation")]
    NoValidationName,
    #[error("Internal match error: {0}")]
    MatchError(clap::Error),
    #[error("Internal error: no child stdout or stderr")]
    NoStdoutOrStderr,
    #[error("Validation error in {}: '{}'", .name, .message)]
    ValidationError { name: String, message: String },
    #[error("{0}")]
    GuiError(String),
}

impl From<clap::Error> for ExecutionError {
    fn from(err: clap::Error) -> Self {
        match clap::Error::kind(&err) {
            ErrorKind::ValueValidation => {
                    Self::ValidationError {
                        name: "idk".to_string(),
                        message: err.to_string()
                    }
            }
            _ => Self::MatchError(err),
        }
    }
}

impl From<String> for ExecutionError {
    fn from(str: String) -> Self {
        Self::GuiError(str)
    }
}

impl From<&str> for ExecutionError {
    fn from(str: &str) -> Self {
        Self::GuiError(str.to_string())
    }
}
