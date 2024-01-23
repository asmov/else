use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Generic(String)
}