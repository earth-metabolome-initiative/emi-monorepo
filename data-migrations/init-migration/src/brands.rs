//! Submodule defining the init migrations for the brands.

use core_structures::{Brand, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

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
pub(crate) async fn fisher_scientific(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<Brand, crate::error::Error> {
    match Brand::from_name("Fisher Scientific", portal_conn).await? {
        Some(brand) => Ok(brand),
        None => {
            Ok(Brand::new()
                .name("Fisher Scientific")?
                .created_by(user.id)?
                .build()?
                .backend_insert(portal_conn)
                .await?)
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
pub(crate) async fn acros_organics(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<Brand, crate::error::Error> {
    match Brand::from_name("Acros Organics", portal_conn).await? {
        Some(brand) => Ok(brand),
        None => {
            Ok(Brand::new()
                .name("Acros Organics")?
                .created_by(user.id)?
                .build()?
                .backend_insert(portal_conn)
                .await?)
        }
    }
}

/// Initializes the brands in the database.
pub(crate) async fn init_brands(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _fisher_scientific = fisher_scientific(user, portal_conn).await?;

    Ok(())
}
