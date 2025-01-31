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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_error_display() {
        // Test IoError
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = Error::IoError(io_error);
        assert!(error.to_string().contains("IO error: file not found"));

        // Test TemplateError
        let error = Error::TemplateError("invalid syntax".to_string());
        assert!(error.to_string().contains("Template error: invalid syntax"));

        // Test TemplateNotFound
        let error = Error::TemplateNotFound("basic".to_string());
        assert!(error.to_string().contains("Template not found: basic"));

        // Test InvalidTemplate
        let error = Error::InvalidTemplate("missing field".to_string());
        assert!(error.to_string().contains("Invalid template: missing field"));

        // Test InvalidPath
        let error = Error::InvalidPath("invalid/path".to_string());
        assert!(error.to_string().contains("Invalid path: invalid/path"));

        // Test InvalidProjectName
        let error = Error::InvalidProjectName("invalid name".to_string());
        assert!(error.to_string().contains("Invalid project name: invalid name"));
    }

    #[test]
    fn test_error_conversions() {
        // Test From<std::io::Error>
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error: Error = io_error.into();
        assert!(matches!(error, Error::IoError(_)));

        // Test From<tera::Error>
        let tera_error = tera::Error::msg("template error");
        let error: Error = tera_error.into();
        assert!(matches!(error, Error::TemplateError(_)));

        // Test From<std::path::StripPrefixError>
        let path = Path::new("/a/b/c");
        let base = Path::new("/x/y/z");
        let strip_error = path.strip_prefix(base).unwrap_err();
        let error: Error = strip_error.into();
        assert!(matches!(error, Error::InvalidPath(_)));
    }
}
