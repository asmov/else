use serde;
use crate::{codebase::*, error::*, identity::*, modeling::*, Thing};


#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Interface {
    uid: UID,
    downlink_uid: Option<UID>,
}

impl Keyed for Interface{}

impl Identifiable for Interface {
    fn uid(&self) -> UID {
        self.uid
    }
}

impl Built for Interface {
    type BuilderType = InterfaceBuilder;
}

impl Interface {
    pub fn device_name(&self) -> String {
        format!("/dev/tty/{:0>3}", Identity::base58(self.uid))
    }

    pub fn downlink_uid(&self) -> Option<UID> {
        self.downlink_uid
    }

    pub fn linked(&self) -> bool {
        self.downlink_uid.is_some()
    }
}

pub enum InterfaceField {
    UID,
    DownlinkUID
}

impl Fields for InterfaceField {
    fn field(&self) -> &'static Field {
        match self {
            Self::UID => &Self::FIELD_UID,
            Self::DownlinkUID => &Self::FIELD_DOWNLINK_UID
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
    const FIELDNAME_DOWNLINK_UID: &'static str = "downlink_uid";
    const FIELD_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_UID, FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_DOWNLINK_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_DOWNLINK_UID, FieldValueType::UID(Thing::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceBuilder {
    builder_mode: BuilderMode,
    uid: Option<UID>,
    downlink_uid: OptionOp<UID>
}

impl Builder for InterfaceBuilder {
    type BuilderType = Self;
    type ModelType = Interface;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            uid: None,
            downlink_uid: OptionOp::None
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
        
        let uid = Build::create_uid(&mut self.uid, &mut fields_changed, InterfaceField::UID)?;
        let downlink_uid = Build::create_uid_option(&mut self.downlink_uid, &mut fields_changed, InterfaceField::DownlinkUID)?;

        let interface = Interface {
            uid,
            downlink_uid
        };

        Ok(Creation::new(self, interface))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify(&mut self, existing)?;

        Build::modify_uid_option(&mut self.downlink_uid, &mut existing.downlink_uid, &mut fields_changed, InterfaceField::DownlinkUID)?;

        Ok(Modification::new(self, fields_changed))
    }
}

impl MaybeIdentifiable for InterfaceBuilder {
    fn try_uid(&self) -> Result<UID> {
        Self::_try_uid(&self)
    }
}

impl BuildableUID for InterfaceBuilder {
    fn uid(&mut self, uid: UID) -> Result<&mut Self> {
        self.uid = Some(uid);
        Ok(self)
    }

    fn get_uid(&self) -> Option<&UID> {
        self.uid.as_ref()
    }
}

impl InterfaceBuilder {
    pub fn downlink_uid(&mut self, downlink_uid: UID) -> Result<&mut Self> {
        self.downlink_uid = OptionOp::Set(downlink_uid);
        Ok(self)
    }

    pub fn unset_downlink_uid(&mut self) -> Result<&mut Self> {
        self.downlink_uid = OptionOp::Unset;
        Ok(self)
    }

    pub fn get_downlink_uid_op(&self) -> OptionOp<&UID> {
        self.downlink_uid.as_ref()
    }
}

pub trait BuildableInterfaceList {
    fn add_interface(&mut self, interface: InterfaceBuilder) -> Result<&mut Self>;
    fn edit_interface(&mut self, interface: InterfaceBuilder) -> Result<&mut Self>;
    fn remove_interface(&mut self, interface_uid: UID) -> Result<&mut Self>;
}
