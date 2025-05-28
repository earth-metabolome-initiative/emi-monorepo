use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) const WEIGHING_PROCEDURE: &str = "Weighing procedure";

pub(super) async fn init_weighing_procedure(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _freezing: ProcedureModel = ProcedureModel::new()
        .name(WEIGHING_PROCEDURE)?
        .description("Weight 50 milligrams of dried material")?
        .icon("weight-scale")?
        .repeatable(true)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
