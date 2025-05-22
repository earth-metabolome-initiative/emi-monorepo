use container_categories::ContainerCategory;
use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelReagent,
    ProcedureModelToolCategory, Reagent, User,
};
use diesel_async::AsyncPgConnection;
use tool_categories::ToolCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable, InsertableVariant},
    prelude::Builder,
};

pub(super) async fn init_emi_solvent_procedure_models(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let emi_solvent_procedure: ProcedureModel = ProcedureModel::new()
        .name("EMI Solvent 0.5L")?
        .description("A procedure to create half a liter of EMI Solvent.")?
        .icon("blender")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    ProcedureModelToolCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .tool_category(ToolCategory::VolumeMeasuringTool)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    ProcedureModelContainerCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .container_category(ContainerCategory::Bottle)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let methanol = Reagent::from_name("Methanol, >= 99.8%", portal_conn)
        .await?
        .expect("Methanol reagent not found");
    let formic_acid = Reagent::from_name("Formic acid, 98+%", portal_conn)
        .await?
        .expect("Formic acid reagent not found");
    let distilled_water = Reagent::from_name("Distilled water", portal_conn)
        .await?
        .expect("Distilled water reagent not found");

    for reagent in vec![methanol, formic_acid, distilled_water] {
        ProcedureModelReagent::new()
            .procedure_model_id(emi_solvent_procedure.id)?
            .reagent_id(reagent.id)?
            .created_by(user.id)?
            .build()?
            .insert(&user.id, portal_conn)
            .await?;
    }

    Ok(())
}
