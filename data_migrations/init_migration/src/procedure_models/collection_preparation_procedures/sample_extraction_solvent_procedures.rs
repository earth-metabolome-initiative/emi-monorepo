//! Submodule creating the procedures describing the creation of solvents to be
//! used to extract samples.

mod sample_extraction_solvent_procedures;

/// Method initializing the procedure models for creating solvents used in
/// sample extraction.
pub(super) fn init_sample_extraction_solvent_procedures(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) {
    sample_extraction_solvent_procedures::init_sample_extraction_solvent_procedure(user, conn);
}
