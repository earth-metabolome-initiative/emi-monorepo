use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) const ALIQUOTING_MASS_SPEC_EXTRACTS: &str = "Aliquoting mass spec extracts procedure";

pub(super) async fn init_aliquoting_mass_spec_extracts_procedure(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _extraction: ProcedureModel = ProcedureModel::new()
        .name(ALIQUOTING_MASS_SPEC_EXTRACTS)?
        .description("Create the samples that will be analyzed through mass spectrometry")?
        .icon("syringe")?
        .repeatable(true)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
