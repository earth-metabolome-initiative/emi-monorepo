//! Submodule defining the DBGI collection preparation procedure model.

use core_structures::{ProcedureModel, User};
use web_common_traits::database::{Insertable, InsertableVariant};

/// The name of the DBGI Collection preparation procedure model.
pub const DBGI_COLLECTION_PREPARATION: &str = "DBGI Collection preparation";

/// Initializes the DBGI Collection preparation procedure model in the database.
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
pub(super) fn init_dbgi_collection_preparation(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    ProcedureModel::new()
        .name(DBGI_COLLECTION_PREPARATION)
        .unwrap()
        .description("DBGI Collection preparation procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}
