//! Submodule defining the init migrations for the brands.

use core_structures::{Brand, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

/// Initializes the Fisher Scientific brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `portal_conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn fisher_scientific(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    match Brand::from_name("Fisher Scientific", portal_conn)? {
        Some(brand) => Ok(brand),
        None => {
            Ok(Brand::new()
                .name("Fisher Scientific")?
                .created_by(user.id)?
                .insert(user.id, portal_conn)?)
        }
    }
}

/// Initializes the Acros Organics brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `portal_conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn acros_organics(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    match Brand::from_name("Acros Organics", portal_conn)? {
        Some(brand) => Ok(brand),
        None => {
            Ok(Brand::new()
                .name("Acros Organics")?
                .created_by(user.id)?
                .insert(user.id, portal_conn)?)
        }
    }
}

/// Initializes the brands in the database.
pub(crate) fn init_brands(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<(), crate::error::Error> {
    let _fisher_scientific = fisher_scientific(user, portal_conn)?;
    let _acros_organics = acros_organics(user, portal_conn)?;

    Ok(())
}
