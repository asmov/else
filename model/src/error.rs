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

    #[error("{model} not found: {key}")]
    ModelKeyNotFound{ model: &'static str, key: String },

    #[error("{model} not found for {op}: {uid}")]
    ModelNotFoundFor{ model: &'static str, uid: u128, op: &'static str },

    #[error("Identity has not been generated")]
    IdentityNotGenerated,

    #[error("Illegal add operation for {model} list ({uid}) in {context}")]
    IllegalAddOp{ model: &'static str, uid: u128, context: &'static str},

    /// "Unable to {op} {model} ({uid}) while its being {whiled}"
    #[error("Unable to {op} {model} ({uid}) while its being {whiled}")]
    ListOpRace{ op: &'static str, model: &'static str, uid: u128, whiled: &'static str},

    #[error("Illegal edit operation for {model} list ({uid}) in {context}")]
    IllegalEditOp{ model: &'static str, uid: u128, context: &'static str},

    #[error("Illegal remove operation for {model} list ({uid}) in {context}")]
    IllegalRemoveOp{ model: &'static str, uid: u128, context: &'static str},

    #[error("Unable to parse {what}: {src}")]
    Parsing { what: &'static str, src: String },

    #[error("Unable to link Interface ({interface_uid}) to Character ({character_uid}) :> Interface has existing link: {linked_character_uid}")]
    InterfaceAlreadyLinked{interface_uid: u128, character_uid: u128, linked_character_uid: u128},

    #[error("Unable to link Interface ({interface_uid}) to Character ({character_uid}) :> Character has existing link: {linked_interface_uid}")]
    CharacterAlreadyLinked{interface_uid: u128, character_uid: u128, linked_interface_uid: u128},

    #[error("Unexpected model type. Expected: {expected}, Found: {found}")]
    UnexpectedModelType{expected: &'static str, found: &'static str},

    #[error("Unable to authenticate")]
    AuthenticationFailed
}

pub type Result<T> = ::core::result::Result<T, Error>;