//! Submodule for negative ionization LC-MS procedures.

use core_structures::{ProcedureModel, User};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

pub(crate) fn init_ms_maintenance_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureModel> {
    const MS_MAINTENANCE: &str = "MS Maintenance";

    if let Some(procedure) = ProcedureModel::from_name(MS_MAINTENANCE, conn).optional()? {
        return Ok(procedure);
    }

    Ok(ProcedureModel::new()
        .name(MS_MAINTENANCE)?
        .description("Procedure model for MS maintenance, used in various analytical procedures.")?
        .created_by(user.id)?
        .insert(user.id, conn)?)

    // Clean the source
    // * Disassemble the source
    // * Alumina powder used to clean most of the dirt
    // * Sonicate different part for 5min with 5% formic acid in water
    // * Sonic bath the source for 5min with pure distilled H20
    // * Sonic bath the source for 5min with MeOH
}
