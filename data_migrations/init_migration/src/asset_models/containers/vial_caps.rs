//! Submodule to initialize the

use core_structures::{
    CommercialProduct, ContainerModel, User,
    tables::insertables::{AssetModelBuildable, CommercialProductBuildable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

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
) -> anyhow::Result<ContainerModel> {
    let name = "Splitted Cap for Vial 1.5ml";

    if let Some(existing_tube) = ContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }

    Ok(ContainerModel::new()
        .name(name.to_owned())?
        .description("Splitted cap for Vial of 1.5 ml used for extracts storage".to_owned())?
        .created_by(user.id)?
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
) -> anyhow::Result<ContainerModel> {
    let name = "Sealed Cap for Vial 1.5ml";
    if let Some(existing_tube) = ContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }
    Ok(ContainerModel::new()
        .name(name.to_owned())?
        .description("Sealed cap for Vial of 1.5 ml used for extracts storage".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Splitted cap
pub(crate) fn init_macherey_nagel_splitted_cap(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let splitted_cap = "Machinery Nagel Splitted Cap 1.5ml";
    if let Some(splitted_cap) = CommercialProduct::from_name(splitted_cap, conn).optional()? {
        return Ok(splitted_cap);
    }

    let splitted_cap_trackable = splitted_cap_vial_1_5ml(user, conn)?;
    let macherey_nagel = macherey_nagel(user, conn)?;
    Ok(CommercialProduct::new()
        .name(splitted_cap.to_owned())?
        .description("Machinery Nagel Splitted Cap 1.5ml, used to partially seal Vial 1.5ml and allows mass spectrometry analysis.".to_owned())?
        .created_by(user.id)?
        .brand(macherey_nagel.id)?
        .parent_model(Some(splitted_cap_trackable.id))?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a sealed cap
pub(crate) fn init_advion_interchim_sealed_cap(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let sealed_cap = "Avion Interchim Sealed Cap 1.5ml";
    if let Some(sealed_cap) = CommercialProduct::from_name(sealed_cap, conn).optional()? {
        return Ok(sealed_cap);
    }

    let sealed_cap_trackable = sealed_cap_vial_1_5ml(user, conn)?;
    let advion_interchim = advion_interchim(user, conn)?;
    Ok(CommercialProduct::new()
        .name(sealed_cap.to_owned())?
        .description(
            "Avion Interchim Sealed Cap 1.5ml, used to seal Vial 1.5ml for storage.".to_owned(),
        )?
        .created_by(user.id)?
        .brand(advion_interchim.id)?
        .parent_model(Some(sealed_cap_trackable.id))?
        .insert(user.id, conn)?)
}
