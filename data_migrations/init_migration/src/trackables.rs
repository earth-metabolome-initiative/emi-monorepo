//! Submodule initializing trackable categories.

use core_structures::User;
use diesel::PgConnection;

pub mod containers;
pub mod reagents;

/// Initializes the trackable categories for the user.
///
/// # Arguments
///
/// * `user` - The user for whom the trackable categories are being initialized.
/// * `portal_conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(super) fn init_trackables(user: &User, portal_conn: &mut PgConnection) {
    reagents::init_reagents(user, portal_conn);
    containers::init_containers(user, portal_conn);
}
