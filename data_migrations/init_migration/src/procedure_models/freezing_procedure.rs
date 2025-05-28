use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) const FREEZING_PROCEDURE: &str = "Freezing";

pub(super) async fn init_freezing_procedure(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _freezing: ProcedureModel = ProcedureModel::new()
        .name(FREEZING_PROCEDURE)?
        .description("Freeze freshly collected samples")?
        .icon("snowflake")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
