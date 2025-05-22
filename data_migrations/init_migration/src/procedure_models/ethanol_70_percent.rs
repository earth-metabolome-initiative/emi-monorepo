use container_categories::ContainerCategory;
use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelReagent,
    ProcedureModelToolCategory, ProcedureStepModel, Reagent, StepModel, User,
};
use diesel_async::AsyncPgConnection;
use tool_categories::ToolCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable, InsertableVariant},
    prelude::Builder,
};

pub(super) async fn init_ethanol_70_percent(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let ethanol_70: ProcedureModel = ProcedureModel::new()
        .name("Ethanol 70%")?
        .description(
            "Ethanol used to sterilize all sorts of tools/instruments in the EMI project.",
        )?
        .icon("blender")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    ProcedureModelToolCategory::new()
        .procedure_model_id(ethanol_70.id)?
        .tool_category(ToolCategory::VolumeMeasuringTool)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    ProcedureModelContainerCategory::new()
        .procedure_model_id(ethanol_70.id)?
        .container_category(ContainerCategory::Bottle)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let ethanol = Reagent::from_name("Absolute Ethanol, >= 95%", portal_conn)
        .await?
        .expect("Absolute Ethanol reagent not found");
    let distilled_water = Reagent::from_name("Distilled water", portal_conn)
        .await?
        .expect("Distilled water reagent not found");

    for reagent in vec![ethanol, distilled_water] {
        ProcedureModelReagent::new()
            .procedure_model_id(ethanol_70.id)?
            .reagent_id(reagent.id)?
            .created_by(user.id)?
            .build()?
            .insert(&user.id, portal_conn)
            .await?;
    }

    let cleaning_step = StepModel::from_name("Cleaning materials", portal_conn)
        .await?
        .expect("Cleaning step model not found");

    for step_model in &[cleaning_step] {
        ProcedureStepModel::new()
            .procedure_model_id(ethanol_70.id)?
            .step_model_id(step_model.id)?
            .created_by(user.id)?
            .build()?
            .insert(&user.id, portal_conn)
            .await?;
    }

    Ok(())
}
