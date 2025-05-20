//! Submodule handling the case where the user is not registered in the system.

use core_structures::{LoginProvider, TemporaryUser};
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

use crate::BackendError;

pub(super) async fn handle_unknown_user(
    email: &str,
    provider: &LoginProvider,
    conn: &mut crate::Conn,
) -> Result<TemporaryUser, BackendError> {
    // If the user is not registered, we create a new temporary user.
    let temporary_user = TemporaryUser::new()
        .email(email)?
        .login_provider_id(provider.id)?
        .build()?
        .backend_insert(conn)
        .await?;

    Ok(temporary_user)
}
