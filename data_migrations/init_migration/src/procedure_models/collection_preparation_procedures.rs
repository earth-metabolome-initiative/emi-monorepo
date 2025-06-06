//! Submodule initializing the collection preparation procedures in the
//! database.

mod diluted_ethanol_procedure;
mod sample_extraction_solvent_procedures;
pub(crate) use diluted_ethanol_procedure::E70_ETHANOL;

pub(super) fn init_collection_preparation_procedures(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) {
    diluted_ethanol_procedure::init_ethanol_70_percent(user, conn);
    sample_extraction_solvent_procedures::init_sample_extraction_solvent_procedures(user, conn);
}
