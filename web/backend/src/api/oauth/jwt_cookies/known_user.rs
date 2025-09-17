//! Submodule handling the case where one of the provided emails for the given
//! provider is already registered in the system.

use core_structures::{
    EmailProvider, LoginProvider, User, UserEmail,
    tables::insertables::{EmailProviderSettable, UserEmailSettable},
};
use diesel::OptionalExtension;
use web_common_traits::database::{BackendInsertableVariant, Insertable, Read};

use crate::BackendError;

/// Handles the case where one of the provided emails for the given provider is
/// already registered in the system.
///
/// # Arguments
///
/// * `emails` - A slice of email strings.
/// * `provider` - A reference to the `LoginProvider` struct.
/// * `conn` - A mutable reference to the database connection.
///
/// # Errors
///
/// * If there is a collision between two users, an error is returned.
/// * If there is an error while inserting a new email or email provider, an
///   error is returned.
pub(super) fn handle_known_user(
    emails: &[&str],
    provider: &LoginProvider,
    conn: &mut crate::Conn,
) -> Result<Option<User>, BackendError> {
    // There may be multiple users retrievable with the set of provided emails.
    let mut users: Vec<User> = Vec::new();
    let mut email_not_registered_with_provider = Vec::new();
    let mut unregistered_emails = Vec::new();

    for email in emails {
        if let Some(user_email) = UserEmail::from_email(email, conn).optional()? {
            let user = user_email.created_by(conn)?;
            if !users.contains(&user) {
                users.push(user);
            }

            if EmailProvider::read((user_email.id, provider.id), conn).optional()?.is_none() {
                email_not_registered_with_provider.push(user_email);
            }
        } else {
            unregistered_emails.push(email);
        }
    }

    // If there are multiple users, we return a collision error.
    // This case may be best handled in the future by merging the users.
    if users.len() > 1 {
        return Err(BackendError::LoginCollision);
    }

    let Some(user) = users.pop() else {
        return Ok(None);
    };

    // We insert the emails that are not registered with the provider.
    for user_email in email_not_registered_with_provider {
        let _ = EmailProvider::new()
            .email(user_email)?
            .login_provider(provider)?
            .backend_insert(conn)?;
    }

    // If there is only one user, and there is one or more emails which are not
    // inserted with the user or the provider, we insert the emails and then
    // we also insert the provider.
    for unregistered_email in unregistered_emails {
        let newly_inserted_email = UserEmail::new()
            .created_by(user.id)?
            .email(*unregistered_email)?
            .primary_email(false)?
            .backend_insert(conn)?;
        let _ = EmailProvider::new()
            .email(newly_inserted_email)?
            .login_provider(provider)?
            .backend_insert(conn)?;
    }

    Ok(Some(user))
}
