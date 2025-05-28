use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{Insertable, InsertableVariant},
    prelude::Builder,
};

pub(crate) const DATA_ENRICHMENT_PROCEDURE: &str = "Data Enrichment Procedure";

pub(super) async fn init_data_enrichment_procedure(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _freezing: ProcedureModel = ProcedureModel::new()
        .name(DATA_ENRICHMENT_PROCEDURE)?
        .description("Enrich data from mass spectrometry and other sources.")?
        .icon("database")?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    Ok(())
}
