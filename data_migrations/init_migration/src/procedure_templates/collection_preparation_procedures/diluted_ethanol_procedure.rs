//! Submodule defining the Ethanol 70 percent procedure creation.

use core_structures::{
    PouringProcedureTemplate, ProcedureTemplate, ProcedureTemplateAssetModel, User,
    tables::insertables::{PouringProcedureTemplateSettable, ProcedureTemplateSettable},
    traits::AppendProcedureTemplate,
};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable, PrimaryKeyLike};

use crate::procedure_template_asset_models::{
    containers::bottle_1l_builder,
    reagents::{absolute_ethanol_model_builder, distilled_water_model_builder},
    volume_measuring_device::volume_measuring_device_model_builder,
};

use core_structures::tables::insertables::ProcedureTemplateAssetModelSettable;

/// Initializes the DBGI Collection preparation procedure template in the
/// database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure template.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure template.
/// * If the procedure template building fails.
pub fn ethanol_70_percent_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<(ProcedureTemplate, ProcedureTemplateAssetModel)> {
    const E70_ETHANOL: &str = "Ethanol 70 percent";
    const ETHANOL_BOTTLE: &str = "Ethanol bottle";

    if let Some(procedure) = ProcedureTemplate::from_name(E70_ETHANOL, conn).optional()? {
        let bottle = ProcedureTemplateAssetModel::from_name_and_procedure_template(
            ETHANOL_BOTTLE,
            procedure.primary_key(),
            conn,
        )?;
        return Ok((procedure, bottle));
    }

    let procedure = ProcedureTemplate::new()
    .name(E70_ETHANOL)?
    .description(
        "procedure template for Ethanol 70 percent Solvent preparation, used in various cleaning procedures.",
    )?
    .created_by(user)?
    .insert(user.id, conn)?;

    let ethanol_step = PouringProcedureTemplate::new()
        .name("Pour ethanol")?
        .description("Pour ethanol in the container.")?
        .procedure_template_measured_with_model(volume_measuring_device_model_builder(user, conn)?)?
        .procedure_template_poured_from_model(absolute_ethanol_model_builder(user, conn)?)?
        .procedure_template_poured_into_model(bottle_1l_builder(user, conn)?.name(ETHANOL_BOTTLE)?)?
        .liters(0.7)?
        .created_by(user)?
        .insert(user.id, conn)?;
    let measured_with = ethanol_step.procedure_template_measured_with_model(conn)?;
    let bottle = ethanol_step.procedure_template_poured_into_model(conn)?;

    let distilled_water_step = PouringProcedureTemplate::new()
        .name("Pour water")?
        .description("Pour water in the container.")?
        .procedure_template_measured_with_model(&measured_with)?
        .procedure_template_poured_from_model(distilled_water_model_builder(user, conn)?)?
        .procedure_template_poured_into_model(&bottle)?
        .liters(0.3)?
        .created_by(user)?
        .insert(user.id, conn)?;

    procedure.extend(&[ethanol_step.into(), distilled_water_step.into()], user, conn)?;

    Ok((procedure, bottle))
}
