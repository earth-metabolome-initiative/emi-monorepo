//! Submodule initializing the collection preparation procedures in the
//! database.

mod diluted_ethanol_procedure;
pub(crate) use diluted_ethanol_procedure::ethanol_70_percent;
mod sample_extraction_solvent_procedures;
pub(crate) use sample_extraction_solvent_procedures::sample_extraction_solvent_procedure;
