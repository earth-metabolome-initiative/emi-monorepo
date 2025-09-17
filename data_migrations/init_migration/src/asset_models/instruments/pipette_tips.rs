//! Submodule defining the functions to initialize `pipette_tips` asset models.
use core_structures::{
    CommercialProduct, PipetteTipModel, User,
    tables::insertables::{AssetModelSettable, CommercialProductSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

use crate::brands::sarstedt;

/// Returns a 200μl pipette tip.
///
/// # Implementation Details
///
/// This function either instantiate a new 200μl pipette tip from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the 200μl pipette tip is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_tip_200ul(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<PipetteTipModel> {
    let name = "Pipette Tip 200μl";

    if let Some(existing) = PipetteTipModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(PipetteTipModel::new()
        .name(name)?
        .description(
            "A 200μl pipette tip used to manipulate and transfer liquids when adapted to a pipette",
        )?
        .created_by(user)?
        .insert(user.id, conn)?)
}

/// Returns a 1ml pipette tip.
///
/// # Implementation Details
///
/// This function either instantiate a new 1ml pipette tip from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the 1ml pipette tip is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_tip_1000ul(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<PipetteTipModel> {
    let name = "Pipette Tip 1ml";

    if let Some(existing) = PipetteTipModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(PipetteTipModel::new()
        .name(name)?
        .description("A 1000μl pipette tip used to manipulate and transfer liquids when adapted to a pipette")?
        .created_by(user)?
        .insert(user.id, conn)?)
}

/// Returns and possibly initializes the Gilson pipette 200 tip tool
/// trackable in the database.
pub(crate) fn init_sarstedt_pipette_tip_200(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Sarstedt Tip for Gilson Pipette 200μl";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let pipette_tip = pipette_tip_200ul(user, conn)?;
    let brand = sarstedt(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name)?
        .description("Tip for Gilson Pipette 200µl to manipulate liquids")?
        .created_by(user)?
        .parent_model(pipette_tip)?
        .brand(brand)?
        .insert(user.id, conn)?)
}

/// Returns and possibly initializes the Sarstedt Tip for Gilson pipette 1000
/// tool trackable in the database.
pub(crate) fn init_sarstedt_pipette_tip_1000(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Sarstedt Tip for Gilson Pipette 1000μl";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let pipette_tip = pipette_tip_1000ul(user, conn)?;
    let brand = sarstedt(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name)?
        .description("Sarstedt Tip for Gilson Pipette 1000μl.")?
        .created_by(user)?
        .parent_model(pipette_tip)?
        .brand(brand)?
        .insert(user.id, conn)?)
}
