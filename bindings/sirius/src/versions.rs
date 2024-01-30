use std::fmt::Debug;

use crate::sirius_types;
use crate::traits::Enablable;
use crate::traits::IntoDefault;
use crate::traits::NamedParametersSet;

pub trait Version: Default {
    const VERSION: usize;
    type Core: ToString + Debug + IntoDefault;
    type Config: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;
    type Canopus: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;
    type Formula: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;
    type Zodiac: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;
    type Fingerprint: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;
    type Structure: ToString + Debug + IntoDefault + NamedParametersSet + Enablable;
}
#[derive(Default)]
pub struct Version5;

impl Version for Version5 {
    const VERSION: usize = 5;
    type Core = crate::parameters::core::CoreV5;
    type Config = crate::parameters::config::ConfigV5;
    type Canopus = crate::parameters::canopus::CanopusV5;
    type Formula = crate::parameters::formula::FormulaV5;
    type Zodiac = crate::parameters::zodiac::ZodiacV5;
    type Fingerprint = crate::parameters::fingerprint::FingerprintV5;
    type Structure = crate::parameters::structure::StructureV5;
}
