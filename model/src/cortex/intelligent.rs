use crate::{codebase::*, error::*, identity::*, modeling::*, cortex::routine::*, interface::*};
use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IntelligentCortex {
    interface_uid: UID,
    routine_uid: UID,
    routine_awareness: Awareness,
}

impl Sensory for IntelligentCortex {
    fn routine_uid(&self) -> UID {
        self.routine_uid
    }

    fn routine_awareness(&self) -> Awareness {
        self.routine_awareness
    }
}

impl IntelligentCortex {
    pub fn interface_id(&self) -> UID {
        self.interface_uid
    }
}

#[derive(Debug)]
pub enum IntelligentCortexField {
    InterfaceID,
    RoutineID,
    RoutineAwareness
}

impl Fields for IntelligentCortexField {
    fn field(&self) -> &'static Field {
        match self {
            Self::InterfaceID => Self::InterfaceID.field(),
            Self::RoutineID => Self::RoutineID.field(),
            Self::RoutineAwareness => Self::RoutineAwareness.field(),
        }
    }
}

impl Class for IntelligentCortexField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl IntelligentCortexField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::IntelligentCortex as ClassID, Self::CLASSNAME);
    pub const CLASSNAME: &'static str = "IntelligentCortex";
    pub const FIELDNAME_INTERFACE_UID: &'static str = "interface_uid";
    pub const FIELDNAME_ROUTINE_UID: &'static str = "routine_uid";
    pub const FIELDNAME_ROUTINE_AWARENESS: &'static str = "routine_awareness";

    pub const FIELD_INTERFACE_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INTERFACE_UID,
        FieldValueType::UID(InterfaceField::class_ident_const()));
    pub const FIELD_ROUTINE_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_UID,
        FieldValueType::UID(RoutineCortexField::class_ident_const()));
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
pub struct IntelligentCortexBuilder {
    builder_mode: BuilderMode,
    interface_uid: Option<UID>,
    routine_uid: Option<UID>,
    routine_awareness: Option<Awareness>
}

impl Builder for IntelligentCortexBuilder {
    type ModelType = IntelligentCortex;
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
        Ok(IntelligentCortex {
            interface_id: self.interface_id
                .ok_or_else(|| Error::FieldNotSet {
                    class: IntelligentCortexField::CLASSNAME,
                    field: IntelligentCortexField::FIELDNAME_INTERFACE_ID})?,
            routine_id: self.routine_id
                .ok_or_else(|| Error::FieldNotSet {
                    class: IntelligentCortexField::CLASSNAME,
                    field: IntelligentCortexField::FIELDNAME_ROUTINE_ID})?,
            routine_awareness: self.routine_awareness
                .ok_or_else(|| Error::FieldNotSet {
                    class: IntelligentCortexField::CLASSNAME,
                    field: IntelligentCortexField::FIELDNAME_ROUTINE_AWARENESS})?,
        })*/
    }

    fn modify(self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        todo!()
    }

    fn class_ident(&self) -> &'static ClassIdent {
        IntelligentCortexField::class_ident()
    }
}

impl IntelligentCortex {
    pub fn routine_id(&mut self, routine_id: UID) -> Result<()> {
        self.routine_uid = routine_id;
        Ok(())
    }

    pub fn routine_awareness(&mut self, routine_awarness: Awareness) -> Result<()> {
        self.routine_awareness = routine_awarness;
        Ok(())
    } 
}

impl CortexBuilderVariant for IntelligentCortexBuilder {
    fn cortex_builder(self) -> CortexBuilder {
        CortexBuilder::Intelligent(self)
    }
}