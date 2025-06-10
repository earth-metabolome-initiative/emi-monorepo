//! Submodule defining the full organism collection procedure model.

use core_structures::{ProcedureModel, User};
use web_common_traits::database::{Insertable, InsertableVariant};

/// The name of the full organism collection procedure model.
const FULL_ORGANISM_COLLECTION: &str = "Full organism collection procedure";

/// Initializes the full organism collection procedure model in the database.
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
pub(crate) fn init_full_organism_collection(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    if let Some(existing) = ProcedureModel::from_name(FULL_ORGANISM_COLLECTION, conn).unwrap() {
        return existing;
    }

    ProcedureModel::new()
        .name(FULL_ORGANISM_COLLECTION)
        .unwrap()
        .description(
            "Procedure model for collecting a full organism, including all its parts and tissues.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}
