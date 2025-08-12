//! Submodule defining the compatibility rules for conical centrifugal tubes.

use core_structures::traits::CompatibleWith;

use crate::trackables::containers::{
    conical_centrifugal_tubes::conical_centrifugal_tube_50ml,
    racks::conical_centrifugal_tube_50ml_rack,
};

pub(super) fn init_conical_centrifugal_tube_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let cct_50ml = conical_centrifugal_tube_50ml(user, conn)?;
    let cct_50ml_rack = conical_centrifugal_tube_50ml_rack(user, conn)?;

    // We can fit 24 conical centrifugal tubes of 50ml in a rack.
    cct_50ml_rack.compatible_with_quantity(&cct_50ml, 24, user, conn)?;
    Ok(())
}

// pub(super) fn init_conical_centrifugal_tubes(
//     user: &User,
//     container: &ContainerModel,
//     wet_lab_container: &ContainerModel,
//     conn: &mut PgConnection,
// ) -> anyhow::Result<()> {
//     conical_tube_rack.compatible_with_quantity(&conical_tube_50ml, 24, user,
// conn)?;

//     ContainerModel::from_name(POLYSTYRENE_BOX, conn)?.compatible_with(
//         &conical_tube_50ml,
//         user,
//         conn,
//     )?;

//     ContainerModel::from_name(SHELF,
// conn)?.compatible_with(&conical_tube_rack, user, conn)?;

//     Ok(())
// }
