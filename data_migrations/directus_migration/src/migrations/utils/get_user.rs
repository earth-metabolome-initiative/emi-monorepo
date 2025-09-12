//! Submodule providing a method to retrieve or insert a user in the database.

use core_structures::{
    User, UserEmail,
    tables::insertables::{UserEmailSettable, UserSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use directus_codegen::{DirectusUser, FieldDatum as DirectusFieldDatum};

/// This module provides a method to retrieve or insert a user in the database.
pub fn get_user(
    directus_user: &DirectusUser,
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> anyhow::Result<User> {
    let email: &str = directus_user.email.as_deref().expect("Directus user must have an email");
    if let Some(stored_email) = UserEmail::from_email(email, portal_conn).optional()? {
        return Ok(stored_email.created_by(portal_conn)?);
    }

    let last_access = directus_user.last_access.expect("Directus user must have last access date");

    let imputed_created_at =
        DirectusFieldDatum::from_user_created(directus_user.id, directus_conn)?
            .into_iter()
            .filter_map(|field_datum| field_datum.date_created)
            .min()
            .unwrap_or(last_access);

    let new_user = User::new()
        .first_name(
            directus_user.first_name.clone().expect("Directus user must have a first name"),
        )?
        .last_name(directus_user.last_name.clone().expect("Directus user must have a last name"))?
        .created_at(imputed_created_at)?
        .updated_at(last_access)?
        .insert(0, portal_conn)?;

    let _new_email = UserEmail::new()
        .created_by(new_user.id)?
        .email(directus_user.email.clone().expect("Directus user must have an email"))?
        .created_at(imputed_created_at)?
        .primary_email(true)?
        .insert(new_user.id, portal_conn)?;

    Ok(new_user)
}
