//! Submodule for negative ionization LC-MS procedures.

use core_structures::{ProcedureModel, User};
use web_common_traits::database::Insertable;
use web_common_traits::database::InsertableVariant;

const NEGATIVE_IONIZATION_LCMS: &str = "Negative Ionization LC-MS";

pub(crate) fn init_negative_ionization_lcms_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    if let Some(procedure) = ProcedureModel::from_name(NEGATIVE_IONIZATION_LCMS, conn).unwrap() {
        return procedure;
    }

    ProcedureModel::new()
		.name(NEGATIVE_IONIZATION_LCMS)
		.unwrap()
		.description(
			"Procedure model for Negative Ionization LC-MS analysis, used in various analytical procedures.",
		)
		.unwrap()
		.created_by(user.id)
		.unwrap()
		.insert(user.id, conn)
		.unwrap()
}
