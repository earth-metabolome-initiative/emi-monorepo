use core_structures::{Reagent, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

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
pub(crate) fn absolute_ethanol(user: &User, conn: &mut PgConnection) -> anyhow::Result<Reagent> {
    let name = "Absolute Ethanol, >= 95%";

    if let Some(existing) = Reagent::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(Reagent::new()
        .name(name.to_owned())?
        .description("Absolute Ethanol, >= 95%, with 5% isopropanol".to_owned())?
        .purity(95.0)?
        .cas_code("64-17-5")?
        .molecular_formula("CH3CH2OH")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
