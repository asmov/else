use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("This is a testing error")]
    Test,
    #[error("{0}")]
    Generic(String),
    #[error("Unable to parse text input: {0}")]
    InputParsing(String),
    #[error("Unknown command: {0}")]
    UnknownCommand(String)
}

pub type Result<T> = core::result::Result<T, Error>;