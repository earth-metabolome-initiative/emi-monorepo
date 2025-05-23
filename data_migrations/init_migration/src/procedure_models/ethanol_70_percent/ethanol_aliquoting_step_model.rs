//! Submodule creating a new Aliquoting step model.

use core_structures::{
    AliquotingStepModel, ProcedureModel, ProcedureModelContainerCategory, Reagent,
    SamplingStepModel, StepModel, StepModelContainerCategory, StepModelReagent, User,
};
use diesel_async::AsyncPgConnection;
use step_model_categories::StepModelCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

use crate::reagents::ETHANOL_95;

pub(super) async fn init_ethanol_aliquoting_step_model(
    darwin: &User,
    procedure: &ProcedureModel,
    procedure_container_category: &ProcedureModelContainerCategory,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let aliquoting_materials_photograph = core_structures::create_photograph(
        include_bytes!("../../../images/cleaning.jpg"),
        darwin,
        portal_conn,
    )
    .await?;

    let aliquoting_materials_step_model = StepModel::new()
        .procedure_model_id(procedure.id)?
        .name("Aliquoting absolute ethanol")?
        .description("Aliquoting absolute ethanol >= 95% with a graduated cylinder")?
        // https://fontawesome.com/icons/flask-vial?f=classic&s=solid
        .icon("flask-vial")?
        .photograph_id(aliquoting_materials_photograph.id)?
        .snoozable(true)?
        .copiable(true)?
        .step_model_category(StepModelCategory::Aliquoting)?
        .created_by(darwin.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_container_category = StepModelContainerCategory::new()
        .step_model_id(aliquoting_materials_step_model.id)?
        .procedure_model_container_category_id(procedure_container_category.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let ethanol_95 =
        Reagent::from_name(ETHANOL_95, portal_conn).await?.expect("Ethanol 95 not found");

    let _step_model_reagent = StepModelReagent::new()
        .id(aliquoting_materials_step_model.id)?
        .reagent_id(ethanol_95.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let sampling_step_model = SamplingStepModel::new()
        .id(aliquoting_materials_step_model.id)?
        .created_by(darwin.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _aliquoting_step_model = AliquotingStepModel::new()
        .id(sampling_step_model.id)?
        .liters(3.5)?
        .created_by(darwin.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
