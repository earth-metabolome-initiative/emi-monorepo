//! Submodule creating a new Aliquoting step model.

use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelToolCategory, TrackableCategory,
    User,
};
use diesel_async::AsyncPgConnection;

use crate::{
    procedure_models::shared_step_models::init_aliquoting_step_model,
    trackable_categories::DISTILLED_WATER,
};

pub(crate) async fn init_water_aliquoting_step_model(
    user: &User,
    procedure: &ProcedureModel,
    procedure_container_category: &ProcedureModelContainerCategory,
    procedure_tool_categories: &[&ProcedureModelToolCategory],
    liters: f32,
    description: &str,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let aliquoting_materials_photograph = core_structures::create_photograph(
        include_bytes!("../../../images/cleaning.jpg"),
        user,
        portal_conn,
    )
    .await?;

    let distilled_water = TrackableCategory::from_name(DISTILLED_WATER, portal_conn)
        .await?
        .expect("Distilled Water not found");

    init_aliquoting_step_model(
        user,
        procedure,
        procedure_container_category,
        procedure_tool_categories,
        &distilled_water,
        liters,
        description,
        &aliquoting_materials_photograph,
        portal_conn,
    )
    .await?;

    Ok(())
}
