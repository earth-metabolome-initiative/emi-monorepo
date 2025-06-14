//! Submodule defining the procedure model to create a standard sample
//! extraction solvent.
use core_structures::{ProcedureModel, User};
use web_common_traits::database::{Insertable, InsertableVariant};

/// The name of the sample extraction solvent procedure model.
pub const SAMPLE_EXTRACTION_SOLVENT: &str = "Sample extraction solvent";

/// Creates a procedure model for sample extraction solvent preparation.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure model.
/// * `conn` - The database connection to use for the insertion.
pub(crate) fn init_sample_extraction_solvent_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    if let Some(procedure) = ProcedureModel::from_name(SAMPLE_EXTRACTION_SOLVENT, conn).unwrap() {
        return procedure;
    }

    ProcedureModel::new()
        .name(SAMPLE_EXTRACTION_SOLVENT)
        .unwrap()
        .description(
			"Procedure model for sample extraction solvent preparation, used in various laboratory and field procedures.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
		.unwrap()
}
