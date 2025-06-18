//! Submodule initializing trackable categories.

use core_structures::User;
use diesel::PgConnection;

pub mod containers;
pub mod instruments;
pub mod organisms;
pub mod products;
pub mod reagents;
pub mod tools;

/// Initializes the trackable categories for the user.
///
/// # Arguments
///
/// * `user` - The user for whom the trackable categories are being initialized.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(super) fn init_trackables(user: &User, conn: &mut PgConnection) {
    reagents::init_reagents(user, conn);
    containers::init_containers(user, conn);
    instruments::init_instruments(user, conn);
    tools::init_tools(user, conn);
    organisms::init_organisms(user, conn);
}
