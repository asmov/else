use serde;
use crate::identity::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Auth {
    Solana(AuthRequestMsg, AuthResponseMsg, AuthAnswerMsg),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum AuthRequestMsg {
    Solana(SolanaAuthRequest)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum AuthResponseMsg {
    Unknown,
    Solana(SolanaAuthChallenge)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum AuthAnswerMsg {
    Solana(SolanaAuthAnswer)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum AuthResultMsg {
    Unknown,
    Incorrect,
    Authenticated(UID)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SolanaAuthRequest {
    pub public_key_hash: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SolanaAuthChallenge {
    pub challenge: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SolanaAuthAnswer {
    pub public_key: String,
    pub signature: String,
}