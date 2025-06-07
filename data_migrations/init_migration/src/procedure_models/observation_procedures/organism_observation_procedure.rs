use core_structures::{ProcedureModel, User};
use web_common_traits::database::{Insertable, InsertableVariant};

/// The name of the Organism observation procedure model.
pub const ORGANISM_OBSERVATION: &str = "Organism observation procedure";

/// Initializes the Organism observation procedure model in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure model.
/// * If the procedure model building fails.
pub(super) fn init_organism_observation_procedure(user: &User, conn: &mut diesel::PgConnection) {
    ProcedureModel::new()
        .name(ORGANISM_OBSERVATION)
        .unwrap()
        .description(
			"Procedure for observing an organism, and relevant details for identification and study.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
		.unwrap();
}
