pub mod routine;
pub mod intelligent;

use crate::{s, error::*, identity::*, builder::*, descriptor::*, entity::*, something::*, thing::*, interface::*,
    timeframe::*};
use routine::*;
use intelligent::*;
use serde;

pub use routine::*;


pub type RoutineID = u8;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Cortex {
    /// Local software automation (Behavior)
    /// Acts as a submliminal information provider when an Interface is in control.
    Routine(RoutineCortex),
    // Remote software automation via Interface (API)
    //Machine(MachineCortex),
    // AI via Interface (API)
    //PsuedoIntelligent(PsuedoIntelligentCortex),
    /// Human via Interface (UI)
    Intelligent(IntelligentCortex)
}

/// Data model trait for a Cortex
pub trait Sensory {
    fn routine_id(&self) -> RoutineID;
    fn routine_awareness(&self) -> Awareness;
}

/// Composition model trait for a Cortex
pub trait Sensitive {
    fn cortex(&self) -> &Cortex;

    fn routine_id(&self) -> RoutineID {
        self.cortex().routine_id()
    }

    fn routine_awareness(&self) -> Awareness {
        self.cortex().routine_awareness()
    }
}

pub trait SensitiveMut: Sensitive {
    fn cortext_mut(&mut self) -> &mut Cortex;
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum Awareness {
    Shock,
    Conscious,
    Subconscious,
}

impl Sensitive for Cortex {
    fn cortex(&self) -> &Cortex {
        self
    }

    fn routine_id(&self) -> RoutineID {
        match self {
            Cortex::Routine(cortex) => cortex.routine_id(),
            //Cortex::Machine(_) => todo!(),
            //Cortex::PsuedoIntelligent(_) => todo!(),
            Cortex::Intelligent(cortex) => cortex.routine_id(),
        }
    }

    fn routine_awareness(&self) -> Awareness {
        match self {
            Cortex::Routine(cortex) => cortex.routine_awareness(),
            //Cortex::Machine(_) => todo!(),
            //Cortex::PsuedoIntelligent(_) => todo!(),
            Cortex::Intelligent(cortex) => cortex.routine_awareness(),
        }        
    }
}

impl SensitiveMut for Cortex {
    fn cortext_mut(&mut self) -> &mut Cortex {
        self
    }
}

pub trait BuildableCortex: Builder {
    fn cortex(&mut self, cortex: CortexBuilder) -> Result<()>;
    fn get_cortex_builder_mut(&mut self) -> &mut Option<CortexBuilder>; 

    fn routine_cortex_builder(&mut self) -> &mut RoutineCortexBuilder {
        let builder_mode = self.builder_mode();
        let cortex_builder = self.get_cortex_builder_mut();
        if let Some(CortexBuilder::Routine(routine_builder)) = cortex_builder {
            return routine_builder
        }
        
        let routine_builder = RoutineCortexBuilder::builder(builder_mode);
        *cortex_builder = Some(routine_builder.cortex_builder());
        if let Some(CortexBuilder::Routine(routine_builder)) = cortex_builder {
            return routine_builder
        } else {
            unreachable!("Dispatch mismatch");
        }
    }

    fn intelligent_cortex_builder(&mut self) -> &mut IntelligentCortexBuilder {
        let builder_mode = self.builder_mode();
        let cortex_builder = self.get_cortex_builder_mut();
        if let Some(CortexBuilder::Intelligent(intelligent_builder)) = cortex_builder {
            return intelligent_builder
        }
        
        let intelligent_builder = IntelligentCortexBuilder::builder(builder_mode);
        *cortex_builder = Some(intelligent_builder.cortex_builder());
        if let Some(CortexBuilder::Intelligent(intelligent_builder)) = cortex_builder {
            return intelligent_builder
        } else {
            unreachable!("Dispatch mismatch");
        }
    }
}


pub trait CortexBuilderTrait: Builder {
    fn create_cortex(self) -> Result<Cortex>;
    fn modify_cortex(self, original: &mut Self::Type) -> Result<Modification<Self>>; 
    fn cortex_builder(self) -> CortexBuilder;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CortexBuilder {
    Routine(RoutineCortexBuilder),
    Intelligent(IntelligentCortexBuilder)
}

impl Builder for CortexBuilder {
    type Type = Cortex;
    type BuilderType = Self;

    fn creator() -> Self {
        panic!("Cannot call CortexBuilder::creator() directly")
    }

    fn editor() -> Self {
        panic!("Cannot call CortexBuilder::editor() directly")
    }

    fn builder_mode(&self) -> BuilderMode {
        match self {
            CortexBuilder::Routine(builder) => builder.builder_mode(),
            CortexBuilder::Intelligent(builder) => builder.builder_mode(),
        }
    }

    fn create(self) -> Result<Self::Type> {
        match self {
            CortexBuilder::Routine(builder) => builder.create_cortex(),
            CortexBuilder::Intelligent(builder) => builder.create_cortex(),
        }
    }

    fn modify(self, original: &mut Self::Type) -> Result<Modification<Self::BuilderType>> {
        panic!("Cannot call CortexBuilder::modify() directly")
        /*match self {
            CortexBuilder::Routine(builder) => {
                if let Cortex::Routine(original_routine_cortex) = original {
                    builder.modify_cortex(original_routine_cortex)
                } else {
                    unreachable!("Dispatch mismatch in CortexBuilder::modify() for RoutineCortex")
                }
            },
            CortexBuilder::Intelligent(builder) => {
                if let Cortex::Intelligent(original_intelligent_cortex) = original {
                    builder.modify_cortex(original_intelligent_cortex)
                } else {
                    unreachable!("Dispatch mismatch in CortexBuilder::modify() for IntelligentCortex")
                }
            }
        }*/
    }
}


