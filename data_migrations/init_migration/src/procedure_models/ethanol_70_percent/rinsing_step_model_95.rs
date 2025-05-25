//! Submodule defining the init migrations for the step models.

use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, Reagent, StepModel,
    StepModelContainerCategory, StepModelReagent, User, create_photograph,
};
use diesel_async::AsyncPgConnection;
use step_model_categories::StepModelCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

use crate::reagents::ETHANOL_95;

pub(super) async fn init_rinsing_step_model_95(
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
        .description("Rinse the glass container with absolute ethanol >= 95% and let it dry.")?
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
        Reagent::from_name(ETHANOL_95, portal_conn).await?.expect("Ethanol 95 not found");

    let _step_model_reagent = StepModelReagent::new()
        .id(rinsing_materials_step_model.id)?
        .reagent_id(ethanol_95.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
