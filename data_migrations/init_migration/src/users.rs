//! Submodule defining the init migrations for the users.

use core_structures::{User, tables::insertables::UserSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{BackendInsertableVariant, Insertable, Read};

/// Returns the root user, creating it if it does not exist.
///
/// # Arguments
///
/// * `conn` - The database connection to use.
///
/// # Errors
///
/// * If the user could not be read or created.
pub fn init_root_user(conn: &mut PgConnection) -> anyhow::Result<User> {
    if let Some(user) = User::read(0, conn).optional()? {
        return Ok(user);
    }

    let user: User =
        User::new().last_name("Darwin")?.first_name("Charles")?.backend_insert(conn)?;

    Ok(user)
}
