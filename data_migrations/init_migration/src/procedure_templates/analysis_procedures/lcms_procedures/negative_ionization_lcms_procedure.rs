//! Submodule for negative ionization LC-MS procedures.

use core_structures::{ProcedureTemplate, User, tables::insertables::ProcedureTemplateSettable};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};
const NEGATIVE_IONIZATION_LCMS: &str = "Negative Ionization LC-MS";

pub(crate) fn init_negative_ionization_lcms_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    if let Some(procedure) =
        ProcedureTemplate::from_name(NEGATIVE_IONIZATION_LCMS, conn).optional()?
    {
        return Ok(procedure);
    }

    Ok(ProcedureTemplate::new()
		.name(NEGATIVE_IONIZATION_LCMS)?
		.description(
			"procedure template for Negative Ionization LC-MS analysis, used in various analytical procedures.",
		)?
		.created_by(user.id)?
		.insert(user.id, conn)?)

    // Execute calibration with Thermo Fisher calibration solution
    // Register one minute of running the LCMS with blank sample
}
