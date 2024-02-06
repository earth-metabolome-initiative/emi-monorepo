//! This crate provides bindings for the Sirius executable.
//! All the parameters of sirius should be accessible from this crate.
#![deny(missing_docs)]

/// import the different modules
mod builder;
mod parameters;
mod sirius;
mod sirius_config;
mod sirius_types;
mod traits;
mod versions;

/// Prelude module
pub mod prelude {
    pub use crate::builder::SiriusBuilder;
    pub use crate::parameters::*;
    pub use crate::sirius::Sirius;
    pub use crate::sirius_types::*;
    pub use crate::versions::*;
}
