//! Submodule creating the instrument commercial product model for the
//! Centrifuge instrument.

use core_structures::{CentrifugeModel, CommercialProduct, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::brands::eppendorf;

/// Returns the centrifuge instrument.
///
/// # Implementation Details
///
/// This function either instantiate a new centrifuge from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the centrifuge is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn safelock_centrifuge(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CentrifugeModel> {
    let name = "Safelock Centrifuge";

    if let Some(existing) = CentrifugeModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(CentrifugeModel::new()
        .name(name.to_owned())?
        .description("Safelock centrifuge, used to precipitate solid material.".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns and possibly initializes the Eppendorf centrifuge instrument
/// trackable in the database.
pub(crate) fn init_eppendorf_centrifuge(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Eppendorf centrifuge";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let centrifuge = safelock_centrifuge(user, conn)?;
    let brand = eppendorf(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name.to_owned())?
        .description("Eppendorf centrifuge, used to precipitate solid material.".to_owned())?
        .created_by(user.id)?
        .parent(Some(centrifuge.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}
