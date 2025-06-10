//! Submodule defining the DBGI plan procedure model.

use core_structures::traits::AppendProcedureModel;
use core_structures::{
    ProcedureModel, User,
    traits::{ChildOptions, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::procedure_models::{
    init_full_organism_collection, init_negative_ionization_lcms_procedure,
    init_part_of_organism_collection, init_positive_ionization_lcms_procedure,
};
mod dbgi_collection_preparation;
mod sample_processing_procedures;

/// The name of the DBGI plan procedure model.
pub const DBGI_PLAN: &str = "DBGI Plan";

/// Initializes the DBGI plan procedure model in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure model.
/// * If the procedure model building fails.
pub(super) fn init_dbgi_plan(user: &User, conn: &mut diesel::PgConnection) {
    let dbgi_plan = ProcedureModel::new()
        .name(DBGI_PLAN)
        .unwrap()
        .description("DBGI Plan procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let collection_preparation =
        dbgi_collection_preparation::init_dbgi_collection_preparation(user, conn);
    let sample_processing_procedure =
        sample_processing_procedures::init_dbgi_sample_processing_procedures(user, conn);
    let positive_lcms_procedure = init_positive_ionization_lcms_procedure(user, conn);
    let negative_lcms_procedure = init_negative_ionization_lcms_procedure(user, conn);
    let full_organism_collection = init_full_organism_collection(user, conn);
    let part_of_organism_collection = init_part_of_organism_collection(user, conn);

    for procedure in [
        &collection_preparation,
        &full_organism_collection,
        &part_of_organism_collection,
        &sample_processing_procedure,
        &positive_lcms_procedure,
        &negative_lcms_procedure,
    ] {
        dbgi_plan
            .child(procedure, ChildOptions::default().inherit_trackables(), user, conn)
            .unwrap();
    }

    dbgi_plan
        .extend(
            &[
                &collection_preparation,
                &full_organism_collection,
                &sample_processing_procedure,
                &positive_lcms_procedure,
                &negative_lcms_procedure,
            ],
            user,
            conn,
        )
        .unwrap();

    dbgi_plan
        .extend(
            &[&collection_preparation, &part_of_organism_collection, &sample_processing_procedure],
            user,
            conn,
        )
        .unwrap();
}
