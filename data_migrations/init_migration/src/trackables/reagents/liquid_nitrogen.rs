use core_structures::{Reagent, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

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
pub(crate) fn liquid_nitrogen(user: &User, conn: &mut PgConnection) -> anyhow::Result<Reagent> {
    let name = "Liquid nitrogen";

    if let Some(existing) = Reagent::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(Reagent::new()
        .name(name.to_owned())?
        .description("Liquid nitrogen, pure".to_owned())?
        .purity(100.0)?
        .cas_code("7727-37-9")?
        .molecular_formula("N2")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
