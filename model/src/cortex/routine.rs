use crate::{s, error::*, identity::*, builder::*, descriptor::*, entity::*, something::*, thing::*, interface::*,
    timeframe::*, stimulus::*};
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

#[derive(Debug)]
pub enum RoutineCortexField {
    RoutineID,
    RoutineAwareness
}

impl RoutineCortexField {
    pub const CLASSNAME: &'static str = "RoutineCortex";
    pub const FIELDNAME_ROUTINE_ID: &'static str = "routine_id";
    pub const FIELDNAME_ROUTINE_AWARENESS: &'static str = "routine_awareness";

    pub const FIELD_ROUTINE_ID: Field = Field::new(Self::FIELDNAME_ROUTINE_ID, FieldValueType::UnsignedInteger);
    pub const FIELD_ROUTINE_AWARENESS: Field = Field::new(Self::FIELDNAME_ROUTINE_AWARENESS, FieldValueType::Enum);

    pub const fn field(&self) -> &'static Field {
        match self {
            Self::RoutineID => &Self::FIELD_ROUTINE_ID,
            Self::RoutineAwareness => &Self::FIELD_ROUTINE_AWARENESS,
        }
    }
}

#[derive(Debug)]
pub struct RoutineCortexBuilder {
    builder_mode: BuilderMode,
    routine_id: Option<RoutineID>,
    routine_awareness: Option<Awareness>
}

impl Builder for RoutineCortexBuilder {
    type Type = RoutineCortex;

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

    fn create(self) -> Result<Self::Type> {
        Ok(RoutineCortex{
            routine_id: self.routine_id
                .ok_or_else(|| Error::FieldNotSet {
                    class: RoutineCortexField::CLASSNAME,
                    field: RoutineCortexField::FIELDNAME_ROUTINE_ID})?,
            routine_awareness: self.routine_awareness
                .ok_or_else(|| Error::FieldNotSet {
                    class: RoutineCortexField::CLASSNAME,
                    field: RoutineCortexField::FIELDNAME_ROUTINE_AWARENESS})?,
        })
    }

    fn modify(self, original: &mut Self::Type) -> Result<ModifyResult> {
        todo!()
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

impl CortexBuilderTrait for RoutineCortexBuilder {
    fn create_cortex(self) -> Result<Cortex> {
        Ok(Cortex::Routine(self.create()?))
    }

    fn modify_cortex(self, original: &mut Self::Type) -> Result<ModifyResult> {
        self.modify(original)
    }

    fn cortex_builder(self) -> CortexBuilder {
        CortexBuilder::Routine(self)
    }
}