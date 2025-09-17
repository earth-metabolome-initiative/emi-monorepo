//! Submodule to initialize the

use core_structures::{PackagingModel, User, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

/// Returns the coffee filter wrapper.
///
/// # Implementation Details
///
/// This function either instantiate a new coffee filter wrapper from the
/// database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the conical tube is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn coffee_filter_wrapper(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<PackagingModel> {
    let name = "Coffee Filter Wrapper";

    if let Some(existing_wrapper) = PackagingModel::from_name(name, conn).optional()? {
        return Ok(existing_wrapper);
    }

    Ok(PackagingModel::new()
        .name(name)?
        .description(
            "Coffee filters used to wrap sample in the field prior to storage in Falcon tubes",
        )?
        .created_by(user)?
        .insert(user.id, conn)?)
}
