//! Submodule to initialize the

use core_structures::{
    CapModel, CommercialCapModel, User,
    tables::insertables::{
        AssetModelSettable, CommercialCapModelSettable, CommercialProductSettable,
    },
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

use crate::brands::{advion_interchim, macherey_nagel};

/// Returns the splitted cap for vial of 1.5 ml.
///
/// # Implementation Details
///
/// This function either instantiate a new splitted cap for vial of 1.5 ml from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the splitted vial cap that is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn splitted_cap_vial_1_5ml(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CapModel> {
    let name = "Splitted Cap for Vial 1.5ml";

    if let Some(existing_tube) = CapModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }

    Ok(CapModel::new()
        .name(name)?
        .description("Splitted cap for Vial of 1.5 ml used for extracts storage")?
        .created_by(user)?
        .insert(user.id, conn)?)
}

/// Returns the sealed cap for vial of 1.5 ml.
///
/// # Implementation Details
///
/// This function either instantiate a new sealed cap for vial of 1.5 ml from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the sealed vial cap that is being created.
/// * `conn` - The database connection.
///
/// # Errors
/// * If the connection to the database fails.
pub(crate) fn sealed_cap_vial_1_5ml(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CapModel> {
    let name = "Sealed Cap for Vial 1.5ml";
    if let Some(existing_tube) = CapModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }
    Ok(CapModel::new()
        .name(name)?
        .description("Sealed cap for Vial of 1.5 ml used for extracts storage")?
        .created_by(user)?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Splitted cap
pub(crate) fn init_macherey_nagel_splitted_cap(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialCapModel> {
    let splitted_cap_name = "Machinery Nagel Splitted Cap 1.5ml";
    if let Some(splitted_cap) = CommercialCapModel::from_name(splitted_cap_name, conn).optional()? {
        return Ok(splitted_cap);
    }

    let splitted_cap = splitted_cap_vial_1_5ml(user, conn)?;
    let macherey_nagel = macherey_nagel(user, conn)?;
    Ok(CommercialCapModel::new()
        .name(splitted_cap_name)?
        .description("Machinery Nagel Splitted Cap 1.5ml, used to partially seal Vial 1.5ml and allows mass spectrometry analysis.")?
        .created_by(user)?
        .brand(macherey_nagel)?
        .cap_model(splitted_cap)?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a sealed cap
pub(crate) fn init_advion_interchim_sealed_cap(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialCapModel> {
    let sealed_cap_name = "Avion Interchim Sealed Cap 1.5ml";
    if let Some(sealed_cap) = CommercialCapModel::from_name(sealed_cap_name, conn).optional()? {
        return Ok(sealed_cap);
    }

    let sealed_cap = sealed_cap_vial_1_5ml(user, conn)?;
    let advion_interchim = advion_interchim(user, conn)?;
    Ok(CommercialCapModel::new()
        .name(sealed_cap_name)?
        .description("Avion Interchim Sealed Cap 1.5ml, used to seal Vial 1.5ml for storage.")?
        .created_by(user)?
        .brand(advion_interchim)?
        .cap_model(sealed_cap)?
        .insert(user.id, conn)?)
}
