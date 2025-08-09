//! Submodule defining procedures which are re-used in sample processing.

use core_structures::{
    BallMillMachineModel, BallMillProcedureModel, CappingProcedureModel, CentrifugeModel,
    CentrifugeProcedureModel, ContainerModel, DisposalProcedureModel, FractioningProcedureModel,
    FreezeDryingProcedureModel, FreezerModel, FreezingProcedureModel, ProcedureModel,
    ProcedureModelTrackable, StorageProcedureModel, SupernatantProcedureModel, Trackable,
    VolumetricContainerModel, WeighingInstrumentModel,
    traits::{AppendProcedureModel, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    procedure_models::sample_collection_procedures::CONICAL_TUBE_BOX,
    trackables::{
        containers::{
            CONICAL_CENTRIFUGAL_TUBE_50ML, POLYSTYRENE_BOX, SAFELOCK_TUBE_2ML, VIAL_1_5ML,
            VIAL_1_5ML_SEALED_CAP,
            wet_lab_containers::{CONICAL_CENTRIFUGAL_TUBE_50ML_RACK, vials::VIAL_BOX},
        },
        instruments::{
            BALL_MILL_MACHINE, FREEZE_DRYER, FREEZER, PIPETTE_TIPS_1000, PIPETTES_1000,
            SAFELOCK_CENTRIFUGE, WEIGHING_SCALE,
        },
        tools::METAL_BEADS_3MM,
    },
};

pub(crate) const DBGI_CONICAL_TUBE: &str = "DBGI Conical Tube (Falcon)";
pub(crate) const DBGI_EPPENDORF_TUBE: &str = "DBGI Eppendorf Tube";

pub(super) fn init_dbgi_sample_processing_procedures(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureModel> {
    let dbgi_sample_processing_procedure = ProcedureModel::new()
        .name("DBGI Sample Processing")?
        .description("DBGI Sample Processing procedure model")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let weighing_scale = WeighingInstrumentModel::from_name(WEIGHING_SCALE, conn)?;
    let conical_tubes_box = ContainerModel::from_name(POLYSTYRENE_BOX, conn)?;

    let conical_tube = VolumetricContainerModel::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML, conn)?;
    let conical_tube_rack = ContainerModel::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML_RACK, conn)?;
    let safelock_tube = VolumetricContainerModel::from_name(SAFELOCK_TUBE_2ML, conn)?;
    let vial = VolumetricContainerModel::from_name(VIAL_1_5ML, conn)?;
    let vial_box = ContainerModel::from_name(VIAL_BOX, conn)?;
    let ball_mill = BallMillMachineModel::from_name(BALL_MILL_MACHINE, conn)?;
    let centrifuge = CentrifugeModel::from_name(SAFELOCK_CENTRIFUGE, conn)?;
    let pipette_1000 = Trackable::from_name(PIPETTES_1000, conn)?;
    let pipette_tips_1000 = Trackable::from_name(PIPETTE_TIPS_1000, conn)?;
    let sealed_cap = Trackable::from_name(VIAL_1_5ML_SEALED_CAP, conn)?;

    let procedure_weighing_scale_builder = ProcedureModelTrackable::new()
        .name(WEIGHING_SCALE)?
        .created_by(user.id)?
        .trackable(weighing_scale.id)?;

    let procedure_conical_tubes_box_builder = ProcedureModelTrackable::new()
        .name(CONICAL_TUBE_BOX)?
        .created_by(user.id)?
        .trackable(conical_tubes_box.id)?;

    let procedure_conical_tube_builder = ProcedureModelTrackable::new()
        .name(DBGI_CONICAL_TUBE)?
        .created_by(user.id)?
        .trackable(conical_tube.id)?;

    let procedure_conical_tube_rack_builder = ProcedureModelTrackable::new()
        .name("DBGI Conical Tube Rack")?
        .created_by(user.id)?
        .trackable(conical_tube_rack.id)?;

    let procedure_safelock_tube_builder = ProcedureModelTrackable::new()
        .name(DBGI_EPPENDORF_TUBE)?
        .created_by(user.id)?
        .trackable(safelock_tube.id)?;

    let procedure_centrifuge_builder = ProcedureModelTrackable::new()
        .name("DBGI Centrifuge")?
        .created_by(user.id)?
        .trackable(centrifuge.id)?;

    let procedure_ball_mill_builder = ProcedureModelTrackable::new()
        .name("DBGI Ball Mill")?
        .created_by(user.id)?
        .trackable(ball_mill.id)?;

    let pipette_1000_builder = ProcedureModelTrackable::new()
        .name("DBGI Pipette 1000")?
        .created_by(user.id)?
        .trackable(pipette_1000.id)?;

    let pipette_tips_1000_builder = ProcedureModelTrackable::new()
        .name("DBGI Pipette Tips 1000")?
        .created_by(user.id)?
        .trackable(pipette_tips_1000.id)?;

    let sealed_cap_builder = ProcedureModelTrackable::new()
        .name("DBGI Sealed Cap")?
        .created_by(user.id)?
        .trackable(sealed_cap.id)?;

    let long_term_storage_vial_builder = ProcedureModelTrackable::new()
        .name("DBGI Long Term Storage Vial")?
        .created_by(user.id)?
        .trackable(vial.id)?;

    let long_term_storage_vial_box_builder = ProcedureModelTrackable::new()
        .name("DBGI Long Term Storage Vial Box")?
        .created_by(user.id)?
        .trackable(vial_box.id)?;

    let freezer = ProcedureModelTrackable::new()
        .name("Standard -80C Freezer")?
        .created_by(user.id)?
        .trackable(FreezerModel::from_name(FREEZER, conn)?.id)?;

    let storage_freezer = ProcedureModelTrackable::new()
        .name("Sample Storage Freezer")?
        .created_by(user.id)?
        .trackable(FreezerModel::from_name(FREEZER, conn)?.id)?;

    let freezing_procedure = FreezingProcedureModel::new()
        .name("DBGI Freezing")?
        .description("DBGI Freezing procedure model")?
        .created_by(user.id)?
        .procedure_frozen_container(procedure_conical_tubes_box_builder.clone())?
        .procedure_frozen_with(freezer.clone())?
        .insert(user.id, conn)?;

    let freeze_drying_procedure = FreezeDryingProcedureModel::new()
        .name("DBGI Freeze Drying")?
        .description("DBGI Freeze Drying procedure model")?
        .created_by(user.id)?
        .procedure_freeze_dried_container(procedure_conical_tube_builder.clone())?
        .procedure_freeze_dried_with(
            ProcedureModelTrackable::new()
                .name("Sample Freeze Dryer")?
                .created_by(user.id)?
                .trackable(Trackable::from_name(FREEZE_DRYER, conn)?.id)?,
        )?
        .insert(user.id, conn)?;

    // Next, we store the lyophilized material contained in the conical tube
    // in the conical tube rack.
    let falcon_storage_procedure = StorageProcedureModel::new()
        .name("DBGI Falcon Storage")?
        .description("DBGI Falcon Storage procedure model")?
        .created_by(user.id)?
        .procedure_parent_container(procedure_conical_tube_rack_builder.clone())?
        .procedure_child_container(procedure_conical_tube_builder.clone())?
        .insert(user.id, conn)?;

    let fractioning_procedure = FractioningProcedureModel::new()
        .name("DBGI Fractioning")?
        .description("DBGI Fractioning procedure model")?
        .created_by(user.id)?
        .kilograms(50e-6)?
        .tolerance_percentage(5.0)?
        .procedure_fragment_source(procedure_conical_tube_builder)?
        .procedure_fragment_placed_into(procedure_safelock_tube_builder.clone())?
        .procedure_weighed_with(procedure_weighing_scale_builder)?
        .insert(user.id, conn)?;

    let metal_beads = Trackable::from_name(METAL_BEADS_3MM, conn)?;
    let procedure_metal_beads_builder = ProcedureModelTrackable::new()
        .name(METAL_BEADS_3MM)?
        .created_by(user.id)?
        .trackable(metal_beads.id)?;

    let first_ball_mill_procedure = BallMillProcedureModel::new()
        .name("DBGI Ball Mill 1")?
        .description("Ball Mill of lyophilized material procedure model")?
        .created_by(user.id)?
        .procedure_milled_container(procedure_safelock_tube_builder.clone())?
        .procedure_milled_with(procedure_ball_mill_builder.clone())?
        .insert(user.id, conn)?;

    let second_ball_mill_procedure = BallMillProcedureModel::new()
        .name("DBGI Ball Mill 2")?
        .description("Second Ball Mill to extract sample procedure model")?
        .created_by(user.id)?
        .procedure_milled_container(procedure_safelock_tube_builder.clone())?
        .procedure_milled_with(procedure_ball_mill_builder)?
        .insert(user.id, conn)?;

    let centrifuge_procedure = CentrifugeProcedureModel::new()
        .name("DBGI Centrifuge")?
        .description("Centrifuge procedure model to separate solid material from supernatant")?
        .created_by(user.id)?
        .procedure_centrifuged_container(procedure_safelock_tube_builder.clone())?
        .procedure_centrifuged_with(procedure_centrifuge_builder)?
        .insert(user.id, conn)?;

    let prelevate_supernatant_procedure = SupernatantProcedureModel::new()
        .name("DBGI Supernatant")?
        .description("Supernatant procedure model to collect supernatant")?
        .created_by(user.id)?
        .liters(1e-3)?
        .procedure_stratified_source(procedure_safelock_tube_builder.clone())?
        .procedure_supernatant_destination(long_term_storage_vial_builder.clone())?
        .procedure_transferred_with(pipette_1000_builder.clone())?
        .procedure_pipette_tip(pipette_tips_1000_builder.clone())?
        .insert(user.id, conn)?;

    let capping_procedure = CappingProcedureModel::new()
        .name("DBGI Capping")?
        .description("Capping procedure model to cap the long term storage vial")?
        .created_by(user.id)?
        .procedure_container(long_term_storage_vial_builder.clone())?
        .procedure_capped_with(sealed_cap_builder)?
        .insert(user.id, conn)?;

    // We store the long term storage vial in a box.
    let long_term_storage_vial_storage_procedure = StorageProcedureModel::new()
        .name("DBGI Long Term Storage Vial Storage")?
        .description("DBGI Long Term Storage Vial Storage procedure model")?
        .created_by(user.id)?
        .procedure_parent_container(long_term_storage_vial_box_builder.clone())?
        .procedure_child_container(long_term_storage_vial_builder)?
        .insert(user.id, conn)?;

    let dispose_of_eppendorf_tube_procedure = DisposalProcedureModel::new()
        .name("DBGI Dispose of Eppendorf Tube")
        ?
        .description("Disposal procedure model for Eppendorf tubes. You can separate the metal beads for further reuse at this step.")
        ?
        .created_by(user.id)
        ?
        .disposed(procedure_safelock_tube_builder)
        ?
        .insert(user.id, conn)
        ?;

    let dispose_of_pipette_tips_procedure = DisposalProcedureModel::new()
        .name("DBGI Dispose of Pipette Tips")?
        .description("Disposal procedure model for used pipette tips.")?
        .created_by(user.id)?
        .disposed(pipette_tips_1000_builder)?
        .insert(user.id, conn)?;

    let subprocedures = vec![
        freezing_procedure.procedure_model(conn)?,
        freeze_drying_procedure.procedure_model(conn)?,
        falcon_storage_procedure.procedure_model(conn)?,
        fractioning_procedure.procedure_model(conn)?,
        first_ball_mill_procedure.procedure_model(conn)?,
        // TODO!: Add the solvant step
        second_ball_mill_procedure.procedure_model(conn)?,
        centrifuge_procedure.procedure_model(conn)?,
        prelevate_supernatant_procedure.procedure_model(conn)?,
        dispose_of_eppendorf_tube_procedure.procedure_model(conn)?,
        dispose_of_pipette_tips_procedure.procedure_model(conn)?,
        // TODO: potentially dispose of the conical tube if it is empty!
        capping_procedure.procedure_model(conn)?,
        long_term_storage_vial_storage_procedure.procedure_model(conn)?,
        // TODO: store the long term storage vial rack in a -80 freezer
        // TODO: store the non-disposed-of conical tube rack in a room temperature shelf
    ];

    let subprocedures_ref = subprocedures.iter().collect::<Vec<_>>();

    for subprocedure in &subprocedures {
        dbgi_sample_processing_procedure.child(
            subprocedure,
            core_structures::traits::ChildOptions::default().inherit_trackables(),
            user,
            conn,
        )?;
    }

    dbgi_sample_processing_procedure.extend(&subprocedures_ref, user, conn)?;

    Ok(dbgi_sample_processing_procedure)
}
