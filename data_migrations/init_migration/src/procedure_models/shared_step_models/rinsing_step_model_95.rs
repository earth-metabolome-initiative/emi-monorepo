//! Submodule defining the init migrations for the step models.

use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, StepModel, StepModelContainerCategory,
    StepModelTrackableCategory, TrackableCategory, User, create_photograph,
};
use diesel_async::AsyncPgConnection;
use step_model_categories::StepModelCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable, InsertableVariant},
    prelude::Builder,
};

use crate::trackable_categories::ETHANOL_95;

/// Initializes the rinsing step model for ethanol 95% in the given procedure.
///
/// # Arguments
///
/// * `user` - The user who is creating the step model.
/// * `procedure` - The procedure model to which the step model belongs.
/// * `procedure_container_category` - The container category for the procedure.
/// * `portal_conn` - The database connection to use for inserting the step
///   model.
///
/// # Errors
///
/// * If there is an error creating the photograph or inserting the step model,
///   it will return an error.
pub(crate) async fn init_rinsing_step_model_95(
    user: &User,
    procedure: &ProcedureModel,
    procedure_container_category: &ProcedureModelContainerCategory,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let rinsing_materials_photograph =
        create_photograph(include_bytes!("../../../images/cleaning.jpg"), user, portal_conn)
            .await?;

    let rinsing_materials_step_model = StepModel::new()
        .name("Rinsing container with ethanol")?
        .description("Rinse container with absolute ethanol >= 95% and let it dry.")?
        .icon("spray-can-sparkles")?
        .procedure_model_id(procedure.id)?
        .photograph_id(rinsing_materials_photograph.id)?
        .created_by(user.id)?
        .snoozable(true)?
        .copiable(true)?
        .step_model_category(StepModelCategory::Cleaning)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_container_category = StepModelContainerCategory::new()
        .step_model_id(rinsing_materials_step_model.id)?
        .procedure_model_container_category_id(procedure_container_category.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let ethanol_95 =
        TrackableCategory::from_name(ETHANOL_95, portal_conn).await?.expect("Ethanol 95 not found");

    let _step_model_reagent = StepModelTrackableCategory::new()
        .id(rinsing_materials_step_model.id)?
        .trackable_category_id(ethanol_95.id)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    Ok(())
}
