use serde;
use crate::{modeling::*, identity::*, codebase::*};

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

