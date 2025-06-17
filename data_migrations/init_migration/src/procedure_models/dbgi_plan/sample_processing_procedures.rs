//! Submodule defining procedures which are re-used in sample processing.

use core_structures::{
    BallMillMachineModel, BallMillProcedureModel, CappingProcedureModel, CentrifugeModel,
    CentrifugeProcedureModel, ContainerModel, DisposalProcedureModel, FractioningProcedureModel,
    FreezeDryingProcedureModel, FreezingProcedureModel, MixCountableProcedureModel,
    MountTipProcedureModel, ProcedureModel, ProcedureModelTrackable, StorageProcedureModel,
    SupernatantProcedureModel, Trackable, VolumetricContainerModel, WeighingInstrumentModel,
    traits::{AppendProcedureModel, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::trackables::{
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
};

pub(crate) const DBGI_CONICAL_TUBE_BOX: &str = "DBGI Conical Tube Box";
pub(crate) const DBGI_CONICAL_TUBE: &str = "DBGI Conical Tube (Falcon)";
pub(crate) const DBGI_EPPENDORF_TUBE: &str = "DBGI Eppendorf Tube";

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

    let weighing_scale = WeighingInstrumentModel::from_name(WEIGHING_SCALE, conn).unwrap().unwrap();
    let conical_tubes_box = ContainerModel::from_name(POLYSTYRENE_BOX, conn).unwrap().unwrap();

    let conical_tube =
        VolumetricContainerModel::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML, conn).unwrap().unwrap();
    let conical_tube_rack =
        ContainerModel::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML_RACK, conn).unwrap().unwrap();
    let safelock_tube =
        VolumetricContainerModel::from_name(SAFELOCK_TUBE_2ML, conn).unwrap().unwrap();
    let vial = VolumetricContainerModel::from_name(VIAL_1_5ML, conn).unwrap().unwrap();
    let vial_box = ContainerModel::from_name(VIAL_BOX, conn).unwrap().unwrap();
    let ball_mill = BallMillMachineModel::from_name(BALL_MILL_MACHINE, conn).unwrap().unwrap();
    let centrifuge = CentrifugeModel::from_name(SAFELOCK_CENTRIFUGE, conn).unwrap().unwrap();
    let pipette_1000 = Trackable::from_name(PIPETTES_1000, conn).unwrap().unwrap();
    let pipette_tips_1000 = Trackable::from_name(PIPETTE_TIPS_1000, conn).unwrap().unwrap();
    let sealed_cap = Trackable::from_name(VIAL_1_5ML_SEALED_CAP, conn).unwrap().unwrap();

    let procedure_weighing_scale_builder = ProcedureModelTrackable::new()
        .name(WEIGHING_SCALE)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(weighing_scale.id)
        .unwrap();

    let procedure_conical_tubes_box_builder = ProcedureModelTrackable::new()
        .name(DBGI_CONICAL_TUBE_BOX)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(conical_tubes_box.id)
        .unwrap();

    let procedure_conical_tube_builder = ProcedureModelTrackable::new()
        .name(DBGI_CONICAL_TUBE)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(conical_tube.id)
        .unwrap();

    let procedure_conical_tube_rack_builder = ProcedureModelTrackable::new()
        .name("DBGI Conical Tube Rack")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(conical_tube_rack.id)
        .unwrap();

    let procedure_safelock_tube_builder = ProcedureModelTrackable::new()
        .name(DBGI_EPPENDORF_TUBE)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(safelock_tube.id)
        .unwrap();

    let procedure_centrifuge_builder = ProcedureModelTrackable::new()
        .name("DBGI Centrifuge")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(centrifuge.id)
        .unwrap();

    let procedure_ball_mill_builder = ProcedureModelTrackable::new()
        .name("DBGI Ball Mill")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(ball_mill.id)
        .unwrap();

    let pipette_1000_builder = ProcedureModelTrackable::new()
        .name("DBGI Pipette 1000")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(pipette_1000.id)
        .unwrap();

    let pipette_tips_1000_builder = ProcedureModelTrackable::new()
        .name("DBGI Pipette Tips 1000")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(pipette_tips_1000.id)
        .unwrap();

    let sealed_cap_builder = ProcedureModelTrackable::new()
        .name("DBGI Sealed Cap")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(sealed_cap.id)
        .unwrap();

    let long_term_storage_vial_builder = ProcedureModelTrackable::new()
        .name("DBGI Long Term Storage Vial")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(vial.id)
        .unwrap();

    let long_term_storage_vial_box_builder = ProcedureModelTrackable::new()
        .name("DBGI Long Term Storage Vial Box")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(vial_box.id)
        .unwrap();

    let freezing_procedure = FreezingProcedureModel::new()
        .name("DBGI Freezing")
        .unwrap()
        .description("DBGI Freezing procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .source_container(procedure_conical_tubes_box_builder.clone())
        .unwrap()
        .procedure_frozen_with(
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
        .source_container(procedure_conical_tubes_box_builder)
        .unwrap()
        .procedure_freeze_dried_with(
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

    // Next, we store the lyophilized material contained in the conical tube
    // in the conical tube rack.
    let falcon_storage_procedure = StorageProcedureModel::new()
        .name("DBGI Falcon Storage")
        .unwrap()
        .description("DBGI Falcon Storage procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .procedure_parent_container_id(procedure_conical_tube_rack_builder.clone())
        .unwrap()
        .procedure_child_container_id(procedure_conical_tube_builder.clone())
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
        .kilograms(50e-6)
        .unwrap()
        .tolerance_percentage(5.0)
        .unwrap()
        .source(procedure_conical_tube_builder)
        .unwrap()
        .destination(procedure_safelock_tube_builder.clone())
        .unwrap()
        .procedure_weighed_with(procedure_weighing_scale_builder)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let metal_beads = Trackable::from_name(METAL_BEADS_3MM, conn).unwrap().unwrap();
    let procedure_metal_beads_builder = ProcedureModelTrackable::new()
        .name(METAL_BEADS_3MM)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable_id(metal_beads.id)
        .unwrap();

    let metal_beads_procedure = MixCountableProcedureModel::new()
        .name("DBGI Metal Beads Mixing")
        .unwrap()
        .description("DBGI Metal Beads Mixing procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .source(procedure_metal_beads_builder)
        .unwrap()
        .destination(procedure_safelock_tube_builder.clone())
        .unwrap()
        .quantity(3i16)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let first_ball_mill_procedure = BallMillProcedureModel::new()
        .name("DBGI Ball Mill 1")
        .unwrap()
        .description("Ball Mill of lyophilized material procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .procedure_container_id(procedure_safelock_tube_builder.clone())
        .unwrap()
        .procedure_milled_with(procedure_ball_mill_builder.clone())
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let second_ball_mill_procedure = BallMillProcedureModel::new()
        .name("DBGI Ball Mill 2")
        .unwrap()
        .description("Second Ball Mill to extract sample procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .procedure_container_id(procedure_safelock_tube_builder.clone())
        .unwrap()
        .procedure_milled_with(procedure_ball_mill_builder)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let centrifuge_procedure = CentrifugeProcedureModel::new()
        .name("DBGI Centrifuge")
        .unwrap()
        .description("Centrifuge procedure model to separate solid material from supernatant")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .procedure_container_id(procedure_safelock_tube_builder.clone())
        .unwrap()
        .procedure_centrifuged_with(procedure_centrifuge_builder)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let mount_tip_1000_procedure = MountTipProcedureModel::new()
        .name("DBGI Mount Tip")
        .unwrap()
        .description("Mount tip procedure model to mount the tip on the pipette 1000")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .pipette(pipette_1000_builder.clone())
        .unwrap()
        .pipette_tip(pipette_tips_1000_builder.clone())
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let prelevate_supernatant_procedure = SupernatantProcedureModel::new()
        .name("DBGI Supernatant")
        .unwrap()
        .description("Supernatant procedure model to collect supernatant")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(1e-3)
        .unwrap()
        .stratified_source(procedure_safelock_tube_builder.clone())
        .unwrap()
        .supernatant_destination(long_term_storage_vial_builder.clone())
        .unwrap()
        .transferred_with(pipette_1000_builder.clone())
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let capping_procedure = CappingProcedureModel::new()
        .name("DBGI Capping")
        .unwrap()
        .description("Capping procedure model to cap the long term storage vial")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .procedure_container_id(long_term_storage_vial_builder.clone())
        .unwrap()
        .procedure_capped_with(sealed_cap_builder)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // We store the long term storage vial in a box.
    let long_term_storage_vial_storage_procedure = StorageProcedureModel::new()
        .name("DBGI Long Term Storage Vial Storage")
        .unwrap()
        .description("DBGI Long Term Storage Vial Storage procedure model")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .procedure_parent_container_id(long_term_storage_vial_box_builder.clone())
        .unwrap()
        .procedure_child_container_id(long_term_storage_vial_builder)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let dispose_of_eppendorf_tube_procedure = DisposalProcedureModel::new()
        .name("DBGI Dispose of Eppendorf Tube")
        .unwrap()
        .description("Disposal procedure model for Eppendorf tubes. You can separate the metal beads for further reuse at this step.")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .disposed_id(procedure_safelock_tube_builder)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let dispose_of_pipette_tips_procedure = DisposalProcedureModel::new()
        .name("DBGI Dispose of Pipette Tips")
        .unwrap()
        .description("Disposal procedure model for used pipette tips.")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .disposed_id(pipette_tips_1000_builder)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let subprocedures = vec![
        freezing_procedure.procedure_model(conn).unwrap(),
        freeze_drying_procedure.procedure_model(conn).unwrap(),
        falcon_storage_procedure.procedure_model(conn).unwrap(),
        fractioning_procedure.procedure_model(conn).unwrap(),
        metal_beads_procedure.procedure_model(conn).unwrap(),
        first_ball_mill_procedure.procedure_model(conn).unwrap(),
        // TODO!: Add the solvant step
        second_ball_mill_procedure.procedure_model(conn).unwrap(),
        centrifuge_procedure.procedure_model(conn).unwrap(),
        mount_tip_1000_procedure.procedure_model(conn).unwrap(),
        prelevate_supernatant_procedure.procedure_model(conn).unwrap(),
        dispose_of_eppendorf_tube_procedure.procedure_model(conn).unwrap(),
        dispose_of_pipette_tips_procedure.procedure_model(conn).unwrap(),
        // TODO: potentially dispose of the conical tube if it is empty!
        capping_procedure.procedure_model(conn).unwrap(),
        long_term_storage_vial_storage_procedure.procedure_model(conn).unwrap(),
        // TODO: store the long term storage vial rack in a -80 freezer
        // TODO: store the non-disposed-of conical tube rack in a room temperature shelf
    ];

    let subprocedures_ref = subprocedures.iter().collect::<Vec<_>>();

    for subprocedure in &subprocedures {
        dbgi_sample_processing_procedure
            .child(
                subprocedure,
                core_structures::traits::ChildOptions::default().inherit_trackables(),
                user,
                conn,
            )
            .unwrap();
    }

    dbgi_sample_processing_procedure.extend(&subprocedures_ref, user, conn).unwrap();

    dbgi_sample_processing_procedure
}
