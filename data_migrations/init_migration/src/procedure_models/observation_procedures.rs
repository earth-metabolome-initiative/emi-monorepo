//! Submodule defining the DBGI plan procedure model.

use core_structures::{
    ProcedureModel, User,
    traits::{ChildOptions, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};
mod organism_observation_procedure;

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
pub(super) fn init_observation_procedures(user: &User, conn: &mut diesel::PgConnection) {
    organism_observation_procedure::init_organism_observation_procedure(user, conn);
}
