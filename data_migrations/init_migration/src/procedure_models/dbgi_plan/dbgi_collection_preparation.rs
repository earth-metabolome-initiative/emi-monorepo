//! Submodule defining the DBGI collection preparation procedure model.

use core_structures::{
    ProcedureModel, User,
    traits::{AppendProcedureModel, ChildOptions, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::procedure_models::{init_ethanol_70_percent, init_sample_extraction_solvent_procedure};

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
    let dbgi_collection_preparation = ProcedureModel::new()
        .name(DBGI_COLLECTION_PREPARATION)
        .unwrap()
        .description("DBGI Collection preparation procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let make_ethanol_70 = init_ethanol_70_percent(user, conn);
    let make_solvent = init_sample_extraction_solvent_procedure(user, conn);

    dbgi_collection_preparation
        .child(&make_ethanol_70, ChildOptions::default().inherit_trackables(), user, conn)
        .unwrap();

    dbgi_collection_preparation
        .child(&make_solvent, ChildOptions::default().inherit_trackables(), user, conn)
        .unwrap();

    dbgi_collection_preparation.extend(&[&make_ethanol_70, &make_solvent], user, conn).unwrap();

    dbgi_collection_preparation
}
