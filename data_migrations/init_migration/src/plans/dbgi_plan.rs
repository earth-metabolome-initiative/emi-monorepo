//! Submodule defining the primary procedure used for the DBGI project.

use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

use crate::procedure_models::{
    ALIQUOTING_MASS_SPEC_EXTRACTS, COLLECTION_PROCEDURE, DATA_ENRICHMENT_PROCEDURE,
    EXTRACTION_PROCEDURE, FREEZE_DRYING, FREEZING_PROCEDURE, MASS_SPEC_PROCEDURE,
    PRECOLLECTION_PROCEDURE, WEIGHING_PROCEDURE,
};

/// Initializes the DBGI plan.
pub(super) async fn init_dbgi_plan(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let dbgi_plan: ProcedureModel = ProcedureModel::new()
        .name("DBGI Plan")?
        .description("The primary procedure used for the DBGI project")?
        .icon("diagram-project")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    for procedure_name in [
        PRECOLLECTION_PROCEDURE,
        COLLECTION_PROCEDURE,
        FREEZING_PROCEDURE,
        FREEZE_DRYING,
        WEIGHING_PROCEDURE,
        EXTRACTION_PROCEDURE,
        ALIQUOTING_MASS_SPEC_EXTRACTS,
        MASS_SPEC_PROCEDURE,
        DATA_ENRICHMENT_PROCEDURE,
    ] {
        let _parent_child = dbgi_plan
            .child(
                &ProcedureModel::from_name(procedure_name, portal_conn)
                    .await?
                    .unwrap_or_else(|| panic!("Procedure model '{procedure_name}' not found")),
                user,
                portal_conn,
            )
            .await?;
    }

    Ok(())
}
