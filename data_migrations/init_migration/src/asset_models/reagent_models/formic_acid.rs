use core_structures::{
    ReagentModel, User,
    tables::insertables::{AssetModelBuildable, ReagentModelBuildable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};
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
pub(crate) fn formic_acid(user: &User, conn: &mut PgConnection) -> anyhow::Result<ReagentModel> {
    let name = "Formic acid";

    if let Some(existing) = ReagentModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(ReagentModel::new()
        .name(name.to_owned())?
        .description("Formic acid, pure".to_owned())?
        .purity(98.0)?
        .cas_code("64-18-6")?
        .molecular_formula("HCOOH")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
