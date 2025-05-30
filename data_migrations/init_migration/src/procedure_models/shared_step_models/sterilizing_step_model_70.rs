//! Submodule defining the init migrations for the step models.

use core_structures::{StepModel, User, create_photograph};
use diesel_async::AsyncPgConnection;
use step_model_categories::StepModelCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(super) async fn init_sterilizing_step_model_70(
    darwin: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let cleaning_materials_photograph =
        create_photograph(include_bytes!("../../images/cleaning.jpg"), darwin, portal_conn).await?;

    let _cleaning_materials_step_model = StepModel::new()
        .name("Sterilizing materials")?
        .description("Sterilizing materials used in the EMI project with >70% ethanol.")?
        .icon("hands-bubbles")?
        .photograph_id(cleaning_materials_photograph.id)?
        .snoozable(true)?
        .copiable(true)?
        .step_model_category(StepModelCategory::Cleaning)?
        .created_by(darwin.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
