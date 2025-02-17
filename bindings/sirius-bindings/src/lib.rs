//! This crate provides bindings for the Sirius executable.
//! All the parameters of sirius should be accessible from this crate.
#![doc = include_str!("../README.md")]
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
    pub use crate::{
        builder::SiriusBuilder, parameters::*, sirius::Sirius, sirius_types::*, versions::*,
    };
}
