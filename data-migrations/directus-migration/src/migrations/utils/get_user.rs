//! Submodule providing a method to retrieve or insert a user in the database.

use diesel_async::AsyncPgConnection;

use crate::codegen::{DirectusUser, FieldDatum as DirectusFieldDatum};
use core_structures::{User, UserEmail};
use web_common_traits::database::{Insertable, InsertableVariant};
use web_common_traits::prelude::Builder;

/// This module provides a method to retrieve or insert a user in the database.
pub async fn get_user(
    directus_user: &DirectusUser,
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<User, crate::error::Error> {
    let email: &str = directus_user
        .email
        .as_deref()
        .ok_or(crate::error::Error::MissingEmail(directus_user.id))?;
    if let Some(stored_email) = UserEmail::from_email(email, portal_conn).await? {
        return Ok(stored_email.created_by(portal_conn).await?);
    }

    let last_access = directus_user
        .last_access
        .ok_or(crate::error::Error::UserNeverLoggedIn(Box::from(directus_user.clone())))?;

    let imputed_created_at: chrono::DateTime<chrono::Utc> =
        DirectusFieldDatum::from_user_created(directus_conn, directus_user)
            .await?
            .into_iter()
            .filter_map(|field_datum| field_datum.date_created)
            .min()
            .unwrap_or(last_access);

    let new_user = User::new()
        .first_name(
            directus_user
                .first_name
                .clone()
                .ok_or(crate::error::Error::MissingFirstName(directus_user.id))?,
        )?
        .last_name(
            directus_user
                .last_name
                .clone()
                .ok_or(crate::error::Error::MissingLastName(directus_user.id))?,
        )?
        .created_at(imputed_created_at)?
        .updated_at(last_access)?
        .build()?
        .insert(portal_conn)
        .await?;

    let _new_email = UserEmail::new()
        .created_by(new_user.id)?
        .email(
            directus_user
                .email
                .clone()
                .ok_or(crate::error::Error::MissingEmail(directus_user.id))?,
        )?
        .created_at(imputed_created_at)?
        .primary_email(true)?
        .build()?
        .insert(portal_conn)
        .await?;

    Ok(new_user)
}
