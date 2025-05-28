use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) const FREEZE_DRYING: &str = "Freeze drying";

pub(super) async fn init_freeze_drying_procedure(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _freeze_drying: ProcedureModel = ProcedureModel::new()
        .name(FREEZE_DRYING)?
        .description("Dry the samples after the collection process.")?
        .icon("snowflake")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
