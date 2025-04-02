use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("system error: {0}")]
    SystemError(String),

    #[error("error message: {0}")]
    ErrorMessage(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
