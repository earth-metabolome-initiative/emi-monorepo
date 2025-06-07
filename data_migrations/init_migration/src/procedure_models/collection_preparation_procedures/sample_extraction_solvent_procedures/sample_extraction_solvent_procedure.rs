//! Submodule defining the procedure model to create a standard sample
//! extraction solvent.

use web_common_traits::database::{Insertable, InsertableVariant};

/// Creates a procedure model for sample extraction solvent preparation.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure model.
/// * `conn` - The database connection to use for the insertion.
pub(super) fn init_sample_extraction_solvent_procedure(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) {
    core_structures::ProcedureModel::new()
        .name("Sample Extraction Solvent")
        .unwrap()
        .description(
			"Procedure model for sample extraction solvent preparation, used in various laboratory procedures.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
		.unwrap();
}
