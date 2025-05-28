use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) const EXTRACTION_PROCEDURE: &str = "Extraction procedure";

pub(super) async fn init_extraction_procedure(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _extraction: ProcedureModel = ProcedureModel::new()
        .name(EXTRACTION_PROCEDURE)?
        .description("Extract the weighed dried material using solvent")?
        .icon("mortar-pestle")?
        .repeatable(true)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
