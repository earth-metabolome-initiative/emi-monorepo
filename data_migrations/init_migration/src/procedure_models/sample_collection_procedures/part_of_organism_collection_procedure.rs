//! Submodule defining the Ethanol 70 percent procedure creation.

use core_structures::{ProcedureModel, User};
use web_common_traits::database::{Insertable, InsertableVariant};

/// The name of the part of organism collection procedure model.
const PART_OF_ORGANISM: &str = "Part of organisms collection procedure";

/// Initializes the part of organism collection procedure model in the database.
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
pub(crate) fn init_part_of_organism_collection(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    if let Some(existing) = ProcedureModel::from_name(PART_OF_ORGANISM, conn).unwrap() {
        return existing;
    }

    ProcedureModel::new()
        .name(PART_OF_ORGANISM)
        .unwrap()
        .description(
            "Procedure model to collect part of organisms, such as leaves, stems, or roots.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}
