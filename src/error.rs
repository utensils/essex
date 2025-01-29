use thiserror::Error;

#[derive(Error, Debug)]
pub enum EssexError {
    #[error("Template error: {0}")]
    TemplateError(#[from] tera::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid project name: {0}")]
    InvalidProjectName(String),
    
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Project directory error: {0}")]
    ProjectDirectoryError(String),
}

pub type Result<T> = std::result::Result<T, EssexError>;