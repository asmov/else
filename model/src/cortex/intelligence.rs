use crate::{codebase::*, error::*, identity::*, modeling::*, cortex::routine::*, interface::*};
use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IntelligenceLobe {
    interface_uid: UID,
}

impl IntelligenceLobe {
    pub fn interface_id(&self) -> UID {
        self.interface_uid
    }
}

#[derive(Debug)]
pub enum IntelligenceLobeField {
    InterfaceID,
    RoutineID,
    RoutineAwareness
}

impl Fields for IntelligenceLobeField {
    fn field(&self) -> &'static Field {
        match self {
            Self::InterfaceID => Self::InterfaceID.field(),
            Self::RoutineID => Self::RoutineID.field(),
            Self::RoutineAwareness => Self::RoutineAwareness.field(),
        }
    }
}

impl Class for IntelligenceLobeField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl IntelligenceLobeField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::IntelligenceLobe as ClassID, Self::CLASSNAME);
    pub const CLASSNAME: &'static str = "IntelligenceCortex";
    pub const FIELDNAME_INTERFACE_UID: &'static str = "interface_uid";
    pub const FIELDNAME_ROUTINE_UID: &'static str = "routine_uid";
    pub const FIELDNAME_ROUTINE_AWARENESS: &'static str = "routine_awareness";

    pub const FIELD_INTERFACE_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INTERFACE_UID,
        FieldValueType::UID(InterfaceField::class_ident_const()));
    pub const FIELD_ROUTINE_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_UID,
        FieldValueType::UID(RoutineLobeField::class_ident_const()));
    pub const FIELD_ROUTINE_AWARENESS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_AWARENESS,
        FieldValueType::Enum(Awareness::class_ident_const()));

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::InterfaceID => &Self::FIELD_ROUTINE_UID,
            Self::RoutineID => &Self::FIELD_ROUTINE_UID,
            Self::RoutineAwareness => &Self::FIELD_ROUTINE_AWARENESS,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IntelligenceLobeBuilder {
    builder_mode: BuilderMode,
    interface_uid: Option<UID>,
    routine_uid: Option<UID>,
    routine_awareness: Option<Awareness>
}

impl Builder for IntelligenceLobeBuilder {
    type ModelType = IntelligenceLobe;
    type BuilderType = CortexBuilder;

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

    fn create(self) -> Result<Creation<Self::BuilderType>> {
        todo!();/*
        Ok(IntelligenceCortex {
            interface_id: self.interface_id
                .ok_or_else(|| Error::FieldNotSet {
                    class: IntelligenceCortexField::CLASSNAME,
                    field: IntelligenceCortexField::FIELDNAME_INTERFACE_ID})?,
            routine_id: self.routine_id
                .ok_or_else(|| Error::FieldNotSet {
                    class: IntelligenceCortexField::CLASSNAME,
                    field: IntelligenceCortexField::FIELDNAME_ROUTINE_ID})?,
            routine_awareness: self.routine_awareness
                .ok_or_else(|| Error::FieldNotSet {
                    class: IntelligenceCortexField::CLASSNAME,
                    field: IntelligenceCortexField::FIELDNAME_ROUTINE_AWARENESS})?,
        })*/
    }

    fn modify(self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        todo!()
    }

    fn class_ident(&self) -> &'static ClassIdent {
        IntelligenceLobeField::class_ident()
    }
}
