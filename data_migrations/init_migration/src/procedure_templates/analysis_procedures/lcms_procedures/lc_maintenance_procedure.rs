//! Submodule for negative ionization LC-MS procedures.

use core_structures::{ProcedureTemplate, User, tables::insertables::ProcedureTemplateSettable};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};
const LC_MAINTENANCE: &str = "LC Maintenance";

pub(crate) fn init_lc_maintenance_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    if let Some(procedure) = ProcedureTemplate::from_name(LC_MAINTENANCE, conn).optional()? {
        return Ok(procedure);
    }

    Ok(ProcedureTemplate::new()
        .name(LC_MAINTENANCE)?
        .description(
            "procedure template for LC maintenance, used in various analytical procedures.",
        )?
        .created_by(user)?
        .insert(user.id, conn)?)

    // Check that the solvents on top of the LCMS are full
    // * Needle cleaner solvent: Magic mix 1:1:1:1 ACN:H2O:MeOH:IPA
    // * Column cleaner solvent: Rear seal wash (ACN:H20 90:10) -> Methanol
    // * Mobile phase A solvent: H2O + 0.1 Formic Acid
    // * Mobile phase B solvent: ACN + 0.1 Formic Acid
    // Prime LC system
    // Conditioning the column
    // Register column pressure
}
