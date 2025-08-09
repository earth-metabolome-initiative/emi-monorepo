//! Submodule defining the sample collection procedures in the
//! database.

use core_structures::ProcedureModel;
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};
const WEATHER_RETRIEVAL_PROCEDURE: &str = "Weather Retrieval Procedure";

/// Initializes the weather retrieval procedure model in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure models.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure models.
/// * If the procedure model building fails.
pub(crate) fn init_weather_retrieval_procedure(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureModel> {
    if let Some(procedure) =
        ProcedureModel::from_name(WEATHER_RETRIEVAL_PROCEDURE, conn).optional()?
    {
        return Ok(procedure);
    }

    Ok(ProcedureModel::new()
        .name(WEATHER_RETRIEVAL_PROCEDURE)
        ?
        .description(
            "Procedure model for retrieving and storing weather data associated with sample collection events. This procedure ensures that relevant meteorological information is linked to each sample, supporting downstream data enrichment and analysis."
        )
        ?
        .created_by(user.id)
        ?
        .insert(user.id, conn)
        ?)
}
