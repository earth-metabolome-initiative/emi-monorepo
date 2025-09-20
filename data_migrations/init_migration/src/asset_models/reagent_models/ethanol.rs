//! Submodule defining the functions to initialize `ethanol` asset models.
use core_structures::{
    ReagentModel, User,
    tables::insertables::{AssetModelSettable, ReagentModelSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};
/// Returns the absolute ethanol reagent.
///
/// # Implementation Details
///
/// This function either instantiate a new absolute ethanol reagent from
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
pub fn absolute_ethanol(user: &User, conn: &mut PgConnection) -> anyhow::Result<ReagentModel> {
    let name = "Absolute Ethanol, >= 95%";

    if let Some(existing) = ReagentModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(ReagentModel::new()
        .name(name)?
        .description("Absolute Ethanol, >= 95%, with 5% isopropanol")?
        .purity(95.0)?
        .cas_code("64-17-5")?
        .molecular_formula("CH3CH2OH")?
        .created_by(user)?
        .insert(user.id, conn)?)
}
