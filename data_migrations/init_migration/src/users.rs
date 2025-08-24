//! Submodule defining the init migrations for the users.

use core_structures::User;
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant, Read};

/// Returns the root user, creating it if it does not exist.
pub fn init_root_user(conn: &mut PgConnection) -> anyhow::Result<User> {
    if let Some(user) = User::read(0, conn)? {
        return Ok(user);
    }

    let user: User = User::new().last_name("Darwin")?.first_name("Charles")?.insert(0, conn)?;

    Ok(user)
}
