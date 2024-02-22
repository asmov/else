use crate::{codebase::*, error::*, identity::*, builder::*};
use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RoutineCortex {
    routine_id: RoutineID,
    routine_awareness: Awareness
}

impl Sensory for RoutineCortex {
    fn routine_id(&self) -> RoutineID {
        self.routine_id
    }

    fn routine_awareness(&self) -> Awareness {
        self.routine_awareness
    }
}

impl RoutineCortex {
    pub fn routine_id(&mut self, routine_id: RoutineID) -> Result<()> {
        self.routine_id = routine_id;
        Ok(())
    }

    pub fn routine_awareness(&mut self, routine_awarness: Awareness) -> Result<()> {
        self.routine_awareness = routine_awarness;
        Ok(())
    } 
}

#[derive(Debug)]
pub enum RoutineCortexField {
    RoutineID,
    RoutineAwareness
}

impl Fields for RoutineCortexField {
    fn field(&self) -> &'static Field {
        match self {
            Self::RoutineID => &Self::FIELD_ROUTINE_ID,
            Self::RoutineAwareness => &Self::FIELD_ROUTINE_AWARENESS,
        }
    }
}

impl Class for RoutineCortexField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl RoutineCortexField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::RoutineCortex as ClassID, Self::CLASSNAME);
    const CLASSNAME: &'static str = "RoutineCortex";
    const FIELDNAME_ROUTINE_ID: &'static str = "routine_id";
    const FIELDNAME_ROUTINE_AWARENESS: &'static str = "routine_awareness";

    const FIELD_ROUTINE_ID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_ID, FieldValueType::UID);
    const FIELD_ROUTINE_AWARENESS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_AWARENESS, FieldValueType::Enum);

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RoutineCortexBuilder {
    builder_mode: BuilderMode,
    routine_id: Option<RoutineID>,
    routine_awareness: Option<Awareness>
}

impl Builder for RoutineCortexBuilder {
    type ModelType = RoutineCortex;
    type BuilderType = CortexBuilder;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
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

    fn create(mut self) -> Result<Creation<Self::BuilderType>> {
        let routine_id = Self::try_assign_value(&mut self.routine_id, RoutineCortexField::RoutineID)?;
        let routine_awareness = Self::try_assign_value(&mut self.routine_awareness, RoutineCortexField::RoutineAwareness)?;

        let routine_cortex = RoutineCortex{
            routine_id: routine_id,
            routine_awareness: routine_awareness,
        };

        Ok(Creation::new(CortexBuilder::Routine(self), Cortex::Routine(routine_cortex)))
    }

    fn modify(self, original: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        todo!()
    }

    fn class_ident(&self) -> &'static ClassIdent {
        RoutineCortexField::class_ident()
    }
}

impl CortexBuilderVariant for RoutineCortexBuilder {
    fn cortex_builder(self) -> CortexBuilder {
        CortexBuilder::Routine(self)
    }
}

impl RoutineCortexBuilder {
    pub fn routine_id(&mut self, routine_id: RoutineID) -> Result<()> {
        self.routine_id = Some(routine_id);
        Ok(())
    }

    pub fn routine_awareness(&mut self, awareness: Awareness) -> Result<()> {
        self.routine_awareness = Some(awareness);
        Ok(())
    }
}