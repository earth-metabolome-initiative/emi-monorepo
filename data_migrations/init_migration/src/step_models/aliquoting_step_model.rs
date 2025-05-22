//! Submodule creating a new Aliquoting step model.

use core_structures::{AliquotingStepModel, StepModel, User};
use diesel_async::AsyncPgConnection;
use step_model_categories::StepModelCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(super) async fn init_aliquoting_step_model(
    darwin: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let aliquoting_materials_step_model = StepModel::new()
        .name("Aliquoting materials")?
        .description("Aliquoting materials used in the EMI project")?
        // https://fontawesome.com/icons/flask-vial?f=classic&s=solid
        .icon("flask-vial")?
        .snoozable(true)?
        .copiable(true)?
        .step_model_category(StepModelCategory::Aliquoting)?
        .created_by(darwin.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _aliquoting_step_model = AliquotingStepModel::new()
        .id(aliquoting_materials_step_model.id)?
        .liters(1.0)?
        .created_by(darwin.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
