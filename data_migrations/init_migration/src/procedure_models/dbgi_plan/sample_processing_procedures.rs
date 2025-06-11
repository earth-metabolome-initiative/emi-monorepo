//! Submodule defining procedures which are re-used in sample processing.

use core_structures::{
    FractioningProcedureModel, FreezeDryingProcedureModel, FreezingProcedureModel, ProcedureModel,
    ProcedureModelTrackable, Trackable,
    traits::{AppendProcedureModel, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::trackables::instruments::{FREEZE_DRYER, FREEZER};

pub(super) fn init_dbgi_sample_processing_procedures(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> ProcedureModel {
    let dbgi_sample_processing_procedure = ProcedureModel::new()
        .name("DBGI Sample Processing")
        .unwrap()
        .description("DBGI Sample Processing procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let freezing_procedure = FreezingProcedureModel::new()
        .name("DBGI Freezing")
        .unwrap()
        .description("DBGI Freezing procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .frozen_with(
            ProcedureModelTrackable::new()
                .name("Sample Freezer")
                .unwrap()
                .created_by(user.id)
                .unwrap()
                .trackable_id(Trackable::from_name(FREEZER, conn).unwrap().unwrap().id)
                .unwrap(),
        )
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let freeze_drying_procedure = FreezeDryingProcedureModel::new()
        .name("DBGI Freeze Drying")
        .unwrap()
        .description("DBGI Freeze Drying procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .freeze_dried_with(
            ProcedureModelTrackable::new()
                .name("Sample Freeze Dryer")
                .unwrap()
                .created_by(user.id)
                .unwrap()
                .trackable_id(Trackable::from_name(FREEZE_DRYER, conn).unwrap().unwrap().id)
                .unwrap(),
        )
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let fractioning_procedure = FractioningProcedureModel::new()
        .name("DBGI Fractioning")
        .unwrap()
        .description("DBGI Fractioning procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    dbgi_sample_processing_procedure
        .child(
            &freezing_procedure,
            core_structures::traits::ChildOptions::default().inherit_trackables(),
            user,
            conn,
        )
        .unwrap();
    dbgi_sample_processing_procedure
        .child(
            &freeze_drying_procedure,
            core_structures::traits::ChildOptions::default().inherit_trackables(),
            user,
            conn,
        )
        .unwrap();

    dbgi_sample_processing_procedure
        .extend(
            &[&freezing_procedure.id(conn).unwrap(), &freeze_drying_procedure.id(conn).unwrap()],
            user,
            conn,
        )
        .unwrap();

    dbgi_sample_processing_procedure
}
