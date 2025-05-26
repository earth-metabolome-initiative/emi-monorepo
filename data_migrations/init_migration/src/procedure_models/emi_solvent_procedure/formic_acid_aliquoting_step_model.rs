//! Submodule creating a new Aliquoting step model.

use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelToolCategory, TrackableCategory,
    User,
};
use diesel_async::AsyncPgConnection;

use crate::{
    procedure_models::shared_step_models::init_aliquoting_step_model,
    trackable_categories::FORMIC_ACID,
};

pub(super) async fn init_formic_acid_aliquoting_step_model(
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

    let formic_acid = TrackableCategory::from_name(FORMIC_ACID, portal_conn)
        .await?
        .expect("Formic Acid not found");

    init_aliquoting_step_model(
        user,
        procedure,
        procedure_container_category,
        procedure_tool_categories,
        &formic_acid,
        5e-4, // 0.5 ml in liters
        "Aliquoting 0.5 ml of Formic Acid to prepare the EMI solvent.",
        &aliquoting_materials_photograph,
        portal_conn,
    )
    .await?;

    Ok(())
}
