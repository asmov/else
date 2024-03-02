use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("This is a testing error")]
    Test,
    #[error("{0}")]
    Generic(String),
    #[error("Unable to parse text input: {0}")]
    GenericInputParsing(String),
    #[error("Unable to parse text input: {text} :> {cause}")]
    InputParsing{ text: String, cause: String},
    #[error("{usage}")]
    CommandUsage{usage: &'static str},
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    #[error("{target} not found: {search}")]
    TargetNotFound { target: &'static str, search: String },


}

pub type Result<T> = core::result::Result<T, Error>;