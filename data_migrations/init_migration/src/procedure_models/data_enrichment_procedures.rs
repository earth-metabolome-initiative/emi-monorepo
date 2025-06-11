//! Submodule defining the sample collection procedures in the
//! database.

use core_structures::{
    ProcedureModel,
    traits::{ChildOptions, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};
mod weather_retrieval_procedure;

const DATA_ENRICHMENT_PROCEDURES: &str = "Data Enrichment Procedure";

/// Initializes the sample collection procedures in the database.
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
pub(crate) fn init_data_enrichment_procedure(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    if let Some(procedure) = ProcedureModel::from_name(DATA_ENRICHMENT_PROCEDURES, conn).unwrap() {
        return procedure;
    }

    let data_enrichment_procedure = ProcedureModel::new()
		.name(DATA_ENRICHMENT_PROCEDURES)
		.unwrap()
		.description(
			"Procedure model for Negative Ionization LC-MS analysis, used in various analytical procedures.",
		)
		.unwrap()
		.created_by(user.id)
		.unwrap()
		.insert(user.id, conn)
		.unwrap();

    let weather_retrieval_procedure =
        weather_retrieval_procedure::init_weather_retrieval_procedure(user, conn);

    data_enrichment_procedure
        .child(
            &weather_retrieval_procedure,
            ChildOptions::default().inherit_trackables(),
            user,
            conn,
        )
        .unwrap();

    data_enrichment_procedure
}
