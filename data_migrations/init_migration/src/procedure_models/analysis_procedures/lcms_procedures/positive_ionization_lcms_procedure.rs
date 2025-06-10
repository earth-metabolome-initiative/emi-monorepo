//! Submodule for positive ionization LC-MS procedures.

use core_structures::{ProcedureModel, User};
use web_common_traits::database::Insertable;
use web_common_traits::database::InsertableVariant;

const POSITIVE_IONIZATION_LCMS: &str = "Positive Ionization LC-MS";

pub(crate) fn init_positive_ionization_lcms_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    if let Some(procedure) = ProcedureModel::from_name(POSITIVE_IONIZATION_LCMS, conn).unwrap() {
        return procedure;
    }

    ProcedureModel::new()
		.name(POSITIVE_IONIZATION_LCMS)
		.unwrap()
		.description(
			"Procedure model for Positive Ionization LC-MS analysis, used in various analytical procedures.",
		)
		.unwrap()
		.created_by(user.id)
		.unwrap()
		.insert(user.id, conn)
		.unwrap()
}
