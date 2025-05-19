//! Submodule defining the init migrations for the users.

use core_structures::User;
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) async fn init_root_user(
    portal_conn: &mut AsyncPgConnection,
) -> Result<User, crate::error::Error> {
    let user: User = User::new()
        .last_name("Darwin")?
        .first_name("Charles")?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(user)
}
