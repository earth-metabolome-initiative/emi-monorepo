//! Submodule defining the init migrations for the users.

use core_structures::User;
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub(crate) fn init_root_user(conn: &mut PgConnection) -> anyhow::Result<User> {
    let user: User = User::new().last_name("Darwin")?.first_name("Charles")?.insert(0, conn)?;

    Ok(user)
}
