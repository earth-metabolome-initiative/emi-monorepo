//! Submodule defining the analysis procedures in the
//! database.

mod lcms_procedures;

/// Initializes the analysis procedures in the database.
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
pub(super) fn init_analysis_procedures(
    _user: &core_structures::User,
    _conn: &mut diesel::PgConnection,
) {
}
