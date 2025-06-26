//! Submodule defining the init migrations for the users.

use core_structures::User;
use diesel::PgConnection;
use web_common_traits::database::{Insertable, UncheckedInsertableVariant};

pub(crate) fn init_root_user(conn: &mut PgConnection) -> Result<User, crate::error::Error> {
    let user: User =
        User::new().last_name("Darwin")?.first_name("Charles")?.unchecked_insert(conn)?;

    Ok(user)
}
