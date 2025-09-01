//! Submodule defining procedures which are re-used in sample processing.

use core_structures::{
    BallMillProcedureTemplate, CappingProcedureTemplate, CentrifugeProcedureTemplate,
    DisposalProcedureTemplate, FractioningProcedureTemplate, FreezeDryingProcedureTemplate,
    FreezingProcedureTemplate, ProcedureTemplate, ProcedureTemplateAssetModel,
    StorageProcedureTemplate, SupernatantProcedureTemplate,
    traits::{AppendProcedureTemplate, ParentProcedureTemplate},
};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    procedure_template_trackables::{
        ball_mill::safelock_ball_mill_builder,
        centrifuge::safelock_centrifuge_builder,
        conical_tubes::cct_builder,
        conical_tubes_box::{cct_box_builder, cct_rack_builder},
        freeze_dryer::freeze_dryer_builder,
        freezer::freezer_builder,
        pipette_tips::pipette_tips_1000ul_builder,
        pipettes::pipette_1000ul_builder,
        safelock::safelock_builder,
        vial_caps::sealed_cap_vial_1_5ml_builder,
        weighing_device::weighing_device_builder,
    },
    trackables::{
        containers::{boxes::vial_rack_1_5ml, vials::vial_1_5ml},
        instruments::freezer::freezer,
    },
};

pub(super) fn init_dbgi_sample_processing_procedures(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    let dbgi_sample_processing_procedure = ProcedureTemplate::new()
        .name("Sample Processing")?
        .description("Sample Processing procedure template")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let long_term_storage_vial_builder = ProcedureTemplateAssetModel::new()
        .name("Long Term Storage Vial")?
        .created_by(user.id)?
        .trackable(vial_1_5ml(user, conn)?.id)?;

    let long_term_storage_vial_box_builder = ProcedureTemplateAssetModel::new()
        .name("Long Term Storage Vial Box")?
        .created_by(user.id)?
        .trackable(vial_rack_1_5ml(user, conn)?.id)?;

    let storage_freezer = ProcedureTemplateAssetModel::new()
        .name("Sample Storage Freezer")?
        .created_by(user.id)?
        .trackable(freezer(user, conn)?.id)?;

    let freezing_procedure = FreezingProcedureTemplate::new()
        .name("Freezing")?
        .description("Freezing procedure template")?
        .created_by(user.id)?
        .procedure_frozen_container(cct_box_builder(user, conn)?)?
        .procedure_frozen_with(freezer_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let freeze_drying_procedure = FreezeDryingProcedureTemplate::new()
        .name("Freeze Drying")?
        .description("Freeze Drying procedure template")?
        .created_by(user.id)?
        .procedure_freeze_dried_container(cct_builder(user, conn)?)?
        .procedure_freeze_dried_with(freeze_dryer_builder(user, conn)?)?
        .insert(user.id, conn)?;

    // Next, we store the lyophilized material contained in the conical tube
    // in the conical tube rack.
    let falcon_storage_procedure = StorageProcedureTemplate::new()
        .name("Falcon Storage")?
        .description("Falcon Storage procedure template")?
        .created_by(user.id)?
        .procedure_parent_container(cct_rack_builder(user, conn)?)?
        .procedure_child_container(cct_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let fractioning_procedure = FractioningProcedureTemplate::new()
        .name("Fractioning")?
        .description("Fractioning procedure template")?
        .created_by(user.id)?
        .kilograms(50e-6)?
        .tolerance_percentage(5.0)?
        .procedure_fragment_source(cct_builder(user, conn)?)?
        .procedure_fragment_placed_into(safelock_builder(user, conn)?)?
        .procedure_weighed_with(weighing_device_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let first_ball_mill_procedure = BallMillProcedureTemplate::new()
        .name("Ball Mill 1")?
        .description("Ball Mill of lyophilized material procedure template")?
        .created_by(user.id)?
        .procedure_milled_container(safelock_builder(user, conn)?)?
        .procedure_milled_with(safelock_ball_mill_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let second_ball_mill_procedure = BallMillProcedureTemplate::new()
        .name("Ball Mill 2")?
        .description("Second Ball Mill to extract sample procedure template")?
        .created_by(user.id)?
        .procedure_milled_container(safelock_builder(user, conn)?)?
        .procedure_milled_with(safelock_ball_mill_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let centrifuge_procedure = CentrifugeProcedureTemplate::new()
        .name("Centrifuge")?
        .description("Centrifuge procedure template to separate solid material from supernatant")?
        .created_by(user.id)?
        .procedure_centrifuged_container(safelock_builder(user, conn)?)?
        .procedure_centrifuged_with(safelock_centrifuge_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let prelevate_supernatant_procedure = SupernatantProcedureTemplate::new()
        .name("Supernatant")?
        .description("Supernatant procedure template to collect supernatant")?
        .created_by(user.id)?
        .liters(1e-3)?
        .procedure_stratified_source(safelock_builder(user, conn)?)?
        .procedure_supernatant_destination(long_term_storage_vial_builder.clone())?
        .procedure_transferred_with(pipette_1000ul_builder(user, conn)?)?
        .procedure_pipette_tip(pipette_tips_1000ul_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let capping_procedure = CappingProcedureTemplate::new()
        .name("Capping")?
        .description("Capping procedure template to cap the long term storage vial")?
        .created_by(user.id)?
        .procedure_container(long_term_storage_vial_builder.clone())?
        .procedure_capped_with(sealed_cap_vial_1_5ml_builder(user, conn)?)?
        .insert(user.id, conn)?;

    // We store the long term storage vial in a box.
    let long_term_storage_vial_storage_procedure = StorageProcedureTemplate::new()
        .name("Long Term Storage Vial Storage")?
        .description("Long Term Storage Vial Storage procedure template")?
        .created_by(user.id)?
        .procedure_parent_container(long_term_storage_vial_box_builder.clone())?
        .procedure_child_container(long_term_storage_vial_builder)?
        .insert(user.id, conn)?;

    let dispose_of_eppendorf_tube_procedure = DisposalProcedureTemplate::new()
        .name("Dispose of Eppendorf Tube")?
        .description("Disposal procedure template for Eppendorf tubes. You can separate the metal beads for further reuse at this step.")?
        .created_by(user.id)?
        .disposed(safelock_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let dispose_of_pipette_tips_procedure = DisposalProcedureTemplate::new()
        .name("Dispose of Pipette Tips")?
        .description("Disposal procedure template for used pipette tips.")?
        .created_by(user.id)?
        .disposed(pipette_tips_1000ul_builder(user, conn)?)?
        .insert(user.id, conn)?;

    let subprocedures = vec![
        freezing_procedure.procedure_template(conn)?,
        freeze_drying_procedure.procedure_template(conn)?,
        falcon_storage_procedure.procedure_template(conn)?,
        fractioning_procedure.procedure_template(conn)?,
        first_ball_mill_procedure.procedure_template(conn)?,
        // TODO!: Add the solvant step
        second_ball_mill_procedure.procedure_template(conn)?,
        centrifuge_procedure.procedure_template(conn)?,
        prelevate_supernatant_procedure.procedure_template(conn)?,
        dispose_of_eppendorf_tube_procedure.procedure_template(conn)?,
        dispose_of_pipette_tips_procedure.procedure_template(conn)?,
        // TODO: potentially dispose of the conical tube if it is empty!
        capping_procedure.procedure_template(conn)?,
        long_term_storage_vial_storage_procedure.procedure_template(conn)?,
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
