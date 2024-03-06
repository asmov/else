use crate::{codebase::*, error::*, identity::*, modeling::*, cortex::*, interface::*};
use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IntelligenceLobe {
    interface_uid: UID,
}

/// Composition trait for IntelligenceLobe
pub trait Intelligent {
    fn intelligence_lobe(&self) -> Option<&IntelligenceLobe>;

    fn intelligent_interface_uid(&self) -> Option<UID> {
        self.intelligence_lobe()
            .map(|lobe| lobe.interface_uid())
    }
}


impl Intelligent for IntelligenceLobe{
    fn intelligence_lobe(&self) -> Option<&IntelligenceLobe> {
        Some(&self)
    }
}

impl Built for IntelligenceLobe {
    type BuilderType = IntelligenceLobeBuilder;
}

impl IntelligenceLobe {
    pub fn interface_uid(&self) -> UID {
        self.interface_uid
    }
}

#[derive(Debug)]
pub enum IntelligenceLobeField {
    InterfaceUID,
    RoutineUID,
    RoutineAwareness
}

impl Fields for IntelligenceLobeField {
    fn field(&self) -> &'static Field {
        match self {
            Self::InterfaceUID => &Self::FIELD_INTERFACE_UID,
            Self::RoutineUID => &Self::FIELD_ROUTINE_UID,
            Self::RoutineAwareness => &Self::FIELD_ROUTINE_AWARENESS,
        }
    }
}

impl Class for IntelligenceLobeField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl IntelligenceLobeField {
    const CLASSNAME: &'static str = "IntelligenceLobe";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::IntelligenceLobe as ClassID, Self::CLASSNAME);
    const FIELDNAME_INTERFACE_UID: &'static str = "interface_uid";
    const FIELDNAME_ROUTINE_UID: &'static str = "routine_uid";
    const FIELDNAME_ROUTINE_AWARENESS: &'static str = "routine_awareness";

    const FIELD_INTERFACE_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INTERFACE_UID,
        FieldValueType::UID(InterfaceField::class_ident_const()));
    const FIELD_ROUTINE_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_UID,
        FieldValueType::UID(RoutineLobeField::class_ident_const()));
    const FIELD_ROUTINE_AWARENESS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_AWARENESS,
        FieldValueType::Enum(Awareness::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IntelligenceLobeBuilder {
    builder_mode: BuilderMode,
    interface_uid: Option<IdentityBuilder>,
    routine_uid: Option<IdentityBuilder>,
    routine_awareness: Option<Awareness>
}

impl Builder for IntelligenceLobeBuilder {
    type ModelType = IntelligenceLobe;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            interface_uid: None,
            routine_uid: None,
            routine_awareness: None
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

    fn create(mut self) -> Result<Creation<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        let interface_uid = Build::create_uid(&mut self.interface_uid, &mut fields_changed, IntelligenceLobeField::InterfaceUID)?;
        
        let intelligence_lobe = IntelligenceLobe {
            interface_uid
        };

        Ok(Creation::new(self, intelligence_lobe))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        Build::modify_uid(&mut self.interface_uid, &mut existing.interface_uid(), &mut fields_changed, IntelligenceLobeField::InterfaceUID)?;

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        IntelligenceLobeField::class_ident()
    }
}
