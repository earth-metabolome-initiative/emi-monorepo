//! Submodule creating the instrument commercial product model for the Ball Mill
//! instrument.

use core_structures::{
    BallMillMachineModel, CommercialProduct, User,
    tables::insertables::{AssetModelSettable, CommercialProductSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::brands::retsch;

/// Returns the ball mill machine.
///
/// # Implementation Details
///
/// This function either instantiate a new ball mill machine from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the conical tube is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn ball_mill_machine(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<BallMillMachineModel> {
    let name = "Ball Mill Machine";

    if let Some(existing) = BallMillMachineModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(BallMillMachineModel::new()
        .name(name)?
        .description("A Ball Mill Machine used to grind samples into powder.")?
        .created_by(user)?
        .insert(user.id, conn)?)
}

/// Returns and possibly initializes the Retsch MM 400 Ball Mill instrument
/// trackable in the database.
pub(crate) fn init_retsch_mm400(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Retsch MM 400 Ball Mill";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let ball_mill = ball_mill_machine(user, conn)?;
    let brand = retsch(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name)?
        .description("Retsch MM 400 Ball Mill, used to reduce solid material into powder.")?
        .created_by(user)?
        .parent_model(ball_mill)?
        .brand(brand)?
        .insert(user.id, conn)?)
}
