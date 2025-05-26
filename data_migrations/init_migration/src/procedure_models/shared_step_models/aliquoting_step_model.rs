//! Submodule creating a new Aliquoting step model.

use core_structures::{
    AliquotingStepModel, Document, ProcedureModel, ProcedureModelContainerCategory,
    ProcedureModelToolCategory, SamplingStepModel, StepModel, StepModelContainerCategory,
    StepModelToolCategory, StepModelTrackableCategory, TrackableCategory, User,
};
use diesel_async::AsyncPgConnection;
use step_model_categories::StepModelCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable, InsertableVariant},
    prelude::Builder,
};

#[allow(clippy::too_many_arguments)]
pub(crate) async fn init_aliquoting_step_model(
    user: &User,
    procedure: &ProcedureModel,
    procedure_container_category: &ProcedureModelContainerCategory,
    procedure_tool_categories: &[&ProcedureModelToolCategory],
    trackable_category: &TrackableCategory,
    liters: f32,
    description: &str,
    photograph: &Document,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let aliquoting_materials_step_model = StepModel::new()
        .procedure_model_id(procedure.id)?
        .name(format!("Aliquoting {}", trackable_category.name))?
        .description(description)?
        // https://fontawesome.com/icons/flask-vial?f=classic&s=solid
        .icon("flask-vial")?
        .photograph_id(photograph.id)?
        .snoozable(true)?
        .copiable(true)?
        .step_model_category(StepModelCategory::Aliquoting)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await
        .expect("Failed to insert aliquoting step model");

    let _step_model_container_category = StepModelContainerCategory::new()
        .step_model_id(aliquoting_materials_step_model.id)?
        .procedure_model_container_category_id(procedure_container_category.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_trackable_category = StepModelTrackableCategory::new()
        .id(aliquoting_materials_step_model.id)?
        .trackable_category_id(trackable_category.id)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    for procedure_tool_category in procedure_tool_categories {
        let _step_model_tool_category = StepModelToolCategory::new()
            .step_model_id(aliquoting_materials_step_model.id)?
            .procedure_model_tool_category_id(procedure_tool_category.id)?
            .created_by(user.id)?
            .build()?
            .insert(&user.id, portal_conn)
            .await?;
    }

    let sampling_step_model = SamplingStepModel::new()
        .id(aliquoting_materials_step_model.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _aliquoting_step_model = AliquotingStepModel::new()
        .id(sampling_step_model.id)?
        .liters(liters)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
