//! Submodule creating a new Aliquoting step model.

use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelToolCategory, TrackableCategory,
    User,
};
use diesel_async::AsyncPgConnection;

use crate::{
    procedure_models::shared_step_models::init_aliquoting_step_model,
    trackable_categories::METHANOL_HPLC,
};

pub(super) async fn init_methanol_aliquoting_step_model(
    user: &User,
    procedure: &ProcedureModel,
    procedure_container_category: &ProcedureModelContainerCategory,
    procedure_tool_categories: &[&ProcedureModelToolCategory],
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let aliquoting_materials_photograph = core_structures::create_photograph(
        include_bytes!("../../../images/cleaning.jpg"),
        user,
        portal_conn,
    )
    .await?;

    let methanol_hplc = TrackableCategory::from_name(METHANOL_HPLC, portal_conn)
        .await?
        .expect("Methanol HPLC not found");

    init_aliquoting_step_model(
        user,
        procedure,
        procedure_container_category,
        procedure_tool_categories,
        &methanol_hplc,
        0.4,
        "Aliquoting 0.4 liters of Methanol HPLC to prepare the EMI solvent.",
        &aliquoting_materials_photograph,
        portal_conn,
    )
    .await?;

    Ok(())
}
