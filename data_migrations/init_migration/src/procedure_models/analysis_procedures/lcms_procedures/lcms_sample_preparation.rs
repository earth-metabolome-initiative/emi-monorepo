//! Submodule for negative ionization LC-MS procedures.

use core_structures::{ProcedureModel, User};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

const NEGATIVE_IONIZATION_LCMS: &str = "Negative Ionization LC-MS";

pub(crate) fn init_lcms_sample_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureModel> {
    if let Some(procedure) = ProcedureModel::from_name(NEGATIVE_IONIZATION_LCMS, conn).optional()? {
        return Ok(procedure);
    }

    Ok(ProcedureModel::new()
		.name(NEGATIVE_IONIZATION_LCMS)
		?
		.description(
			"Procedure model for Negative Ionization LC-MS analysis, used in various analytical procedures.",
		)
		?
		.created_by(user.id)
		?
		.insert(user.id, conn)
		?)

    // Take out the vial rack from the freezer
    // Add inserts inside the single use vials
    // Aliquoting 120ul the long term storage vials into short term single use
    // vials Put the long term storage vials back into the freezer
    // Add the splitted caps on the single use vials

    // Setup the LCMS sample list CSV
    // Setup the method files for the LCMS
}
