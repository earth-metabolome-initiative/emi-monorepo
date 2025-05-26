//! Submodule creating a new Aliquoting step model.

use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelToolCategory, TrackableCategory,
    User,
};
use diesel_async::AsyncPgConnection;

use crate::{
    procedure_models::shared_step_models::init_aliquoting_step_model,
    trackable_categories::ETHANOL_95,
};

pub(super) async fn init_ethanol_aliquoting_step_model(
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

    let ethanol_95 =
        TrackableCategory::from_name(ETHANOL_95, portal_conn).await?.expect("Ethanol 95 not found");

    init_aliquoting_step_model(
        user,
        procedure,
        procedure_container_category,
        procedure_tool_categories,
        &ethanol_95,
        3.5,
        "Aliquoting 3.5 liters of Ethanol 95 to prepare the Ethanol 70 percent.",
        &aliquoting_materials_photograph,
        portal_conn,
    )
    .await?;

    Ok(())
}
