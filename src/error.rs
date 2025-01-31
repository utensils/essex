use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Invalid project name: {0}")]
    InvalidProjectName(String),

    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    #[error("Task join error: {0}")]
    TaskJoinError(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    StripPrefix(#[from] std::path::StripPrefixError),

    #[error(transparent)]
    Tera(#[from] tera::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
