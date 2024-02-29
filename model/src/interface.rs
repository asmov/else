use serde;
use crate::{codebase::*, error::*, identity::*, modeling::*};


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
    pub fn device_name(&self) -> String {
        format!("/dev/tty/{:0>3}", Identity::from_uid(self.uid).id_to_string())
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceBuilder {
    builder_mode: BuilderMode,
    identity: Option<IdentityBuilder>
}

impl Builder for InterfaceBuilder {
    type BuilderType = Self;
    type ModelType = Interface;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            identity: None
        }
    }

    fn editor() -> Self {
        Self {
            builder_mode: BuilderMode::Editor,
            ..Self::creator()
        }
    }

    fn builder_mode(&self) -> BuilderMode {
        self.builder_mode
    }

    fn class_ident(&self) -> &'static ClassIdent {
        InterfaceField::class_ident_const()
    }

    fn create(mut self) -> Result<Creation<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);
        
        let uid = Build::create(&mut self.identity, &mut fields_changed, InterfaceField::UID)?.uid();

        let interface = Interface {
            uid
        };

        Ok(Creation::new(self, interface))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let fields_changed = Build::prepare_modify(&mut self, existing)?;

        Ok(Modification::new(self, fields_changed))
    }
}

impl MaybeIdentifiable for InterfaceBuilder {
    fn try_uid(&self) -> Result<UID> {
        self.identity
            .as_ref()
            .ok_or_else(|| Error::IdentityNotGenerated)
            .and_then(|identity| identity.try_uid())
    }
}

impl BuildableIdentity for InterfaceBuilder {
    fn identity(&mut self, identity: IdentityBuilder) -> Result<&mut Self> {
        assert!(self.builder_mode == BuilderMode::Creator);
        self.identity = Some(identity);
        Ok(self)
    }
    
    fn identity_builder(&mut self) -> &mut IdentityBuilder {
        assert!(self.builder_mode == BuilderMode::Creator);
        self.identity.get_or_insert_with(IdentityBuilder::creator)
    }
    
    fn get_identity(&self) -> Option<&IdentityBuilder> {
        self.identity.as_ref()
    }
}


/*todo
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
*/