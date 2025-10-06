//! Submodule initializing trackable categories.

pub mod bead;
pub mod compatibility_rules;
pub mod containers;
pub mod instruments;
pub mod markers;
pub mod organisms;
pub mod photographs;
pub mod ppe;
pub mod reagent_models;
pub mod soils;
pub mod tools;

pub(crate) use compatibility_rules::init_compatibility_rules;
