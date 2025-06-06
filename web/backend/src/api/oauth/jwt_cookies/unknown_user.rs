//! Submodule handling the case where the user is not registered in the system.

use core_structures::{LoginProvider, TemporaryUser};
use web_common_traits::database::{Insertable, UncheckedInsertableVariant};

use crate::BackendError;

pub(super) fn handle_unknown_user(
    email: &str,
    provider: &LoginProvider,
    conn: &mut crate::Conn,
) -> Result<TemporaryUser, BackendError> {
    // We check whether there is already a temporary user with the same email and
    // login provider.
    if let Some(existing_user) =
        TemporaryUser::from_email_and_login_provider_id(email, &provider.id, conn)?
    {
        // If such a user exists, we return it.
        return Ok(existing_user);
    }

    // If the user is not registered, we create a new temporary user.
    let temporary_user = TemporaryUser::new()
        .email(email)?
        .login_provider_id(provider.id)?
        .unchecked_insert(conn)?;

    Ok(temporary_user)
}
