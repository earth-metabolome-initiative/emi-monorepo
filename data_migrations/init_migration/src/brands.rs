//! Submodule defining the init migrations for the brands.

use core_structures::{Brand, User, tables::insertables::BrandBuildable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

/// Initializes the Fisher Scientific brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn fisher_scientific(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    if let Some(brand) = Brand::from_name("Fisher Scientific", conn).optional()? {
        return Ok(brand);
    }
    Ok(Brand::new().name("Fisher Scientific")?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes the Acros Organics brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn acros_organics(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "Acros Organics";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }
    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes the greiner BIO-ONE brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn greiner_bio_one(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "greiner BIO-ONE";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes MACHINERY-NAGEL `GmbH` & Co. KG brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn macherey_nagel(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "MACHINERY-NAGEL GmbH & Co. KG";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes VICI Schweiz AG brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn vici_schweiz(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "VICI Schweiz AG";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes Advion Interchim scientific brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn advion_interchim(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "Advion Interchim scientific";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes eppendorf brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn eppendorf(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "eppendorf";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes GILSON brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn gilson(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "gilson";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes SARSTEDT AG & Co. KG brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn sarstedt(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "SARSTEDT AG & Co. KG";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes AXYGEN brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn axygen(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "AXYGEN";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes Retsch MILLING & SIEVING brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn retsch(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "Retsch MILLING & SIEVING";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}

/// Initializes Fisherbrand brand in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the brand.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the brand cannot be created.
pub(crate) fn fisherbrand(user: &User, conn: &mut PgConnection) -> anyhow::Result<Brand> {
    let brand_name = "Fisherbrand";
    if let Some(brand) = Brand::from_name(brand_name, conn).optional()? {
        return Ok(brand);
    }

    Ok(Brand::new().name(brand_name)?.created_by(user.id)?.insert(user.id, conn)?)
}
