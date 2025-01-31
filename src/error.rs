pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    IoError(std::io::Error),
    TemplateError(String),
    TemplateNotFound(String),
    InvalidTemplate(String),
    InvalidPath(String),
    InvalidProjectName(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::TemplateError(e) => write!(f, "Template error: {}", e),
            Error::TemplateNotFound(e) => write!(f, "Template not found: {}", e),
            Error::InvalidTemplate(e) => write!(f, "Invalid template: {}", e),
            Error::InvalidPath(e) => write!(f, "Invalid path: {}", e),
            Error::InvalidProjectName(e) => write!(f, "Invalid project name: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<tera::Error> for Error {
    fn from(error: tera::Error) -> Self {
        Error::TemplateError(error.to_string())
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(error: std::path::StripPrefixError) -> Self {
        Error::InvalidPath(error.to_string())
    }
}
