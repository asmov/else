use serde;
use serde_with::{serde_as, Bytes};
use crate::identity::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Web3Blockchain {
    Solana,
    Ethereum
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum AuthRegisterMsg {
    Web3(Web3AuthRegisterRequest)
}

#[serde_as]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Web3AuthRegisterRequest {
    pub blockchain: Web3Blockchain,
    pub public_key: Bytes32,
    pub email_address: String,
    #[serde_as(as = "Bytes")]
    pub email_address_signature: Bytes64,
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum AuthRequestMsg {
    Web3(Web3AuthRequest)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum AuthChallengeMsg {
    Web3(Web3AuthChallenge)
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum AuthAnswerMsg {
    Web3(Web3AuthAnswer)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthorizedMsg {
    pub interface_uid: UID
}

type Bytes32 = [u8; 32];
type Bytes64 = [u8; 64];

/// The client sends its public key and a nonce for the server to sign
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Web3AuthRequest {
    pub blockchain: Web3Blockchain,
    pub client_public_key: Bytes32,
    /// Nonce, generated for each request
    pub client_challenge: Bytes32,
}

/// The server responds to a [SolanaAuthRequest] with its public key and the client's nonce signed.
/// The client will:
///   1. Verify the signature
///   2. Launch the wallet's signIn process (Sign-In With Solana SIWS)
#[serde_as]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Web3AuthChallenge {
    pub blockchain: Web3Blockchain,
    pub server_public_key: Bytes32,
    /// The result of the server signing the [SolanaAuthRequest::client_challenge]
    #[serde_as(as = "Bytes")]
    pub server_challenge: Bytes64
}

/// The client has successfully signed in with Sign-In With Solana (SIWS) using the [SolanaAuthChallenge::server_challenge]. 
/// The client answers with the server's challenge signed.
/// The server will:
///   1. Verify the signature.
///   1.a If the signature is invalid, the server will respond with [AuthResultMsg::Incorrect]
///   1.b If public key is no longer known, the server will respond with [AuthResultMsg::Unknown]
///   2. Start a session for the client's Interface
///   3. Return the UID of the interface using [AuthResultMsg::Authenticated]
#[serde_as]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Web3AuthAnswer {
    #[serde_as(as = "Bytes")]
    pub signature: Bytes64,
}