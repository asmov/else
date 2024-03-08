//! Cortex and Lobes
//! We've borrowed the terms from neuroscience to model the interface between a [Character], its environment, and its behavior.
//! In our model, a Cortex represents the core behavioral interface of a Character as a whole. It receives stimulus and
//! outputs [Action]s. It passes stimulus between its lobes (event processing layers):
//! - Routine: Typical simulated behavior, local
//! - Automated: Remote automation via API
//! - Psuedo-intelligence: Remote control by an AI service via API 
//! - Intelligence: Human control via UI
//! 
//! Lobe divisions are not so much functional, as real lobes in a cerebral cortex are, but rather they are a layers of 
//! authority and control over a Character's behavior that are reminiscent of Frued's concept of a psyche being divided
//! between base id, ego, and superego. Our Routine Lobe is the most basic and is always present. All others are available
//! only if the Character has been downlinked into. All lobes can exist at the same time and they can interact, with
//! the most intelligent lobe having final say over the Character's behavior. 
//! 
//! The omnipresent [Routine] has its [Awareness] tracked even when downlinked. If some major trauma occurs or if
//! the Characters' recent actions are too far from the norm, the Routine Lobe will go into a state of shock, removing its
//! uplinks, and resuming control of the Character.
//! 
//! Routines may establish some of the capabilities of a Character. Upper lobes can only make use of them, enhance them,
//! and improve them.

pub mod awareness;
pub mod lobe {
    pub mod routine;
    pub mod intelligence;
}

use serde;
use crate::{error::*, modeling::*, identity::*, codebase::*};

pub use awareness::*;
pub use lobe::routine::*;
pub use lobe::intelligence::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Cortex {
    routine_lobe: RoutineLobe,
    intelligence_lobe: Option<IntelligenceLobe>
}

/// Composition trait for Cortex
pub trait Sensitive {
    fn cortex(&self) -> &Cortex;
}

pub trait Productive {
    //fn productive_lobe() -> Option<&RepetitionLobe>;
    //todo
}

/// Composition trait for PsuedoIntelligenceLobe
pub trait PsuedoIntelligent {
    //fn psuedo_intelligence_lobe() -> Option<&PsuedoIntelligenceLobe>;
    //todo
}

impl Sensitive for Cortex {
    fn cortex(&self) -> &Cortex {
        self
    }
}

impl Habitual for Cortex {
    fn routine_lobe(&self) -> &RoutineLobe {
        &self.routine_lobe
    }
}

impl Intelligent for Cortex {
    fn intelligence_lobe(&self) -> Option<&IntelligenceLobe> {
        self.intelligence_lobe.as_ref()
    }
}

impl Built for Cortex {
    type BuilderType = CortexBuilder;
}

pub enum CortexField {
    RoutineLobe,
    IntelligenceLobe
}

impl CortexField {
    const CLASSNAME: &'static str = "Cortex";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Cortex as ClassID, Self::CLASSNAME);
    const FIELDNAME_ROUTINE_LOBE: &'static str = "routine_lobe";
    const FIELDNAME_INTELLIGENCE_LOBE: &'static str = "intelligence_lobe";
    const FIELD_ROUTINE_LOBE: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_ROUTINE_LOBE, FieldValueType::Model(RoutineLobeField::class_ident_const()));
    const FIELD_INTELLIGENCE_LOBE: Field = Field::new(&Self::CLASS_IDENT, Self::FIELDNAME_INTELLIGENCE_LOBE, FieldValueType::Model(IntelligenceLobeField::class_ident_const()));

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl Fields for CortexField {
    fn field(&self) -> &'static Field {
        match self {
            Self::RoutineLobe => &Self::FIELD_ROUTINE_LOBE,
            Self::IntelligenceLobe => &Self::FIELD_INTELLIGENCE_LOBE,
        }
    }
}

impl Class for CortexField {
    fn class_ident() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CortexBuilder {
    builder_mode: BuilderMode,
    routine_lobe: Option<RoutineLobeBuilder>,
    intelligence_lobe: OptionOp<IntelligenceLobeBuilder>
}

impl Builder for CortexBuilder {
    type ModelType = Cortex;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            routine_lobe: None,
            intelligence_lobe: OptionOp::None
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

        let routine_lobe = Build::create(&mut self.routine_lobe, &mut fields_changed, CortexField::RoutineLobe)?;
        let intelligence_lobe = Build::create_option(&mut self.intelligence_lobe, &mut fields_changed, CortexField::IntelligenceLobe)?;

        let cortex = Cortex {
            routine_lobe,
            intelligence_lobe
        };

        Ok(Creation::new(self, cortex))
    }

    fn modify(mut self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        let mut fields_changed = Build::prepare_modify_composite(&mut self, existing)?;

        Build::modify(&mut self.routine_lobe, &mut existing.routine_lobe, &mut fields_changed, CortexField::RoutineLobe)?;
        Build::modify_option(&mut self.intelligence_lobe, &mut existing.intelligence_lobe, &mut fields_changed, CortexField::IntelligenceLobe)?;

        Ok(Modification::new(self, fields_changed))
    }

    fn class_ident(&self) -> &'static ClassIdent {
        CortexField::class_ident_const()
    }
}

pub trait BuildableCortex: Builder {
    fn cortex(&mut self, cortex: CortexBuilder) -> Result<&mut Self>;
    fn cortex_builder(&mut self) -> &mut CortexBuilder;
    fn get_cortex_builder(&self) -> Option<&CortexBuilder>;

    fn routine_lobe(&mut self, routine_lobe: RoutineLobeBuilder) -> Result<&mut Self> {
        self.cortex_builder().routine_lobe = Some(routine_lobe);
        Ok(self)
    }

    fn set_intelligence_lobe(&mut self, intelligence_lobe: IntelligenceLobeBuilder) -> Result<&mut Self> {
        self.cortex_builder().intelligence_lobe = OptionOp::Set(intelligence_lobe);
        Ok(self)
    }

    fn edit_intelligence_lob(&mut self, intelligence_lobe: IntelligenceLobeBuilder) -> Result<&mut Self> {
        self.cortex_builder().intelligence_lobe = OptionOp::Edit(intelligence_lobe);
        Ok(self)
    }

    fn unset_intelligence_lobe(&mut self) -> Result<&mut Self> {
        self.cortex_builder().intelligence_lobe = OptionOp::Unset;
        Ok(self)
    }
}

impl BuildableCortex for CortexBuilder {
    fn cortex(&mut self, cortex: CortexBuilder) -> Result<&mut Self> {
        *self = cortex;
        Ok(self)
    }

    fn cortex_builder(&mut self) -> &mut CortexBuilder {
        self
    }

    fn get_cortex_builder(&self) -> Option<&CortexBuilder> {
        Some(self)
    }
}

