#![doc = include_str!("../README.md")]

mod asset_models;
mod errors;
mod procedure_templates_vis;
pub use asset_models::asset_model_hierarchy;
pub use errors::Error;
mod traits;
pub use traits::MermaidDB;
mod table_icons;
pub(crate) use table_icons::asset_model_icon;
