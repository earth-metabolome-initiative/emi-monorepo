use core_structures::{
    CommercialProduct, PipetteModel, User,
    tables::insertables::{AssetModelSettable, CommercialProductSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::brands::gilson;

/// Returns a 200μl pipette.
///
/// # Implementation Details
///
/// This function either instantiate a new 200μl pipette from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the 200μl pipette is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_200ul(user: &User, conn: &mut PgConnection) -> anyhow::Result<PipetteModel> {
    let name = "Pipette 200μl";

    if let Some(existing) = PipetteModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(PipetteModel::new()
        .name(name)?
        .description(
            "A pipette used to manipulate liquids (needs to be equipped with a pipette tip).",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns a 1000µl pipette.
///
/// # Implementation Details
///
/// This function either instantiate a new 1000µl pipette from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the 1000µl pipette is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_1000ul(user: &User, conn: &mut PgConnection) -> anyhow::Result<PipetteModel> {
    let name = "Pipette 1000µl";

    if let Some(existing) = PipetteModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(PipetteModel::new()
        .name(name)?
        .description(
            "A pipette used to manipulate liquids (needs to be equipped with a pipette tip).",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns and possibly initializes the Gilson pipettes 200
/// trackable in the database.
pub(crate) fn init_gilson_pipette_200(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Gilson pipette 200";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let pipette = pipette_200ul(user, conn)?;
    let brand = gilson(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name)?
        .description("Gilson pipette 200, used to precipitate solid material.")?
        .created_by(user.id)?
        .parent_model(Some(pipette.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}

/// Returns and possibly initializes the Gilson pipette 1000 instrument
/// trackable in the database.
pub(crate) fn init_gilson_pipette_1000(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Gilson Pipette 1000";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let pipette = pipette_1000ul(user, conn)?;
    let brand = gilson(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name)?
        .description("Gilson Pipette 1000μl to manipulate liquids.")?
        .created_by(user.id)?
        .parent_model(Some(pipette.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}
