//! Submodule defining the functions to initialize `formic_acid` asset models.
use core_structures::{
    ReagentModel, User,
    tables::insertables::{AssetModelSettable, ReagentModelSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};
/// Returns the formic acid reagent.
///
/// # Implementation Details
///
/// This function either instantiate a new formic acid reagent from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the formic acid is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn formic_acid(user: &User, conn: &mut PgConnection) -> anyhow::Result<ReagentModel> {
    let name = "Formic acid";

    if let Some(existing) = ReagentModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(ReagentModel::new()
        .name(name)?
        .description("Formic acid, pure")?
        .purity(98.0)?
        .cas_code("64-18-6")?
        .molecular_formula("HCOOH")?
        .created_by(user)?
        .insert(user.id, conn)?)
}
