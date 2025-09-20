//! Submodule defining the functions to initialize `liquid_nitrogen` asset
//! models.
use core_structures::{
    ReagentModel, User,
    tables::insertables::{AssetModelSettable, ReagentModelSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};
/// Returns the liquid nitrogen reagent.
///
/// # Implementation Details
///
/// This function either instantiate a new liquid nitrogen reagent from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the freeze dryer is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn liquid_nitrogen(user: &User, conn: &mut PgConnection) -> anyhow::Result<ReagentModel> {
    let name = "Liquid nitrogen";

    if let Some(existing) = ReagentModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(ReagentModel::new()
        .name(name)?
        .description("Liquid nitrogen, pure")?
        .purity(100.0)?
        .cas_code("7727-37-9")?
        .molecular_formula("N2")?
        .created_by(user)?
        .insert(user.id, conn)?)
}
