use serde;
use crate::{error::*, modeling::*, identity::*, codebase::*};


#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Interface {
    uid: UID,
}

impl Keyed for Interface{}

impl Identifiable for Interface {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Interface {
    fn id_to_tty(&self) -> String {
        Identity::from_uid(self.uid).id_to_string()
    }
}

pub enum InterfaceField {
    UID
}

impl Fields for InterfaceField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID
        }
    }
}

impl Class for InterfaceField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl InterfaceField {
    const CLASSNAME: &'static str = "Interface";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Interface as ClassID, Self::CLASSNAME);
    const FIELDNAME_UID: &'static str = "uid";
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::UID(&Self::CLASS_IDENT));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct InterfaceContact {
    interface_uid: UID,
    contacts: Vec<Contact>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct InterfaceLogin {
    interface_uid: UID,
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