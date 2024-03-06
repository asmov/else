use crate::{codebase::*, error::*, identity::*, modeling::*};
use super::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RoutineLobe {
    routine_uid: UID,
    routine_awareness: Awareness
}

impl Sensory for RoutineLobe {
    fn routine_uid(&self) -> UID {
        self.routine_uid
    }

    fn routine_awareness(&self) -> Awareness {
        self.routine_awareness
    }
}

impl RoutineLobe {
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
pub enum RoutineLobeField {
    RoutineUID,
    RoutineAwareness
}

impl Fields for RoutineLobeField {
    fn field(&self) -> &'static Field {
        match self {
            Self::RoutineUID => &Self::FIELD_ROUTINE_UID,
            Self::RoutineAwareness => &Self::FIELD_ROUTINE_AWARENESS,
        }
    }
}

impl Class for RoutineLobeField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl RoutineLobeField {
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::RoutineLobe as ClassID, Self::CLASSNAME);
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
pub struct RoutineLobeBuilder {
    builder_mode: BuilderMode,
    routine_uid: Option<UID>,
    routine_awareness: Option<Awareness>
}

impl Builder for RoutineLobeBuilder {
    type ModelType = RoutineLobe;
    type BuilderType = RoutineLobeBuilder;

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

        let routine_uid = Build::create_value(&mut self.routine_uid, &mut fields_changed, RoutineLobeField::RoutineUID)?;
        let routine_awareness = Build::create_value(&mut self.routine_awareness, &mut fields_changed, RoutineLobeField::RoutineAwareness)?;

        let routine_lobe = RoutineLobe {
            routine_uid,
            routine_awareness,
        };

        Ok(Creation::new(self, routine_lobe))
    }

    fn modify(self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = FieldsChanged::from_builder(&self);

        if self.routine_uid.is_some() {
            existing.routine_uid = Build::modify_value(&self.routine_uid, &mut fields_changed, RoutineLobeField::RoutineUID)?;
        }
        if self.routine_awareness.is_some() {
            existing.routine_awareness = Build::modify_value(&self.routine_awareness, &mut fields_changed, RoutineLobeField::RoutineAwareness)?;
        }

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        RoutineLobeField::class_ident()
    }
}

impl RoutineLobeBuilder {
    pub fn routine_uid(&mut self, routine_id: UID) -> Result<()> {
        self.routine_uid = Some(routine_id);
        Ok(())
    }

    pub fn routine_awareness(&mut self, awareness: Awareness) -> Result<()> {
        self.routine_awareness = Some(awareness);
        Ok(())
    }
}