//! Submodule defining the init migrations for the procedure templates.

mod dbgi_plan;
mod vinesh_plan;
pub use dbgi_plan::init_dbgi_plan;
mod analysis_procedures;
mod collection_preparation_procedures;
mod data_enrichment_procedures;
pub(crate) use analysis_procedures::{
    init_negative_ionization_lcms_procedure, init_positive_ionization_lcms_procedure,
};
pub(crate) use collection_preparation_procedures::sample_extraction_solvent_procedure;
pub(crate) use data_enrichment_procedures::init_data_enrichment_procedure;
