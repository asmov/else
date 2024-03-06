pub mod routine;
pub mod intelligence;

use { serde, strum };
use crate::{error::*, modeling::*, world::*, identity::*, codebase::*};

pub use intelligence::*;
pub use routine::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Cortex {
    /// Local behavior
    routine_lobe: RoutineLobe,

    // Remote software automation via Interface (API)
    //Machine(MachineCortex),
    // AI via Interface (API)
    //PsuedoIntelligent(PsuedoIntelligentCortex),

    /// Human via Interface (UI)
    intelligence_lobe: Option<IntelligenceLobe>
}

/// Data model trait for a Cortex
pub trait Sensory {
    fn routine_uid(&self) -> UID;
    fn routine_awareness(&self) -> Awareness;
}

/// Composition model trait for a Cortex
pub trait Sensitive {
    fn cortex(&self) -> &Cortex;
}

pub trait Habitual {
    fn routine_lobe(&self) -> &RoutineLobe;

    fn routine_id(&self) -> UID {
        self.routine_lobe().routine_uid()
    }

    fn routine_awareness(&self) -> Awareness {
        self.routine_lobe().routine_awareness()
    }
}

pub trait Productive {
    //fn productive_lobe() -> Option<&RepetitionLobe>;
    //todo
}

pub trait PsuedoIntelligent {
    //fn psuedo_intelligence_lobe() -> Option<&PsuedoIntelligenceLobe>;
    //todo
}

pub trait Intelligent {
    fn intelligence_lobe(&self) -> Option<&IntelligenceLobe>;

    fn interface_id(&self) -> Option<UID> {
        self.intelligence_lobe()
            .map(|lobe| lobe.interface_id())
    }
}

/// Routines are either:
/// - Fully in control of a Character's behavior: Conscious
/// - Not in control, but provides prompts to a Character's behavior on how it would act: Subconscious
/// - Temporarily suspending a Character's behavior due to trauma: Shock
/// - Character behavior is completely suspended and all lobes non-responsive: Unconscious
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, strum::Display, strum::EnumString)]
pub enum Awareness {
    Unconscious,
    Conscious,
    Subconscious,
    Shock,
}

impl Awareness {
    const CLASSNAME: &'static str = "Awareness";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Awareness as ClassID, Self::CLASSNAME);

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
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

pub trait BuildableCortex: Builder {
    fn cortex(&mut self, cortex: CortexBuilder) -> Result<&mut Self>;
    fn cortex_builder(&mut self) -> &mut CortexBuilder;
    fn get_cortex_builder(&self) -> Option<&CortexBuilder>;

    fn routine_lobe(&mut self, routine_lobe: RoutineLobeBuilder) -> Result<&mut Self> {
        self.cortex_builder().routine_lobe = Some(routine_lobe);
        Ok(self)
    }

    fn intelligence_lobe(&mut self, intelligence_lobe: IntelligenceLobeBuilder) -> Result<&mut Self> {
        self.cortex_builder().intelligence_lobe = Some(intelligence_lobe);
        Ok(self)
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CortexBuilder {
    builder_mode: BuilderMode,
    routine_lobe: Option<RoutineLobeBuilder>,
    intelligence_lobe: Option<IntelligenceLobeBuilder>
}

impl Builder for CortexBuilder {
    type ModelType = Cortex;
    type BuilderType = Self;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            routine_lobe: None,
            intelligence_lobe: None
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
        todo!()
    }

    fn modify(self, existing: &mut Self::ModelType) -> Result<Modification<Self::BuilderType>> {
        todo!()
    }

    fn class_ident(&self) -> &'static ClassIdent {
        CortexField::class_ident_const()
    }
}


