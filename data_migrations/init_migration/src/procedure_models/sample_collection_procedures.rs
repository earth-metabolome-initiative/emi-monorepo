//! Submodule defining the sample collection procedures in the
//! database.

mod full_organism_collection_procedure;
mod part_of_organism_collection_procedure;

/// Initializes the sample collection procedures in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure models.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure models.
/// * If the procedure model building fails.
pub(super) fn init_sample_collection_procedures(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) {
	full_organism_collection_procedure::init_full_organism_collection(user, conn);
	part_of_organism_collection_procedure::init_part_of_organism_collection(user, conn);
}
