use core_structures::{
    ReagentModel, User,
    tables::insertables::{AssetModelBuildable, ReagentModelBuildable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

/// Returns the distilled water reagent.
///
/// # Implementation Details
///
/// This function either instantiate a new distilled water reagent from
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
pub(crate) fn distilled_water(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<ReagentModel> {
    let name = "Distilled water";

    if let Some(existing) = ReagentModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(ReagentModel::new()
        .name(name.to_owned())?
        .description("Distilled water, pure".to_owned())?
        .purity(100.0)?
        .cas_code("7732-18-5")?
        .molecular_formula("H2O")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
