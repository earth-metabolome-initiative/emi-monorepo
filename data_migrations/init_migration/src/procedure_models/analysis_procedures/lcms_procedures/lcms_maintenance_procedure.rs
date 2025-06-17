//! Submodule for negative ionization LC-MS procedures.

use core_structures::{ProcedureModel, User};
use web_common_traits::database::{Insertable, InsertableVariant};

const LCMS_MAINTENANCE: &str = "LC-MS Maintenance";

pub(crate) fn init_lcms_maintenance_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    if let Some(procedure) = ProcedureModel::from_name(LCMS_MAINTENANCE, conn).unwrap() {
        return procedure;
    }

    ProcedureModel::new()
		.name(LCMS_MAINTENANCE)
		.unwrap()
		.description(
			"Procedure model for Negative Ionization LC-MS analysis, used in various analytical procedures.",
		)
		.unwrap()
		.created_by(user.id)
		.unwrap()
		.insert(user.id, conn)
		.unwrap()

    // Check that the solvents on top of the LCMS are full
    // * Needle cleaner solvent: Magic mix 1:1:1:1 ACN:H2O:MeOH:IPA
    // * Column cleaner solvent: Rear seal wash (ACN:H20 90:10)
    // * Mobile phase A solvent: H2O + 0.1 Formic Acid
    // * Mobile phase B solvent: ACN + 0.1 Formic Acid

    // Clean the source
    // Prime LC system
    // Conditioning the column
    // Register column pressure
}
