use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{Insertable, InsertableVariant},
    prelude::Builder,
};

pub(crate) const MASS_SPEC_PROCEDURE: &str = "Mass Spec procedure";

pub(super) async fn init_mass_spec_procedure(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _freezing: ProcedureModel = ProcedureModel::new()
        .name(MASS_SPEC_PROCEDURE)?
        .description("Analyze samples using mass spectrometry.")?
        .icon("tachograph-digital")?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    Ok(())
}
