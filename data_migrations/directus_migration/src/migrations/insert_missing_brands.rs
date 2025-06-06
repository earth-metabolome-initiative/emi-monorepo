//! Submodule providing a method to migrate the brands from the Directus
//! database to the new database.

use core_structures::Brand as PortalBrand;
use diesel::PgConnection;
use web_common_traits::database::{BoundedRead, Insertable, InsertableVariant};

use super::get_user;
use crate::codegen::Brand as DirectusBrand;

/// Inserts missing brands into the portal database.
///
/// # Arguments
///
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the portal database connection.
///
/// # Errors
///
/// * If the insertion fails, an error of type `error::Error` is returned.
pub fn insert_missing_brands(
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> Result<(), crate::error::Error> {
    let directus_brands = DirectusBrand::bounded_read(0, u16::MAX, directus_conn)?;

    for directus_brand in directus_brands {
        if PortalBrand::from_name(&directus_brand.brand, portal_conn)?.is_some() {
            continue;
        }

        let directus_created_by = directus_brand.user_created(directus_conn)?.ok_or_else(|| {
            crate::error::Error::BrandWithMissingUser(Box::from(directus_brand.clone()))
        })?;
        let portal_created_by = match get_user(&directus_created_by, directus_conn, portal_conn) {
            Ok(user) => user,
            Err(crate::error::Error::UserNeverLoggedIn(_)) => {
                continue;
            }
            Err(error) => {
                return Err(error);
            }
        };
        let directus_updated_by = directus_brand
            .user_updated(directus_conn)?
            .unwrap_or_else(|| directus_created_by.clone());
        let portal_updated_by = match get_user(&directus_updated_by, directus_conn, portal_conn) {
            Ok(user) => user,
            Err(crate::error::Error::UserNeverLoggedIn(_)) => {
                continue;
            }
            Err(error) => {
                return Err(error);
            }
        };

        let created_at = directus_brand.date_created.ok_or_else(|| {
            crate::error::Error::MissingDate(
                "directus_brands".to_owned(),
                "date_created".to_owned(),
            )
        })?;

        let _portal_brand = PortalBrand::new()
            .created_by(portal_created_by.id)?
            .created_at(created_at)?
            .updated_at(directus_brand.date_updated.unwrap_or(created_at))?
            .updated_by(portal_updated_by.id)?
            .name(directus_brand.brand.clone())?
            .insert(portal_created_by.id, portal_conn)?;
    }

    Ok(())
}
