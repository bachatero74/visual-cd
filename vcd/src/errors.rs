use std::io;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Application Error: {0}")]
    Str(String),

    #[error("Application Error: {0}")]
    StatStr(&'static str),

    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Log Error: {0}")]
    Log(#[from] log::SetLoggerError),
    // #[error("Yaml Error: {0}")]
    // Yaml(#[from] serde_yaml::Error),
}
