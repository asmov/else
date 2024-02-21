use crate::{codebase::*, error::*, identity::*, builder::*, interface::*};
use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IntelligentCortex {
    interface_id: InterfaceID,
    routine_id: RoutineID,
    routine_awareness: Awareness,
}

impl Sensory for IntelligentCortex {
    fn routine_id(&self) -> RoutineID {
        self.routine_id
    }

    fn routine_awareness(&self) -> Awareness {
        self.routine_awareness
    }
}

impl IntelligentCortex {
    pub fn interface_id(&self) -> InterfaceID {
        self.interface_id
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
    pub const FIELDNAME_INTERFACE_ID: &'static str = "interface_id";
    pub const FIELDNAME_ROUTINE_ID: &'static str = "routine_id";
    pub const FIELDNAME_ROUTINE_AWARENESS: &'static str = "routine_awareness";

    pub const FIELD_INTERFACE_ID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INTERFACE_ID, FieldValueType::U64);
    pub const FIELD_ROUTINE_ID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_ID, FieldValueType::U64);
    pub const FIELD_ROUTINE_AWARENESS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_AWARENESS, FieldValueType::Enum);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::InterfaceID => &Self::FIELD_ROUTINE_ID,
            Self::RoutineID => &Self::FIELD_ROUTINE_ID,
            Self::RoutineAwareness => &Self::FIELD_ROUTINE_AWARENESS,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct IntelligentCortexBuilder {
    builder_mode: BuilderMode,
    interface_id: Option<InterfaceID>,
    routine_id: Option<RoutineID>,
    routine_awareness: Option<Awareness>
}

impl Builder for IntelligentCortexBuilder {
    type ModelType = IntelligentCortex;
    type BuilderType = CortexBuilder;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            interface_id: None,
            routine_id: None,
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

    fn modify(self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        todo!()
    }

    fn class_id(&self) -> ClassID {
        IntelligentCortexField::class_id()
    }
}

impl IntelligentCortex {
    pub fn routine_id(&mut self, routine_id: RoutineID) -> Result<()> {
        self.routine_id = routine_id;
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