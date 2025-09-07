//! Submodule defining the sample collection procedures in the
//! database.

use core_structures::{ProcedureTemplate, traits::ParentProcedureTemplate};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};
mod weather_retrieval_procedure;
use core_structures::tables::insertables::ProcedureTemplateSettable;
const DATA_ENRICHMENT_PROCEDURES: &str = "Data Enrichment Procedure";

/// Initializes the sample collection procedures in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure templates.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure templates.
/// * If the procedure template building fails.
pub(crate) fn init_data_enrichment_procedure(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    if let Some(procedure) =
        ProcedureTemplate::from_name(DATA_ENRICHMENT_PROCEDURES, conn).optional()?
    {
        return Ok(procedure);
    }

    let data_enrichment_procedure = ProcedureTemplate::new()
		.name(DATA_ENRICHMENT_PROCEDURES)?
		.description(
			"procedure template for Negative Ionization LC-MS analysis, used in various analytical procedures.",
		)?
		.created_by(user.id)?
		.insert(user.id, conn)?;

    let weather_retrieval_procedure =
        weather_retrieval_procedure::init_weather_retrieval_procedure(user, conn)?;

    data_enrichment_procedure.child(&weather_retrieval_procedure, user, conn)?;

    Ok(data_enrichment_procedure)
}
