//! Submodule defining the DBGI plan procedure model.

use core_structures::{
    ProcedureModel, User,
    traits::{ChildOptions, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};
mod dbgi_collection_preparation;

/// The name of the DBGI plan procedure model.
pub const DBGI_PLAN: &str = "DBGI Plan";

/// Initializes the DBGI plan procedure model in the database.
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
pub(super) fn init_dbgi_plan(user: &User, conn: &mut diesel::PgConnection) {
    let dbgi_plan = ProcedureModel::new()
        .name(DBGI_PLAN)
        .unwrap()
        .description("DBGI Plan procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let collection_preparation =
        dbgi_collection_preparation::init_dbgi_collection_preparation(user, conn);

    dbgi_plan
        .child(&collection_preparation, ChildOptions::default().inherit_trackables(), user, conn)
        .unwrap();
}
