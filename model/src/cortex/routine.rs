use crate::{codebase::*, error::*, identity::*, modeling::*};
use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RoutineCortex {
    routine_uid: UID,
    routine_awareness: Awareness
}

impl Sensory for RoutineCortex {
    fn routine_uid(&self) -> UID {
        self.routine_uid
    }

    fn routine_awareness(&self) -> Awareness {
        self.routine_awareness
    }
}

impl RoutineCortex {
    pub fn routine_uid(&mut self, routine_id: UID) -> Result<()> {
        self.routine_uid = routine_id;
        Ok(())
    }

    pub fn routine_awareness(&mut self, routine_awarness: Awareness) -> Result<()> {
        self.routine_awareness = routine_awarness;
        Ok(())
    } 
}

#[derive(Debug)]
pub enum RoutineCortexField {
    RoutineUID,
    RoutineAwareness
}

impl Fields for RoutineCortexField {
    fn field(&self) -> &'static Field {
        match self {
            Self::RoutineUID => &Self::FIELD_ROUTINE_UID,
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

    const FIELD_ROUTINE_UID: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_ID, FieldValueType::UID(&Self::CLASS_IDENT));
    const FIELD_ROUTINE_AWARENESS: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_AWARENESS, FieldValueType::Enum(Awareness::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RoutineCortexBuilder {
    builder_mode: BuilderMode,
    routine_uid: Option<UID>,
    routine_awareness: Option<Awareness>
}

impl Builder for RoutineCortexBuilder {
    type DomainType = World;
    type ModelType = RoutineCortex;
    type BuilderType = CortexBuilder;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
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

        let routine_uid = Build::create_value(&mut self.routine_uid, &mut fields_changed, RoutineCortexField::RoutineUID)?;
        let routine_awareness = Build::create_value(&mut self.routine_awareness, &mut fields_changed, RoutineCortexField::RoutineAwareness)?;

        let routine_cortex = RoutineCortex {
            routine_uid,
            routine_awareness,
        };

        Ok(Creation::new(CortexBuilder::Routine(self), Cortex::Routine(routine_cortex)))
    }

    fn modify(self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        if self.routine_uid.is_some() {
            existing.routine_uid = Build::modify_value(&self.routine_uid, &mut fields_changed, RoutineCortexField::RoutineUID)?;
        }
        if self.routine_awareness.is_some() {
            existing.routine_awareness = Build::modify_value(&self.routine_awareness, &mut fields_changed, RoutineCortexField::RoutineAwareness)?;
        }

        Ok(Modification::new(CortexBuilder::Routine(self), fields_changed))
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
    pub fn routine_uid(&mut self, routine_id: UID) -> Result<()> {
        self.routine_uid = Some(routine_id);
        Ok(())
    }

    pub fn routine_awareness(&mut self, awareness: Awareness) -> Result<()> {
        self.routine_awareness = Some(awareness);
        Ok(())
    }
}