//! Submodule for negative ionization LC-MS procedures.

use core_structures::{ProcedureModel, User};
use web_common_traits::database::{Insertable, InsertableVariant};

const MS_MAINTENANCE: &str = "MS Maintenance";

pub(crate) fn init_ms_maintenance_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    if let Some(procedure) = ProcedureModel::from_name(MS_MAINTENANCE, conn).unwrap() {
        return procedure;
    }

    ProcedureModel::new()
        .name(MS_MAINTENANCE)
        .unwrap()
        .description("Procedure model for MS maintenance, used in various analytical procedures.")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()

    // Clean the source
    // * Disassemble the source
    // * Alumina powder used to clean most of the dirt
    // * Sonicate different part for 5min with 5% formic acid in water
    // * Sonic bath the source for 5min with pure distilled H20
    // * Sonic bath the source for 5min with MeOH
}
