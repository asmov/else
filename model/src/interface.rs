use serde;
use crate::identity::*;


#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct InterfaceID {
    universe_id: UniverseID,
    serial: u64,
}

impl InterfaceID {
    pub fn new(universe_id: UniverseID, serial: u64) -> Self {
        Self {
            universe_id,
            serial,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Interface {
    interface_id: InterfaceID,
    tty_name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct InterfaceContact {
    interface_id: InterfaceID,
    contacts: Vec<Contact>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct InterfaceLogin {
    interface_id: InterfaceID,
    login_accounts: Vec<Login>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum Login{
    Web3(Web3Login),
    Google(GoogleLogin),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Web3Login{
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct GoogleLogin{
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum Contact{
    Email(EmailContact)
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct EmailContact {

}