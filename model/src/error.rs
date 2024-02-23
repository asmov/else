use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown field for `{class}`: {field}")]
    UnknownField{ class: &'static str, field: String },

    #[error("Required field for `{class}` was not set: {field}")]
    FieldNotSet{ class: &'static str, field: &'static str},

    #[error("Required field for `{class}` was not set before `{field}`: {required_field}")]
    FieldNotSetFirst{ class: &'static str, field: &'static str, required_field: &'static str},

    #[error("{model} not found: {uid}")]
    ModelNotFound{ model: &'static str, uid: u128 },

    #[error("{model} not found for {op}: {uid}")]
    ModelNotFoundFor{ model: &'static str, uid: u128, op: &'static str },

    //todo: clean this up
    #[error("Buildable UID not available")]
    BuildableUID{},

    #[error("Unable to parse {what}: {src}")]
    ParseError{ what: &'static str, src: String }
}

pub type Result<T> = ::core::result::Result<T, Error>;