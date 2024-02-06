//! Bidingins for the Sirius library.
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod builder;
mod parameters;
mod sirius;
mod sirius_config;
mod sirius_types;
mod traits;
mod versions;

pub mod prelude {
    pub use crate::builder::SiriusBuilder;
    pub use crate::parameters::*;
    pub use crate::sirius::Sirius;
    pub use crate::sirius_types::*;
    pub use crate::versions::*;
}
