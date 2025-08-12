//! Submodule to initialize the

use core_structures::{ContainerModel, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

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
) -> anyhow::Result<ContainerModel> {
    let name = "Coffee Filter Wrapper";

    if let Some(existing_wrapper) = ContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_wrapper);
    }

    Ok(ContainerModel::new()
        .name(name.to_owned())?
        .description(
            "Coffee filters used to wrap sample in the field prior to storage in Falcon tubes"
                .to_owned(),
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
