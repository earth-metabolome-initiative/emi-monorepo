//! Submodule defining the sample collection procedures in the
//! database.

/// Initializes the sample collection procedures in the database.
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
pub(super) fn init_data_enrichment_procedures(
    _user: &core_structures::User,
    _conn: &mut diesel::PgConnection,
) {
}
