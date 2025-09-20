//! Submodule defining the functions to initialize `methanol` asset models.
use core_structures::{
    ReagentModel, User,
    tables::insertables::{AssetModelSettable, ReagentModelSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};
/// Returns the methanol reagent.
///
/// # Implementation Details
///
/// This function either instantiate a new methanol reagent from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the methanol is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn methanol_hplc(user: &User, conn: &mut PgConnection) -> anyhow::Result<ReagentModel> {
    let name = "Methanol, >= 99.8%, HPLC grade";

    if let Some(existing) = ReagentModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(ReagentModel::new()
        .name(name)?
        .description("Methanol, >= 99.8%, HPLC grade")?
        .purity(99.8)?
        .cas_code("67-56-1")?
        .molecular_formula("CH3OH")?
        .created_by(user)?
        .insert(user.id, conn)?)
}
