//! Submodule defining the init migrations for the step models.

use core_structures::{StepModel, User};
use diesel_async::AsyncPgConnection;
use step_model_categories::StepModelCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(super) async fn init_cleaning_step_model_95(
    darwin: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _cleaning_materials_step_model = StepModel::new()
        .name("Cleaning materials")?
        .description("Cleaning materials used in the EMI project with >95% ethanol.")?
        .icon("spray-can-sparkles")?
        .created_by(darwin.id)?
        .snoozable(true)?
        .copiable(true)?
        .step_model_category(StepModelCategory::Cleaning)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
