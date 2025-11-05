//! Submodule defining the init migrations for the procedure templates.

mod dbgi_procedure_template;
mod emi_procedure_template;
pub use dbgi_procedure_template::dbgi_procedure_template;
pub use emi_procedure_template::{emi_insect_procedure_template, emi_procedure_template};
mod analysis_procedures;
mod collection_preparation_procedures;
mod data_enrichment_procedures;
mod shared_sub_procedure_templates;
pub(crate) use analysis_procedures::{
    init_negative_ionization_lcms_procedure, init_positive_ionization_lcms_procedure,
};
pub(crate) use collection_preparation_procedures::sample_extraction_solvent_procedure;
pub(crate) use data_enrichment_procedures::init_data_enrichment_procedure;
pub use shared_sub_procedure_templates::organism_observation_procedure;
