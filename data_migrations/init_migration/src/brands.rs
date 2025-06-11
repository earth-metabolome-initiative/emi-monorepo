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
    if let Some(brand) = Brand::from_name("Fisher Scientific", portal_conn)? {
        return Ok(brand);
    }
    Ok(Brand::new().name("Fisher Scientific")?.created_by(user.id)?.insert(user.id, portal_conn)?)
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
    let brand_name = "Acros Organics";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }
    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes the greiner BIO-ONE brand in the database.
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
pub(crate) fn greiner_bio_one(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "greiner BIO-ONE";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes MACHINERY-NAGEL `GmbH` & Co. KG brand in the database.
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
pub(crate) fn machinery_nagel(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "MACHINERY-NAGEL GmbH & Co. KG";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes VICI Schweiz AG brand in the database.
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
pub(crate) fn vici_schweiz(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "VICI Schweiz AG";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes Advion Interchim scientific brand in the database.
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
pub(crate) fn advion_interchim(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "Advion Interchim scientific";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes eppendorf brand in the database.
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
pub(crate) fn eppendorf(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "eppendorf";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes GILSON brand in the database.
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
pub(crate) fn gilson(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "gilson";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes SARSTEDT AG & Co. KG brand in the database.
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
pub(crate) fn sarstedt(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "SARSTEDT AG & Co. KG";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes AXYGEN brand in the database.
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
pub(crate) fn axygen(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "AXYGEN";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}

/// Initializes Retsch MILLING & SIEVING brand in the database.
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
pub(crate) fn retsch(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<Brand, crate::error::Error> {
    let brand_name = "Retsch MILLING & SIEVING";
    if let Some(brand) = Brand::from_name(brand_name, portal_conn)? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, portal_conn)?)
}
