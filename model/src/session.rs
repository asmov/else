use crate::{error::*, modeling::*, identity::*, codebase::*};

/// A Universe stores each active session as such
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Session {
    uid: UID,
    interface_uid: UID,
    // Timestamp of when the session expires
    expiration: u64,
}

impl Keyed for Session {}

impl Identifiable for Session {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Session {
    pub fn interface_uid(&self) -> UID {
        self.interface_uid
    }

    pub fn expiration(&self) -> u64 {
        self.expiration
    }
}

pub enum SessionField {
    UID,
    InterfaceUID,
    Expiration
}

impl Fields for SessionField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::InterfaceUID => &Self::FIELD_INTERFACE_UID,
            Self::Expiration => &Self::FIELD_EXPIRATION
        }
    }
}

impl Class for SessionField {
    fn class_ident() -> &'static ClassIdent {
        todo!()
    }
}

impl SessionField {
    const CLASSNAME: &'static str = "Session";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Session as ClassID, Self::CLASSNAME);
    const FIELDNAME_UID: &'static str = "uid";
    const FIELDNAME_INTERFACE_UID: &'static str = "interface_uid";
    const FIELDNAME_EXPIRATION: &'static str = "expiration";
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_INTERFACE_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INTERFACE_UID, FieldValueType::UID(&Self::CLASS_IDENT)); 
    const FIELD_EXPIRATION: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_EXPIRATION, FieldValueType::U64);

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl Built for Session {
    type BuilderType = SessionBuilder;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionBuilder {
    builder_mode: BuilderMode,
    uid: Option<UID>,
    interface_uid: Option<UID>,
    expiration: Option<u64>,
}

impl Builder for SessionBuilder {
    type BuilderType = Self;
    type ModelType = Session;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            uid: None,
            interface_uid: None,
            expiration: None,
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
        &SessionField::CLASS_IDENT
    }

    fn create(mut self) -> crate::Result<Creation<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        let uid = Build::create_uid(&mut self.uid, &mut fields_changed, SessionField::UID)?;
        let interface_uid = Build::create_uid(&mut self.interface_uid, &mut fields_changed, SessionField::InterfaceUID)?;
        let expiration = Build::create_value(&mut self.expiration, &mut fields_changed, SessionField::Expiration)?;

        let session = Session {
            uid,
            interface_uid,
            expiration
        };

        Ok(Creation::new(self, session))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> crate::Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify(&mut self, existing)?;

        //todo: error if interface_uid attempts to change
        Build::modify_value(&mut self.expiration, &mut existing.expiration, &mut fields_changed, SessionField::Expiration)?;

        Ok(Modification::new(self, fields_changed))
    }
}

impl MaybeIdentifiable for SessionBuilder {
    fn try_uid(&self) -> Result<UID> {
        self._try_uid()
    }
}

impl BuildableUID for SessionBuilder {
    fn uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.uid = Some(uid);
        Ok(self)
    }

    fn get_uid(&self) -> Option<&UID> {
        self.uid.as_ref()
    }
}